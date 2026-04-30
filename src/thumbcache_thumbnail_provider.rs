use log::{error, trace};
use std::path::Path;

use crate::{thumbnail_provider::ThumbnailProvider, Error, Thumbnail};

pub struct ThumbCacheThumbnailProvider;

impl ThumbnailProvider for ThumbCacheThumbnailProvider {
    fn get_thumbnail(path: &Path) -> crate::Result<Thumbnail> {
        trace!("get_thumbnail for: {}", path.to_str().unwrap_or_default());
        match thumbcache::get_bmp(
            path.to_str().unwrap_or_default(),
            thumbcache::ThumbSize::S96,
        ) {
            Ok(bmp) => Ok(bmp.into()),
            Err(error) => {
                error!("err: {}", error);
                Err(Error::AllMyToes(format!("a error: {error:?}")))
            }
        }
    }
}
