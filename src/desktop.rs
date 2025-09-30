use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::directory_reader::DirectoryReader;
#[cfg(not(feature = "xdg"))]
use crate::path_reader::PathReader;
use crate::thumbnail_provider::ThumbnailProvider;

#[cfg(all(not(feature = "thumb_cache"), not(feature = "xdg")))]
use crate::thumbnail_provider::EmptyThumbnailProvider;

#[cfg(feature = "thumb_cache")]
use crate::thumbcache_thumbnail_provider::ThumbCacheThumbnailProvider;
#[cfg(feature = "xdg")]
use crate::{
    xdg_directory_reader::XdgDirectoryReader, xdg_thumbnail_provider::XdgThumbnailProvider,
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
        {
            let reader = XdgDirectoryReader;
            return reader.read_directory(&request);
        }
        #[cfg(not(feature = "xdg"))]
        {
            use tauri::Manager;
            match self.0.path().picture_dir() {
                Ok(path) => {
                    let reader = PathReader::new(&path);
                    return reader.read_directory(&request);
                }
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
                #[cfg(feature = "xdg")]
                return XdgThumbnailProvider::get_thumbnail(&path);
                #[cfg(feature = "thumb_cache")]
                return ThumbCacheThumbnailProvider::get_thumbnail(&path);

                #[cfg(all(not(feature = "thumb_cache"), not(feature = "xdg")))]
                EmptyThumbnailProvider::get_thumbnail(&path)
            }
            Err(err) => Err(err),
        }
    }
}
