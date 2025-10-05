use color_eyre::eyre::{self, ContextCompat as _};
use reqwest::header::USER_AGENT;
use std::{fs, path::PathBuf};
use url::Url;

pub fn get_image<'a>(urls: impl Iterator<Item = &'a Url>, slug: &str) -> eyre::Result<String> {
    let cache_file_path = PathBuf::from(format!("target/debug/odilf-site/{}", slug));
    let image_url = fs::read_to_string(&cache_file_path).or_else(|_| {
        for url in urls {
            tracing::info!(?url);

            let image_url = match url.host_str() {
                Some(host) if host.contains("wikipedia.org") => extract_wikipedia_image(url),
                _ => continue,
            }?;

            fs::create_dir_all(
                cache_file_path
                    .parent()
                    .expect("File path declared should have parent."),
            )?;
            fs::write(cache_file_path, &image_url)?;
            return Ok(image_url.to_string());
        }

        eyre::bail!("No image found")
    })?;

    Ok(image_url)
}

fn extract_wikipedia_image(url: &Url) -> eyre::Result<String> {
    let title = url
        .path_segments()
        .wrap_err("Wikipedia link has no path segments")?
        .next_back()
        .expect("path_segments has at least one string when non-empty");

    // First try to get the main image
    let api_url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&titles={}&prop=pageprops&format=json",
        // TODO: Maybe we should URL encode
        title
    );

    let client = reqwest::blocking::Client::new();
    let response: serde_json::Value = client
        .get(api_url)
        .header(USER_AGENT, "rust-web-api-client")
        .send()?
        .json()?;

    if let Some(pages) = response["query"]["pages"].as_object() {
        for (_, page) in pages {
            if let Some(page_image) = page["pageprops"]["page_image"].as_str() {
                // We have the filename, now get the actual URL
                let image_title = format!("File:{}", page_image);
                let Some(image_url) = get_wikipedia_image_url(&image_title, &client)? else {
                    continue;
                };

                return Ok(image_url);
            }
        }
    }

    // Then fallback to "thumbnail" image
    let api_url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&titles={}&prop=pageimages&format=json&piprop=original",
        title
    );

    let client = reqwest::blocking::Client::new();
    let response: serde_json::Value = client
        .get(api_url)
        .header(USER_AGENT, "rust-web-api-client")
        .send()?
        .json()?;

    if let Some(pages) = response["query"]["pages"].as_object() {
        for (_, page) in pages {
            if let Some(original) = page["original"]["source"].as_str() {
                return Ok(original.to_string());
            }
        }
    }

    eyre::bail!("Couldn't get wikipedia image")
}

fn get_wikipedia_image_url(
    image_title: &str,
    client: &reqwest::blocking::Client,
) -> eyre::Result<Option<String>> {
    let api_url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&titles={}&prop=imageinfo&iiprop=url&format=json",
        image_title
    );

    let response: serde_json::Value = client
        .get(api_url)
        .header(USER_AGENT, "rust-web-api-client")
        .send()?
        .json()?;

    if let Some(pages) = response["query"]["pages"].as_object() {
        for (_, page) in pages {
            if let Some(imageinfo) = page["imageinfo"].as_array()
                && let Some(first) = imageinfo.first()
                && let Some(url) = first["url"].as_str()
            {
                return Ok(Some(url.to_string()));
            }
        }
    }

    Ok(None)
}
