use http::{header::*, response::Builder as ResponseBuilder, status::StatusCode};
use tauri::{AppHandle, Runtime};

use crate::{protocol_handler::get_uri_string_from_request, MedialibraryExt};

pub(crate) async fn get_response<R: Runtime>(
    request: http::Request<Vec<u8>>,
    app: &AppHandle<R>,
) -> Result<http::Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let uri_str = get_uri_string_from_request(&request);

    let thumbnail_response = app.medialibrary().get_thumbnail_data(uri_str).await?;
    let file_len = thumbnail_response.content.len();

    Ok(ResponseBuilder::new()
        .status(StatusCode::OK)
        .header(CONTENT_LENGTH, file_len)
        .body(thumbnail_response.content)?)
}
