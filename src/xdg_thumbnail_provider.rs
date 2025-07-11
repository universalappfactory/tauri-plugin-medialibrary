use allmytoes::{AMTConfiguration, ThumbSize, AMT};
use base64::{engine::general_purpose, Engine as _};
use std::fs;
use std::path::Path;

use crate::{thumbnail_provider::ThumbnailProvider, Error, GetThumbnailResponse};

pub struct XdgThumbnailProvider;

impl ThumbnailProvider for XdgThumbnailProvider {
    fn get_thumbnail(path: &Path) -> crate::Result<GetThumbnailResponse> {
        // The configuration for allmytoes
        // Usually, the defaults are fine.
        let configuration = AMTConfiguration::default();

        // An instance of allmytoes that can be used to provide thumbnails
        let amt = AMT::new(&configuration);

        // The size of the thumbnail we want
        let thumb_size = ThumbSize::Large;

        // Get a thumbnail
        match amt.get(path, thumb_size) {
            Ok(thumb) => {
                let bytes = fs::read(&thumb.path)?;
                let content = general_purpose::STANDARD.encode(&bytes);

                return Ok(GetThumbnailResponse { content });
            }
            Err(error) => Err(Error::AllMyToes(format!("allmytoes error: {:?}", error))),
        }
    }
}
