use chrono::{DateTime, Utc};

use crate::{
    directory_reader::DirectoryReader, Error, GetImagesResult, GetLibraryContentRequest, ImageInfo,
    MediaLibrarySource,
};

use std::{path::Path, time::SystemTime};
use walkdir::WalkDir;

pub struct WalkdirReader<'a> {
    path: &'a Path,
}

impl<'a> WalkdirReader<'a> {
    pub fn new(path: &'a Path) -> Self {
        WalkdirReader { path }
    }
}

fn is_image_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => {
            let ext = ext.to_lowercase();
            matches!(
                ext.as_str(),
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff"
            )
        }
        None => false,
    }
}

fn get_mime_type(path: &Path) -> String {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => {
            let ext = ext.to_lowercase();
            match ext.as_str() {
                "jpg" | "jpeg" => "image/jpeg".to_owned(),
                "png" => "image/png".to_owned(),
                "gif" => "image/gif".to_owned(),
                "bmp" => "image/bmp".to_owned(),
                "webp" => "image/webp".to_owned(),
                "tiff" => "image/tiff".to_owned(),
                _ => "application/octet-stream".to_owned(),
            }
        }
        None => "application/octet-stream".to_owned(),
    }
}

impl<'a> DirectoryReader for WalkdirReader<'a> {
    fn read_directory(&self, request: &GetLibraryContentRequest) -> Result<GetImagesResult, Error> {
        match &request.source {
            #[cfg(not(target_os = "android"))]
            MediaLibrarySource::PictureDir => {
                use std::path::PathBuf;
                let mut items = Vec::new();

                let mut all_entries: Vec<(PathBuf, Option<SystemTime>, Option<SystemTime>)> =
                    WalkDir::new(self.path)
                        .into_iter()
                        .filter_map(Result::ok)
                        .filter(|e| e.file_type().is_file())
                        .filter(|e| is_image_file(e.path()))
                        .filter_map(|entry| {
                            let path = entry.path().to_path_buf();

                            match std::fs::metadata(&path) {
                                Ok(meta) => {
                                    let modified = meta.modified().ok();
                                    let created = meta.created().ok();
                                    Some((path, modified, created))
                                }
                                Err(_) => None, // Ignoriere Dateien ohne Metadaten
                            }
                        })
                        .collect();

                all_entries.sort_by(|a, b| sort_entries(a, b, request));

                let skip = request.offset;
                let limit = request.limit;

                let page = if skip >= all_entries.len() {
                    &[]
                } else {
                    &all_entries[skip..(skip + limit).min(all_entries.len())]
                };
                for (path, modified, created) in page {
                    items.push(ImageInfo {
                        path: path.to_string_lossy().to_string(),
                        content_uri: format!("file://{}", path.to_string_lossy()),
                        mime_type: get_mime_type(path),
                        meta_data: get_meta_data(modified, created),
                    });
                }

                Ok(GetImagesResult { items })
            }
            #[cfg(target_os = "android")]
            _ => Ok(GetImagesResult { items: Vec::new() }),
        }
    }
}

fn sort_entries(
    a: &(std::path::PathBuf, Option<SystemTime>, Option<SystemTime>),
    b: &(std::path::PathBuf, Option<SystemTime>, Option<SystemTime>),
    request: &GetLibraryContentRequest,
) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let (_, a_modified, a_created) = a;
    let (_, b_modified, b_created) = b;

    let ordering = match &request.sort_column {
        Some(sort_column) => match sort_column {
            crate::SortColumn::DateModified => a_modified.cmp(b_modified),
            crate::SortColumn::DateAdded => a_created.cmp(b_created),
        },
        None => Ordering::Equal,
    };

    match &request.sort_direction {
        Some(sort_direction) => match sort_direction {
            crate::SortDirection::Ascending => ordering,
            crate::SortDirection::Descending => ordering.reverse(),
        },
        None => Ordering::Equal,
    }
}

fn system_time_to_iso8601(time: &SystemTime) -> String {
    let datetime: DateTime<Utc> = (*time).into();
    datetime.to_rfc3339()
}

fn get_meta_data(
    modified: &Option<SystemTime>,
    created: &Option<SystemTime>,
) -> Option<std::collections::HashMap<crate::MetaDataField, String>> {
    let mut meta_data = std::collections::HashMap::new();

    if let Some(modified) = modified {
        meta_data.insert(
            crate::MetaDataField::DateModified,
            system_time_to_iso8601(modified),
        );
    }

    if let Some(created) = created {
        meta_data.insert(
            crate::MetaDataField::DateAdded,
            system_time_to_iso8601(created),
        );
    }

    Some(meta_data)
}
