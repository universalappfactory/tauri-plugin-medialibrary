use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tauri::plugin::PermissionState;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MediaLibrarySource {
    #[cfg(not(target_os = "android"))]
    PictureDir,
    #[cfg(target_os = "android")]
    ExternalStorage,
    #[cfg(target_os = "android")]
    VolumeExternalPrimary,
}

impl Display for MediaLibrarySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(not(target_os = "android"))]
            MediaLibrarySource::PictureDir => write!(f, "PictureDir"),
            #[cfg(target_os = "android")]
            MediaLibrarySource::ExternalStorage => write!(f, "ExternalStorage"),
            #[cfg(target_os = "android")]
            MediaLibrarySource::VolumeExternalPrimary => write!(f, "VolumeExternalPrimary"),
        }
    }
}

impl Default for MediaLibrarySource {
    fn default() -> Self {
        #[cfg(target_os = "android")]
        return MediaLibrarySource::ExternalStorage;

        #[cfg(not(target_os = "android"))]
        return MediaLibrarySource::PictureDir;
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLibraryContentRequest {
    pub limit: usize,
    pub offset: usize,
    pub source: MediaLibrarySource,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPermissionsArgs {
    pub source: MediaLibrarySource,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageInfo {
    pub path: String,
    pub content_uri: String,
    pub mime_type: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetImagesResult {
    pub items: Vec<ImageInfo>,
}

#[derive(Deserialize, Default, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResponse {
    pub post_notification: PermissionState,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetThumbnailResponse {
    pub content: String,
}

impl PermissionResponse {
    pub fn granted() -> Self {
        Self {
            post_notification: PermissionState::Granted,
        }
    }
}
