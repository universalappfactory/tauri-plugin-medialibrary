use crate::MediaLibrarySource;
use serde::Deserialize;

#[derive(Debug)]
pub struct Entry {
    pub source: MediaLibrarySource,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum EntryRaw {
    Value(MediaLibrarySource),
    Object { source: MediaLibrarySource },
}
