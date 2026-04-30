use http::{header::*, response::Builder as ResponseBuilder, status::StatusCode};
use tauri::{AppHandle, Runtime};

use crate::MedialibraryExt;

pub(crate) async fn get_response<R: Runtime>(
    request: http::Request<Vec<u8>>,
    app: &AppHandle<R>,
) -> Result<http::Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let uri_str = percent_encoding::percent_decode(&request.uri().to_string().as_bytes())
        .decode_utf8_lossy()
        .to_string();

    let thumbnail_response = app.medialibrary().get_thumbnail_data(uri_str).await?;
    let file_len = thumbnail_response.content.len();

    Ok(ResponseBuilder::new()
        .status(StatusCode::OK)
        // .header(CONTENT_TYPE, "image/png") //ToDo determine contenttype
        .header(CONTENT_LENGTH, file_len)
        .body(thumbnail_response.content)?)
}
