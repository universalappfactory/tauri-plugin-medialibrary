use crate::{
    directory_reader::DirectoryReader, Error, GetImagesResult, GetLibraryContentRequest, ImageInfo,
    MediaLibrarySource,
};
use std::{fs, path::{Path}};
use log::trace;
pub struct PathReader<'a>
{
    path: &'a Path
}

impl<'a> PathReader<'a> {
    pub fn new(path: &'a Path) -> Self {
        PathReader { path }
    }
}

impl<'a> DirectoryReader for PathReader<'a> {
    fn read_directory(&self, request: &GetLibraryContentRequest) -> Result<GetImagesResult, Error> {
        trace!("windows read_directory: {:?}", self.path);
        match &request.source {
            MediaLibrarySource::PictureDir => { 
                let mut items = Vec::new();


                if let Ok(entries) = fs::read_dir(self.path) {
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
