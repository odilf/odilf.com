pub mod fetch;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// A photo from an Immich album
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    /// Unique identifier (slug) for the photo
    pub id: String,
    /// The caption/description of the image
    pub caption: String,
    /// The original filename
    pub filename: String,
}

impl Photo {
    pub fn path(&self) -> String {
        format!("/static/pics/{}.webp", self.id)
    }

    pub fn thumb_path(&self) -> String {
        format!("/static/pics/{}_thumb.webp", self.id)
    }

    pub fn fs_path(&self, output_dir: impl AsRef<Path>) -> PathBuf {
        output_dir.as_ref().join(&self.path()[1..])
    }

    pub fn fs_thumb_path(&self, output_dir: impl AsRef<Path>) -> PathBuf {
        output_dir.as_ref().join(&self.thumb_path()[1..])
    }
}

/// Response from Immich album API
#[derive(Debug, Deserialize)]
pub struct AlbumResponse {
    #[serde(default)]
    pub assets: Vec<AssetResponse>,
}

/// Asset in an album
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponse {
    pub id: String,
    #[serde(default)]
    pub original_file_name: String,
    #[serde(default)]
    pub exif_info: Option<ExifInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ExifInfo {
    #[serde(default)]
    pub description: Option<String>,
}
