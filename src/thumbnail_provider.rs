use std::path::Path;

#[cfg(all(not(feature = "thumb_cache"), not(feature = "amt")))]
use log::warn;

use crate::GetThumbnailResponse;

pub trait ThumbnailProvider {
    fn get_thumbnail(path: &Path) -> crate::Result<GetThumbnailResponse>;
}

#[cfg(all(not(feature = "thumb_cache"), not(feature = "amt")))]
pub struct EmptyThumbnailProvider;

#[cfg(all(not(feature = "thumb_cache"), not(feature = "amt")))]
impl ThumbnailProvider for EmptyThumbnailProvider {
    fn get_thumbnail(_path: &Path) -> crate::Result<GetThumbnailResponse> {
        warn!("using EmptyThumbnailProvider");
        Ok(GetThumbnailResponse::default())
    }
}
