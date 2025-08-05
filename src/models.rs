use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, fs, path::Path};
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub enum SortColumn {
    #[default]
    DateAdded,
    DateModified,
    #[cfg(target_os = "android")]
    DateTaken,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
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
    pub sort_column: Option<SortColumn>,
    pub sort_direction: Option<SortDirection>,
    pub include_file_metadata: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetImageRequest {
    pub uri: String,
}

impl From<String> for GetImageRequest {
    fn from(uri: String) -> Self {
        GetImageRequest { uri }
    }
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
    pub meta_data: Option<HashMap<String, String>>,
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

impl GetImagesResult {
    pub fn with_file_metadata(self, include_file_metadata: bool) -> Self {
        if include_file_metadata {
            let items = self
                .items
                .into_iter()
                .map(|item| item.with_file_metadata())
                .collect();

            return Self { items };
        }
        self
    }
}

impl ImageInfo {
    pub fn with_file_metadata(self) -> Self {
        if let Ok(metadata) = fs::metadata(&self.path) {
            let mut meta_data = self.meta_data.unwrap_or_default();
            meta_data.insert("file_file_size".to_string(), metadata.len().to_string());

            if let Ok(created) = metadata.created() {
                if let Ok(duration) = created.duration_since(std::time::UNIX_EPOCH) {
                    meta_data.insert("file_created".to_string(), duration.as_secs().to_string());
                }
            }

            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                    meta_data.insert("file_modified".to_string(), duration.as_secs().to_string());
                }
            }

            meta_data.insert(
                "file_readonly".to_string(),
                metadata.permissions().readonly().to_string(),
            );

            if let Some(file_name) = Path::new(&self.path).file_name() {
                if let Some(name_str) = file_name.to_str() {
                    meta_data.insert("file_file_name".to_string(), name_str.to_string());
                }
            }

            if let Some(extension) = Path::new(&self.path).extension() {
                if let Some(ext_str) = extension.to_str() {
                    meta_data.insert("file_file_extension".to_string(), ext_str.to_string());
                }
            }

            return Self {
                meta_data: Some(meta_data),
                ..self
            };
        }

        self
    }
}
