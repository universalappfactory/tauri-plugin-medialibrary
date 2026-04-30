use http::{header::*, response::Builder as ResponseBuilder, status::StatusCode};
use tauri::{AppHandle, Runtime};

use crate::MedialibraryExt;

pub(crate) async fn get_response<R: Runtime>(
    request: http::Request<Vec<u8>>,
    _app: &AppHandle<R>,
) -> Result<http::Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let uri_str = percent_encoding::percent_decode(request.uri().to_string().as_bytes())
        .decode_utf8_lossy()
        .to_string();

    let image = _app.medialibrary().get_image_data(uri_str).await?;
    let file_len = image.content.len();

    Ok(ResponseBuilder::new()
        .status(StatusCode::OK)
        // .header(CONTENT_TYPE, "image/png") //ToDo determine contenttype
        .header(CONTENT_LENGTH, file_len)
        .body(image.content)?)
}
