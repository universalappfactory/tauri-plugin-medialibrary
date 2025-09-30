use std::path::Path;

use crate::GetThumbnailResponse;

pub trait ThumbnailProvider {
    fn get_thumbnail(path: &Path) -> crate::Result<GetThumbnailResponse>;
}

pub struct EmptyThumbnailProvider;

impl ThumbnailProvider for EmptyThumbnailProvider {
    fn get_thumbnail(_path: &Path) -> crate::Result<GetThumbnailResponse> {
        Ok(GetThumbnailResponse::default())
    }
}
