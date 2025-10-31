use base64::{engine::general_purpose, Engine as _};
use log::{error, trace};
use std::fs;
use std::path::Path;

use crate::{thumbnail_provider::ThumbnailProvider, Error, GetThumbnailResponse};

pub struct ThumbCacheThumbnailProvider;

impl ThumbnailProvider for ThumbCacheThumbnailProvider {
    fn get_thumbnail(path: &Path) -> crate::Result<GetThumbnailResponse> {
        trace!("get_thumbnail for: {}", path.to_str().unwrap_or_default());
        match thumbcache::get_bmp(
            path.to_str().unwrap_or_default(),
            thumbcache::ThumbSize::S96,
        ) {
            Ok(bmp) => {
                let content = general_purpose::STANDARD.encode(&bmp);
                Ok(GetThumbnailResponse { content })
            }
            Err(error) => {
                error!("err: {}", error);
                Err(Error::AllMyToes(format!("a error: {error:?}")))
            }
        }
    }
}
