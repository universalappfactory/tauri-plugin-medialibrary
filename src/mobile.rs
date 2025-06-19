use serde::{de::DeserializeOwned, Serialize};
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_medialibrary);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Medialibrary<R>> {
    #[cfg(target_os = "android")]
    let handle =
        api.register_android_plugin("de.universalappfactory.medialibrary", "MediaLibraryPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_medialibrary)?;
    Ok(Medialibrary(handle))
}

/// Access to the medialibrary APIs.
pub struct Medialibrary<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Medialibrary<R> {
    pub fn get_images(&self, request: GetLibraryContentRequest) -> crate::Result<GetImagesResult> {
        self.0
            .run_mobile_plugin("getImages", request)
            .map_err(Into::into)
    }

    pub fn check_permissions(&self) -> crate::Result<PermissionResponse> {
        self.0
            .run_mobile_plugin::<PermissionResponse>("checkPermissions", ())
            .map_err(Into::into)
    }

    pub async fn get_thumbnail(&self, uri: String) -> crate::Result<GetThumbnailResponse> {
        return self
            .0
            .run_mobile_plugin("getThumbnailAsBase64", uri)
            .map_err(Into::into);
    }

    pub fn request_permissions(
        &self,
        args: RequestPermissionsArgs,
    ) -> crate::Result<PermissionResponse> {
        self.0
            .run_mobile_plugin("requestPermissions", args)
            .map_err(Into::into)
    }
}
