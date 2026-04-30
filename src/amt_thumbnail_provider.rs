use allmytoes::{AMTConfiguration, ThumbSize, AMT};
use std::fs;
use std::path::Path;

use crate::{thumbnail_provider::ThumbnailProvider, Error, Thumbnail};

pub struct AmtThumbnailProvider;

impl ThumbnailProvider for AmtThumbnailProvider {
    fn get_thumbnail(path: &Path) -> crate::Result<Thumbnail> {
        let configuration = AMTConfiguration::default();
        let amt = AMT::new(&configuration);

        let thumb_size = ThumbSize::Large;
        match amt.get(path, thumb_size) {
            Ok(thumb) => {
                let bytes = fs::read(&thumb.path)?;
                Ok(bytes.into())
            }
            Err(error) => Err(Error::AllMyToes(format!("get_thumbnail error: {error:?}"))),
        }
    }
}
