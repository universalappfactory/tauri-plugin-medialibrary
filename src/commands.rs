use crate::scope::Entry;
use crate::MedialibraryExt;
use crate::Result;
use crate::{models::*, Error};
use tauri::ipc::{CommandScope, GlobalScope};
use tauri::{command, AppHandle, Runtime};

#[command]
pub(crate) async fn get_images<R: Runtime>(
    app: AppHandle<R>,
    global_scope: GlobalScope<Entry>,
    command_scope: CommandScope<Entry>,
    request: GetLibraryContentRequest,
) -> Result<GetImagesResult> {
    if !global_scope
        .allows()
        .iter()
        .chain(command_scope.allows())
        .any(|c| c.source.eq(&request.source))
    {
        return Err(Error::MediaLibrarySourceForbidden(request.source));
    }

    let include_file_metadata = match request.include_file_metadata {
        Some(include) => include,
        None => false,
    };

    match app.medialibrary().get_images(request) {
        Ok(images) => Ok(images.with_file_metadata(include_file_metadata)),
        Err(err) => Err(err),
    }
}

#[command]
pub(crate) async fn request_permissions<R: Runtime>(
    app: AppHandle<R>,
    args: RequestPermissionsArgs,
) -> Result<PermissionResponse> {
    app.medialibrary().request_permissions(args)
}

#[command]
pub(crate) async fn get_thumbnail<R: Runtime>(
    app: AppHandle<R>,
    uri: String,
) -> Result<GetThumbnailResponse> {
    return app.medialibrary().get_thumbnail(uri).await;
}

#[command]
pub(crate) async fn get_image<R: Runtime>(
    app: AppHandle<R>,
    uri: String,
) -> Result<Option<ImageInfo>> {
    app.medialibrary().get_image(uri.into())
}

#[command]
pub(crate) async fn get_available_sources<R: Runtime>(
    _app: AppHandle<R>,
    global_scope: GlobalScope<Entry>,
    _command_scope: CommandScope<Entry>,
) -> Result<Vec<MediaLibrarySource>> {
    let allowed_sources = global_scope
        .allows()
        .iter()
        .map(|f| f.source.clone())
        .collect();

    Ok(allowed_sources)
}
