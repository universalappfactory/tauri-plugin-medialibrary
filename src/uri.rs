use fluent_uri::Uri;
use std::path::PathBuf;

use crate::Error;
use log::{trace,error};

pub fn parse_uri(uri: &str) -> crate::Result<Uri<String>> {


    #[cfg(target_os = "windows")]
    let uri = &uri.replace("\\", "/");

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

fn get_authority_as_string(input: &Uri<&str>) -> String
{
    match input.authority() {
        Some(authority) => authority.to_string(),
        None => String::new()
    }
}

pub fn uri_to_path(uri: &str) -> crate::Result<std::path::PathBuf> {
    trace!("uri_to_path: {}", uri);
    match parse_uri(uri) {
        Ok(uri) => {
            #[cfg(target_os = "windows")]
            let path =format!("{}{}", get_authority_as_string(uri.borrow()), uri.path().to_string().replace("/", "\\"));

            #[cfg(not(target_os = "windows"))]
            let path = uri.path().to_string();
            Ok(PathBuf::from(path))
        }
        Err(e) => {
            error!("err: {}", e);
            Err(e)
        },
    }
}


#[cfg(test)]
mod tests {
    use crate::uri::uri_to_path;


    #[test]
    pub fn test_uri_to_path_for_windows_path()
    {
        let r = uri_to_path("file://C:/Users/Test/Pictures/my_file.jpg").unwrap();
        assert_eq!(r.to_str().unwrap(), "C:\\Users\\Test\\Pictures\\my_file.jpg");
    }

    
}