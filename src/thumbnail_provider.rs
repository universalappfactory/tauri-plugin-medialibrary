use std::path::Path;

#[cfg(all(not(feature = "thumb_cache"), not(feature = "amt")))]
use log::warn;

use crate::Thumbnail;

pub trait ThumbnailProvider {
    fn get_thumbnail(path: &Path) -> crate::Result<Thumbnail>;
}

#[cfg(all(not(feature = "thumb_cache"), not(feature = "amt")))]
pub struct EmptyThumbnailProvider;

#[cfg(all(not(feature = "thumb_cache"), not(feature = "amt")))]
impl ThumbnailProvider for EmptyThumbnailProvider {
    fn get_thumbnail(_path: &Path) -> crate::Result<Thumbnail> {
        warn!("using EmptyThumbnailProvider");
        Ok(Thumbnail::default())
    }
}
