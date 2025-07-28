use crate::{
    directory_reader::DirectoryReader, Error, GetImagesResult, GetLibraryContentRequest, ImageInfo,
    MediaLibrarySource,
};
use std::fs;
use xdg_user;

pub struct XdgDirectoryReader;

impl DirectoryReader for XdgDirectoryReader {
    fn read_directory(request: &GetLibraryContentRequest) -> Result<GetImagesResult, Error> {
        match &request.source {
            MediaLibrarySource::PictureDir => {
                let pictures_dir = if let Some(dir) =
                    std::env::var_os("XDG_PICTURES_DIR").map(std::path::PathBuf::from)
                {
                    Some(dir)
                } else {
                    match xdg_user::pictures() {
                        Ok(Some(dir)) => Some(dir),
                        Ok(None) | Err(_) => None,
                    }
                }
                .ok_or_else(|| Error::MediaLibrarySourceForbidden(request.source.clone()))?;

                let mut items = Vec::new();
                if let Ok(entries) = fs::read_dir(&pictures_dir) {
                    let mut all_entries: Vec<_> = entries.flatten().collect();
                    all_entries.sort_by_key(|e| e.file_name());

                    for entry in all_entries
                        .into_iter()
                        .skip(request.offset)
                        .take(request.limit)
                    {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                let ext = ext.to_ascii_lowercase();
                                // determine mime type by extension for now
                                let is_image = matches!(
                                    ext.as_str(),
                                    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp"
                                );
                                if is_image {
                                    let mime_type = match ext.as_str() {
                                        "jpg" | "jpeg" => "image/jpeg",
                                        "png" => "image/png",
                                        "gif" => "image/gif",
                                        "bmp" => "image/bmp",
                                        "tiff" => "image/tiff",
                                        "webp" => "image/webp",
                                        _ => "application/octet-stream",
                                    }
                                    .to_string();

                                    items.push(ImageInfo {
                                        path: path.to_string_lossy().to_string(),
                                        content_uri: format!("file://{}", path.to_string_lossy()),
                                        mime_type,
                                        meta_data: None,
                                    });
                                }
                            }
                        }
                    }
                }
                Ok(GetImagesResult { items })
            }
        }
    }
}
