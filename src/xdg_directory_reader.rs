use crate::{
    directory_reader::DirectoryReader, path_reader::PathReader, Error, GetImagesResult,
    GetLibraryContentRequest, MediaLibrarySource,
};

pub struct XdgDirectoryReader;

impl DirectoryReader for XdgDirectoryReader {
    fn read_directory(&self, request: &GetLibraryContentRequest) -> Result<GetImagesResult, Error> {
        match &request.source {
            MediaLibrarySource::PictureDir => {
                let pictures_dir = if let Some(dir) =
                    std::env::var_os("XDG_PICTURES_DIR").map(std::path::PathBuf::from)
                {
                    Some(dir)
                } else {
                    match xdg_user::pictures() {
                        Ok(Some(dir)) => Some(dir),
                        Ok(None) | Err(_) => None,
                    }
                }
                .ok_or_else(|| Error::MediaLibrarySourceForbidden(request.source.clone()))?;

                let reader = PathReader::new(&pictures_dir);
                reader.read_directory(request)
            }
        }
    }
}
