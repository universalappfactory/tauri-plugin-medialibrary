use fluent_uri::Uri;
use log::error;
use std::path::PathBuf;
use urlencoding::{decode, encode};

use crate::Error;

fn with_encoded_path(uri: &str) -> String {
    match uri.rfind('/') {
        Some(pos) => {
            let (prefix, last_segment) = uri.split_at(pos + 1);
            format!("{}{}", prefix, encode(last_segment).into_owned())
        }
        None => encode(uri).into_owned(),
    }
}

pub fn parse_uri(uri: &str) -> crate::Result<Uri<String>> {
    #[cfg(target_os = "windows")]
    let uri = &uri.replace("\\", "/");

    let uri = with_encoded_path(uri);
    match Uri::parse(uri.as_bytes()) {
        Ok(uri) => match uri.scheme() {
            Some(scheme) => {
                if scheme.eq_lowercase("file") {
                    Ok(uri.to_owned())
                } else {
                    Err(Error::InvalidUriScheme(scheme.to_string()))
                }
            }
            _ => Err(Error::InvalidUriScheme(uri.to_string())),
        },
        Err(err) => Err(Error::ParseUriError(format!("uri: {err}"))),
    }
}

#[cfg(target_os = "windows")]
fn get_authority_as_string(input: &Uri<&str>) -> String {
    match input.authority() {
        Some(authority) => authority.to_string(),
        None => String::new(),
    }
}

pub fn uri_to_path(uri: &str) -> crate::Result<std::path::PathBuf> {
    match parse_uri(uri) {
        Ok(uri) => {
            #[cfg(target_os = "windows")]
            let path = format!(
                "{}{}",
                get_authority_as_string(uri.borrow()),
                uri.path().to_string().replace("/", "\\")
            );

            #[cfg(not(target_os = "windows"))]
            let path = uri.path().to_string();

            let path = match decode(&uri.path().to_string()) {
                Ok(decoded_path) => decoded_path.into_owned(),
                Err(_) => path,
            };

            Ok(PathBuf::from(path))
        }
        Err(e) => {
            error!("err: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uri::uri_to_path;

    #[test]
    pub fn test_uri_to_path_for_windows_path() {
        let r = uri_to_path("file://C:/Users/Test/Pictures/my_file.jpg").unwrap();

        #[cfg(target_os = "windows")]
        assert_eq!(
            r.to_str().unwrap(),
            "C:\\Users\\Test\\Pictures\\my_file.jpg"
        );
        #[cfg(not(target_os = "windows"))]
        assert_eq!(r.to_str().unwrap(), "/Users/Test/Pictures/my_file.jpg");
    }
}
