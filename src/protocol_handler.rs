pub(crate) fn get_uri_string_from_request(request: &http::Request<Vec<u8>>) -> String {
    #[cfg(target_os = "windows")]
    {
        percent_encoding::percent_decode(request.uri().to_string().as_bytes())
            .decode_utf8_lossy()
            .to_string()
            .replace("localhost/", "")
    }

    #[cfg(not(target_os = "windows"))]
    {
        percent_encoding::percent_decode(request.uri().to_string().as_bytes())
            .decode_utf8_lossy()
            .to_string()
    }
}
