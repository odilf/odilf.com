pub mod fetch;

use serde::{Deserialize, Serialize};

/// A photo from an Immich album
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    /// The relative path to the local image file
    pub image_path: String,
    /// The caption/description of the image
    pub caption: String,
    /// The original filename
    pub filename: String,
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
