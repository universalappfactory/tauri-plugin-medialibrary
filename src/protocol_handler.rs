use std::path::Path;

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

pub(crate) fn build_uri_from_path(scheme: &str, path: &Path) -> String {
    #[cfg(target_os = "windows")]
    {
        use percent_encoding::NON_ALPHANUMERIC;

        let path =
            percent_encoding::utf8_percent_encode(&(path.to_string_lossy()), NON_ALPHANUMERIC)
                .to_string();
        return format!("http://{}.localhost/{}", scheme, path);
    }

    #[cfg(not(target_os = "windows"))]
    {
        return format!(
            "{}://localhost{}",
            scheme,
            path.to_str().unwrap_or_default()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_build_uri_from_path() {
        let path = Path::new("/path/to/file.txt");
        assert_eq!(
            build_uri_from_path("image", path),
            "image://localhost/path/to/file.txt"
        );
        assert_eq!(
            build_uri_from_path("thumbnail", path),
            "thumbnail://localhost/path/to/file.txt"
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_get_uri_string_from_request() {
        // URIs without percent-encoding are returned as-is.
        let request = http::Request::builder()
            .uri("image://localhost/path/to/file.txt")
            .body(vec![])
            .unwrap();
        assert_eq!(
            get_uri_string_from_request(&request),
            "image://localhost/path/to/file.txt"
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_get_uri_string_from_request_decodes_percent_encoding() {
        // Percent-encoded characters (e.g. spaces) must be decoded.
        let request = http::Request::builder()
            .uri("image://localhost/path/to/file%20with%20spaces.txt")
            .body(vec![])
            .unwrap();
        assert_eq!(
            get_uri_string_from_request(&request),
            "image://localhost/path/to/file with spaces.txt"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_build_uri_from_path() {
        let path = Path::new("C:\\path\\to\\file.txt");
        assert_eq!(
            build_uri_from_path("image", path),
            "http://image.localhost/C%3A%5Cpath%5Cto%5Cfile%2Etxt"
        );
        assert_eq!(
            build_uri_from_path("thumbnail", path),
            "http://thumbnail.localhost/C%3A%5Cpath%5Cto%5Cfile%2Etxt"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_get_uri_string_from_request() {
        // URIs without percent-encoding are returned as-is.
        let request = http::Request::builder()
            .uri("thumbnail://localhost/C%3A%5Cpath%5Cto%5Cfile%2Etxt")
            .body(vec![])
            .unwrap();
        assert_eq!(
            get_uri_string_from_request(&request),
            "thumbnail://C:\\path\\to\\file.txt"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_get_uri_string_from_request_decodes_percent_encoding() {
        // Percent-encoded characters (e.g. spaces) must be decoded.
        let request = http::Request::builder()
            .uri("thumbnail://localhost/C%3A%5Cpath%5Cto%5Cfile%20with%20spaces.txt")
            .body(vec![])
            .unwrap();
        assert_eq!(
            get_uri_string_from_request(&request),
            "thumbnail://C:\\path\\to\\file with spaces.txt"
        );
    }
}
