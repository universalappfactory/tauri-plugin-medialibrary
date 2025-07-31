use fluent_uri::Uri;
use std::path::PathBuf;

use crate::Error;

pub fn parse_uri(uri: &str) -> crate::Result<Uri<String>> {
    match Uri::parse(uri) {
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
        Err(err) => Err(Error::ParseUriError(format!(
            "uri: {uri}, {err}"
        ))),
    }
}

pub fn uri_to_path(uri: &str) -> crate::Result<std::path::PathBuf> {
    match parse_uri(uri) {
        Ok(uri) => {
            let path = uri.path().to_string();
            Ok(PathBuf::from(path))
        }
        Err(e) => Err(e),
    }
}
