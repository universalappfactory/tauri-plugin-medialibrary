use std::path::Path;

use crate::GetThumbnailResponse;

pub trait ThumbnailProvider {
    fn get_thumbnail(path: &Path) -> crate::Result<GetThumbnailResponse>;
}
