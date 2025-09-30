use serde::{ser::Serializer, Serialize};

use crate::MediaLibrarySource;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error("forbidden path: {0}")]
    MediaLibrarySourceForbidden(MediaLibrarySource),
    #[error("XDG_PICTURES_DIR is not set")]
    XdgPicturesDirNotSet,
    #[error("unsupported media library source: {0}")]
    MediaLibrarySourceNotSupported(MediaLibrarySource),
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
    #[error("amytoes error: {0}")]
    AllMyToes(String),
    #[error("cannot parse uri: {0}")]
    ParseUriError(String),
    #[error("invalid uri scheme: {0}")]
    InvalidUriScheme(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
