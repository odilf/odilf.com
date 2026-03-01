//! Fetching from immich for pics.
//!
//! There is a "cache" file that holds all photo metadata. This is fetched if
//! - the `.immich_cache` file is missing or
//! - if some pic that is listed in the cache file is missing
//! - or if it compiled in release mode.

// NOTE: This file is badly coded. There are a thousand invisible invariants
// not properly upheld. It just does not seem worth to improve.

use color_eyre::eyre::{self, Context as _, ContextCompat as _};
use image::ImageReader;
use reqwest::header::USER_AGENT;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::pics::immich::AssetResponse;

use super::{AlbumResponse, Photo};

/// Get the cache file path for album metadata
fn get_cache_file(album_id: &str) -> eyre::Result<PathBuf> {
    let cache_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".immich_cache");
    fs::create_dir_all(&cache_dir).wrap_err("Failed to create Immich cache directory")?;
    Ok(cache_dir.join(format!("{}.json", album_id)))
}

/// Load cached gallery metadata if it exists and is valid
fn load_from_cache(album_id: &str) -> eyre::Result<Option<Vec<Photo>>> {
    let cache_file = get_cache_file(album_id)?;

    if !cache_file.exists() {
        tracing::info!("No cache file found");
        return Ok(None);
    }

    let cached_data = fs::read_to_string(&cache_file)
        .wrap_err_with(|| format!("Failed to read cache file at {:?}", cache_file))?;

    let photos: Vec<Photo> =
        serde_json::from_str(&cached_data).wrap_err("Failed to parse cached photos")?;

    tracing::info!("Loaded {} photo metadata from cache", photos.len());
    Ok(Some(photos))
}

/// Fetch photos from an Immich album, downloading and converting images
pub fn fetch_immich_album(
    immich_url: &str,
    album_id: &str,
    api_key: &str,
    output_dir: &Path,
) -> eyre::Result<Vec<Photo>> {
    tracing::info!("Fetching Immich album: {}", album_id);

    let images_dir = output_dir.join("static/pics");
    fs::create_dir_all(&images_dir).wrap_err("Failed to create images directory")?;

    if let Some(cached_photos) = load_from_cache(album_id)?
        && !cfg!(debug_assertions)
    {
        tracing::info!("Found cached photo metadata, verifying image files...");

        let all_files_exist = cached_photos.iter().all(|photo| {
            let full_path = images_dir.join(&photo.id);
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

    let api_url = format!("{immich_url}/api/albums/{album_id}");
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

    let photos = album
        .assets
        .into_iter()
        .map(|asset| get_immich_pic(asset, output_dir, immich_url, api_key))
        .collect::<eyre::Result<Vec<_>>>()?;

    let cache_file = get_cache_file(album_id)?;
    fs::write(
        &cache_file,
        serde_json::to_string_pretty(&photos).wrap_err("Failed to serialize photo metadata")?,
    )
    .wrap_err_with(|| format!("Failed to write cache file at {:?}", cache_file))?;

    tracing::info!("Saved {} photo metadata to cache", photos.len());

    Ok(photos)
}

/// Gets the immich picture, downloading it and creating a thumbnail if it doesn't exist.
fn get_immich_pic(
    asset: AssetResponse,
    output_dir: &Path,
    immich_url: &str,
    api_key: &str,
) -> eyre::Result<Photo> {
    let caption = asset
        .exif_info
        .and_then(|exif| exif.description)
        .unwrap_or_default();

    let photo = Photo {
        id: asset.id,
        caption,
        filename: asset.original_file_name,
    };
    let output_path = photo.fs_path(output_dir);
    fs::create_dir_all(output_path.parent().expect("/static at least"))?;

    fetch_and_convert_pic(&photo, &output_path, immich_url, api_key)?;
    create_thumbnail(&photo, output_dir)?;
    tracing::info!("Saved WebP to: {}", output_path.to_string_lossy());

    Ok(photo)
}

fn fetch_and_convert_pic(
    photo: &Photo,
    output_dir: &Path,
    immich_url: &str,
    api_key: &str,
) -> eyre::Result<()> {
    if photo.fs_path(output_dir).exists() {
        tracing::debug!(?photo.id, "Image already exists");
        return Ok(());
    }

    tracing::debug!(?photo.id, "Downloading image");
    let download_url = format!("{immich_url}/api/assets/{}/original?edited=true", &photo.id);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&download_url)
        .header(USER_AGENT, "rust-web-client")
        .header("x-api-key", api_key)
        .send()
        .wrap_err_with(|| format!("Failed to download image {}", photo.id))?;

    let status = response.status();
    if !status.is_success() {
        eyre::bail!(
            "Failed to download image {}: {} {}",
            photo.id,
            status.as_u16(),
            status.canonical_reason().unwrap_or("unknown error")
        );
    }

    let image_data = response
        .bytes()
        .wrap_err_with(|| format!("Failed to read image bytes for {}", photo.id))?;
    tracing::info!("Using ImageMagick to convert image: {}", photo.caption);

    let mut cmd = Command::new("magick")
        .arg("-")
        // Strip all metadata and profiles
        .arg("-auto-orient")
        .arg("-strip")
        .arg("-quality")
        .arg("85")
        .arg(format!(
            "webp:{}",
            output_dir.to_str().expect("Valid unicode")
        ))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .wrap_err("Failed to spawn ImageMagick convert command")?;

    let mut stdin = cmd
        .stdin
        .take()
        .wrap_err("Couldn't take imagemgick stdin")?;
    stdin.write_all(&image_data)?;
    drop(stdin);

    let output = cmd.wait_with_output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eyre::bail!("ImageMagick conversion failed for {}: {}", photo.id, stderr);
    }

    tracing::debug!(
        "ImageMagick successfully converted image: {}",
        photo.caption
    );

    Ok(())
}

/// Create a thumbnail of the picture at [`Photo::fs_path`] if it doesn't exist.
///
/// Errors if it doesn't exist.
fn create_thumbnail(photo: &Photo, output_dir: &Path) -> eyre::Result<()> {
    if photo.fs_thumb_path(output_dir).exists() {
        return Ok(());
    }

    let img = ImageReader::open(photo.fs_path(output_dir))?.decode()?;
    let thumb = img.thumbnail(400, 400);

    thumb
        .save_with_format(photo.fs_thumb_path(output_dir), image::ImageFormat::WebP)
        .map_err(|e| eyre::eyre!("Failed to save thumbnail: {}", e))?;

    tracing::debug!(
        thumb_path=?photo.fs_thumb_path(output_dir),
        "Created thumbnail"
    );

    Ok(())
}
