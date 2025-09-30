use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

#[cfg(target_os = "linux")]
use crate::{
    directory_reader::DirectoryReader, thumbnail_provider::ThumbnailProvider,
    xdg_directory_reader::XdgDirectoryReader, xdg_thumbnail_provider::XdgThumbnailProvider,
};
#[cfg(target_os = "windows")]
use crate::{
    directory_reader::DirectoryReader, thumbnail_provider::ThumbnailProvider,
    path_reader::PathReader, windows_thumbnail_provider::WindowsThumbnailProvider,
};


use crate::{models::*, uri::uri_to_path};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Medialibrary<R>> {
    Ok(Medialibrary(app.clone()))
}

/// Access to the medialibrary APIs.
pub struct Medialibrary<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Medialibrary<R> {
    pub fn get_images(&self, request: GetLibraryContentRequest) -> crate::Result<GetImagesResult> {
        #[cfg(feature = "xdg")]
        return XdgDirectoryReader::read_directory(&request);
        #[cfg(not(feature = "xdg"))]
        {
            use tauri::Manager;
            match self.0.path().picture_dir() {
                Ok(path) => {
                    let reader = PathReader::new(&path);
                    return reader.read_directory(&request);
                },
                Err(e) => Err(e.into()),
            }
        }
    }

    pub fn get_image(&self, _request: GetImageRequest) -> crate::Result<Option<ImageInfo>> {
        todo!()
    }

    pub fn delete_image(&self, request: DeleteImageRequest) -> crate::Result<()> {
        match uri_to_path(&request.uri) {
            Ok(path) => match std::fs::remove_file(&path) {
                Ok(_) => Ok(()),
                Err(err) => Err(err.into()),
            },
            Err(err) => Err(err),
        }
    }

    pub fn check_permissions(&self) -> crate::Result<PermissionResponse> {
        Ok(PermissionResponse::granted())
    }

    pub fn request_permissions(
        &self,
        _args: RequestPermissionsArgs,
    ) -> crate::Result<PermissionResponse> {
        Ok(PermissionResponse::granted())
    }

    pub async fn get_thumbnail(&self, uri: String) -> crate::Result<GetThumbnailResponse> {
        match uri_to_path(&uri) {
            Ok(path) => {
                #[cfg(target_os = "linux")]
                return XdgThumbnailProvider::get_thumbnail(&path);
                #[cfg(target_os = "windows")]
                return WindowsThumbnailProvider::get_thumbnail(&path);
            }
            Err(err) => Err(err),
        }
    }
}
