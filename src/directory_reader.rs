use crate::{Error, GetImagesResult, GetLibraryContentRequest};

pub trait DirectoryReader {
    fn read_directory(request: &GetLibraryContentRequest) -> Result<GetImagesResult, Error>;
}
