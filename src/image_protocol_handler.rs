use http::{header::*, response::Builder as ResponseBuilder, status::StatusCode};
use std::io::Read;
use tauri::{AppHandle, Runtime};

pub(crate) async fn get_response<R: Runtime>(
    request: http::Request<Vec<u8>>,
    _app: &AppHandle<R>,
) -> Result<http::Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let path = percent_encoding::percent_decode(&request.uri().path().as_bytes())
        .decode_utf8_lossy()
        .to_string();

    let mut file = std::fs::File::open(&path)?;
    let file_len = file.metadata()?.len();

    let mut buf = Vec::with_capacity(file_len as usize);
    file.read_to_end(&mut buf)?;

    Ok(ResponseBuilder::new()
        .status(StatusCode::OK)
        // .header(CONTENT_TYPE, "image/png") //ToDo determine contenttype
        .header(CONTENT_LENGTH, file_len)
        .body(buf)?)
}
