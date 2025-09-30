use allmytoes::{AMTConfiguration, ThumbSize, AMT};
use base64::{engine::general_purpose, Engine as _};
use std::fs;
use std::path::Path;

use crate::{thumbnail_provider::ThumbnailProvider, Error, GetThumbnailResponse};

pub struct XdgThumbnailProvider;

impl ThumbnailProvider for XdgThumbnailProvider {
    fn get_thumbnail(path: &Path) -> crate::Result<GetThumbnailResponse> {
        let configuration = AMTConfiguration::default();
        let amt = AMT::new(&configuration);

        let thumb_size = ThumbSize::Large;
        match amt.get(path, thumb_size) {
            Ok(thumb) => {
                let bytes = fs::read(&thumb.path)?;
                let content = general_purpose::STANDARD.encode(&bytes);

                Ok(GetThumbnailResponse { content })
            }
            Err(error) => Err(Error::AllMyToes(format!("get_thumbnail error: {error:?}"))),
        }
    }
}
