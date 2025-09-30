use base64::{engine::general_purpose, Engine as _};
use std::fs;
use std::path::Path;

use crate::{thumbnail_provider::ThumbnailProvider, Error, GetThumbnailResponse};

pub struct WindowsThumbnailProvider;

impl ThumbnailProvider for WindowsThumbnailProvider {
    fn get_thumbnail(path: &Path) -> crate::Result<GetThumbnailResponse> {
        
        Ok(GetThumbnailResponse {  content: "".to_owned() })

        
    }
}
