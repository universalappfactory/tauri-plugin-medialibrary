use crate::{Error, GetImagesResult, GetLibraryContentRequest};

pub trait DirectoryReader {
    fn read_directory(&self, request: &GetLibraryContentRequest) -> Result<GetImagesResult, Error>;
}
