use color_eyre::eyre::{self, Context as _};
use reqwest::header::USER_AGENT;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::{AlbumResponse, Photo};

/// Get the images output directory
fn get_images_dir(output: &Path) -> eyre::Result<PathBuf> {
    let images_dir = output.join("static/pics");
    fs::create_dir_all(&images_dir).wrap_err("Failed to create images directory")?;
    Ok(images_dir)
}

/// Get the cache directory for Immich metadata
fn get_cache_dir() -> eyre::Result<PathBuf> {
    let cache_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".immich_cache");
    fs::create_dir_all(&cache_dir).wrap_err("Failed to create Immich cache directory")?;
    Ok(cache_dir)
}

/// Get the cache file path for album metadata
fn get_cache_file(album_id: &str) -> eyre::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    Ok(cache_dir.join(format!("{}.json", album_id)))
}

/// Load cached gallery metadata if it exists and is valid
fn load_from_cache(album_id: &str) -> eyre::Result<Option<Vec<Photo>>> {
    let cache_file = get_cache_file(album_id)?;

    if !cache_file.exists() {
        tracing::debug!("No cache file found");
        return Ok(None);
    }

    let cached_data = fs::read_to_string(&cache_file)
        .wrap_err_with(|| format!("Failed to read cache file at {:?}", cache_file))?;

    let photos: Vec<Photo> =
        serde_json::from_str(&cached_data).wrap_err("Failed to parse cached photos")?;

    tracing::info!("Loaded {} photo metadata from cache", photos.len());
    Ok(Some(photos))
}

/// Save gallery metadata to cache
fn save_to_cache(album_id: &str, photos: &[Photo]) -> eyre::Result<()> {
    let cache_file = get_cache_file(album_id)?;

    let json =
        serde_json::to_string_pretty(photos).wrap_err("Failed to serialize photo metadata")?;

    fs::write(&cache_file, json)
        .wrap_err_with(|| format!("Failed to write cache file at {:?}", cache_file))?;

    tracing::info!("Saved {} photo metadata to cache", photos.len());
    Ok(())
}

/// Convert image using ImageMagick with metadata stripping
/// This is used as a fallback for formats not supported by the image crate (namely, HEIC)
fn convert_with_imagemagick(
    input_path: &Path,
    output_path: &Path,
    asset_id: &str,
) -> eyre::Result<()> {
    tracing::info!("Using ImageMagick to convert image: {}", asset_id);

    let output = Command::new("magick")
        .arg("convert")
        // Explicitly specify HEIC format for input in case it's not detected
        .arg(format!("heic:{}", input_path.to_string_lossy()))
        // Strip all metadata and profiles
        .arg("-strip")
        .arg("-quality")
        .arg("85")
        .arg(format!("webp:{}", output_path.to_string_lossy()))
        .output()
        .wrap_err("Failed to execute ImageMagick convert command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(eyre::eyre!(
            "ImageMagick conversion failed for {}: {}",
            asset_id,
            stderr
        ));
    }

    tracing::info!("ImageMagick successfully converted image: {}", asset_id);
    Ok(())
}

/// Download, convert and save a single image to WebP format
fn download_and_convert_image(
    immich_url: &str,
    asset_id: &str,
    api_key: &str,
    output_dir: &Path,
    filename: &str,
) -> eyre::Result<String> {
    // Generate a safe filename for the output
    let filename_stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image");

    let output_filename = format!("{}.webp", filename_stem);
    let output_path = output_dir.join(&output_filename);

    // Skip if already downloaded
    if output_path.exists() {
        tracing::debug!("Image already exists: {}", output_filename);
        return Ok(format!("/static/pics/{}", output_filename));
    }

    // Download the image
    tracing::info!("Downloading image: {} -> {}", asset_id, output_filename);

    let download_url = format!("{}/api/assets/{}/original", immich_url, asset_id);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&download_url)
        .header(USER_AGENT, "rust-web-api-client")
        .header("x-api-key", api_key)
        .send()
        .wrap_err_with(|| format!("Failed to download image {}", asset_id))?;

    let status = response.status();
    if !status.is_success() {
        return Err(eyre::eyre!(
            "Failed to download image {}: {} {}",
            asset_id,
            status.as_u16(),
            status.canonical_reason().unwrap_or("unknown error")
        ));
    }

    let image_data = response
        .bytes()
        .wrap_err_with(|| format!("Failed to read image bytes for {}", asset_id))?;

    tracing::info!("Converting image to WebP: {}", output_filename);

    // Try the Rust image crate first (fast path for common formats)
    let conversion_result = (|| -> eyre::Result<()> {
        let img = image::load_from_memory(&image_data)
            .map_err(|e| eyre::eyre!("Rust image crate failed to decode: {}", e))?;

        // Strip all metadata by converting to RGB/RGBA and creating a new image
        // This ensures no EXIF data, color profiles, or other metadata is preserved
        let stripped_img = match img {
            image::DynamicImage::ImageRgba8(rgba) => image::DynamicImage::ImageRgba8(rgba),
            image::DynamicImage::ImageRgb8(rgb) => image::DynamicImage::ImageRgb8(rgb),
            other => other.to_rgb8().into(),
        };

        stripped_img
            .save_with_format(&output_path, image::ImageFormat::WebP)
            .map_err(|e| eyre::eyre!("Failed to save WebP: {}", e))?;

        Ok(())
    })();

    // ImageMagick fallback
    if let Err(error) = conversion_result {
        tracing::warn!(
            "Rust image crate failed for {}, falling back to ImageMagick: {}",
            asset_id,
            error
        );

        let temp_file = output_dir.join(format!(".tmp_{}", asset_id));
        fs::write(&temp_file, &image_data)
            .wrap_err_with(|| format!("Failed to write temporary file for {}", asset_id))?;

        convert_with_imagemagick(&temp_file, &output_path, asset_id)?;
        fs::remove_file(&temp_file).ok();

        tracing::info!(
            "ImageMagick successfully converted and saved image: {}",
            asset_id
        );
    }

    tracing::info!("Saved WebP to: {}", output_path.to_string_lossy());

    Ok(format!("/static/pics/{}", output_filename))
}

/// Fetch photos from an Immich album, downloading and converting images
pub fn fetch_immich_album(
    immich_url: &str,
    album_id: &str,
    api_key: &str,
    output_dir: &Path,
) -> eyre::Result<Vec<Photo>> {
    tracing::info!("Fetching Immich album: {}", album_id);

    let images_dir = get_images_dir(output_dir)?;

    if let Some(cached_photos) = load_from_cache(album_id)? {
        tracing::info!("Found cached photo metadata, verifying image files...");

        let all_files_exist = cached_photos.iter().all(|photo| {
            let full_path = output_dir.join(&photo.image_path);
            full_path.exists()
        });

        if all_files_exist {
            tracing::info!(
                "All {} images verified in cache, skipping downloads",
                cached_photos.len()
            );
            return Ok(cached_photos);
        } else {
            tracing::warn!("Some cached images are missing, re-fetching from Immich");
        }
    }

    tracing::info!("Cache miss or incomplete, fetching from Immich API");

    let api_url = format!("{}/api/albums/{}", immich_url, album_id);
    tracing::debug!("API URL: {}", api_url);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&api_url)
        .header(USER_AGENT, "rust-web-api-client")
        .header("x-api-key", api_key)
        .send()
        .wrap_err("Failed to fetch Immich album")?;

    let status = response.status();
    if !status.is_success() {
        return Err(eyre::eyre!(
            "Immich API returned status {}: {} - check that the album ID and API key are correct",
            status.as_u16(),
            status.canonical_reason().unwrap_or("unknown error")
        ));
    }

    let album: AlbumResponse = response
        .json()
        .wrap_err("Failed to parse Immich API response as JSON")?;

    tracing::info!("Fetched {} assets from Immich", album.assets.len());

    let mut photos = Vec::new();
    for asset in album.assets {
        let caption = asset
            .exif_info
            .and_then(|exif| exif.description)
            .unwrap_or_default();

        let image_path = download_and_convert_image(
            immich_url,
            &asset.id,
            api_key,
            &images_dir,
            &asset.original_file_name,
        )?;

        photos.push(Photo {
            image_path,
            caption,
            filename: asset.original_file_name,
        });
    }

    save_to_cache(album_id, &photos)?;

    Ok(photos)
}
