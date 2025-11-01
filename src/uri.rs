use fluent_uri::Uri;
use log::error;
use std::path::PathBuf;
use urlencoding::{decode, encode};

use crate::Error;
use regex::Regex;

fn with_encoded_path(uri: &str) -> String {
    let re = Regex::new(r"^(?P<scheme>[^:/?#]+://)?(?P<host>[^/?#]+)(?P<path>/?.*)$").unwrap();

    if let Some(caps) = re.captures(uri) {
        let scheme = caps.name("scheme").map_or("", |m| m.as_str());
        let host = caps["host"].to_string();
        let path = &caps["path"];

        let encoded_path = path
            .split('/')
            .map(|segment| encode(segment).into_owned()) // Encode each segment
            .collect::<Vec<String>>()
            .join("/");

        format!("{}{}{}", scheme, host, encoded_path)
    } else {
        encode(uri).into_owned()
    }
}

pub fn parse_uri(uri: &str) -> crate::Result<Uri<String>> {
    let uri = with_encoded_path(&uri.replace("\\", "/"));

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
        Err(err) => Err(Error::ParseUriError(format!("uri: {uri} {err}"))),
    }
}

#[cfg(target_os = "windows")]
fn get_authority_as_string(input: &Uri<&str>) -> String {
    match input.authority() {
        Some(authority) => authority.to_string(),
        None => String::new(),
    }
}

fn decode_path(uri: &Uri<&str>) -> String {
    match decode(&uri.path().to_string()) {
        Ok(decoded_path) => {
            #[cfg(target_os = "windows")]
            return format!(
                "{}{}",
                get_authority_as_string(uri),
                decoded_path.replace("/", "\\")
            );

            #[cfg(not(target_os = "windows"))]
            return decoded_path.into_owned();
        }
        Err(_) => uri.to_string(),
    }
}

pub fn uri_to_path(uri: &str) -> crate::Result<std::path::PathBuf> {
    match parse_uri(uri) {
        Ok(uri) => {
            let path = decode_path(uri.borrow());
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
        let r = uri_to_path("file://C:/Users/Test/My Pictures/my_file.jpg").unwrap();

        #[cfg(target_os = "windows")]
        assert_eq!(
            r.to_str().unwrap(),
            "C:\\Users\\Test\\My Pictures\\my_file.jpg"
        );
        #[cfg(not(target_os = "windows"))]
        assert_eq!(r.to_str().unwrap(), "/Users/Test/My Pictures/my_file.jpg");
    }
}
