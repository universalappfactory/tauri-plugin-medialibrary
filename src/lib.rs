use tauri::{
    ipc::ScopeObject,
    plugin::{Builder, TauriPlugin},
    utils::acl::Value,
    AppHandle, Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod directory_reader;
mod error;
mod models;
mod scope;

mod thumbnail_provider;
mod uri;
#[cfg(feature = "xdg")]
mod xdg_directory_reader;
#[cfg(feature = "xdg")]
mod xdg_thumbnail_provider;

#[cfg(feature = "thumb_cache")]
mod thumbcache_thumbnail_provider;
mod walkdir_reader;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Medialibrary;
#[cfg(mobile)]
use mobile::Medialibrary;

// implement ScopeObject here instead of in the scope module because it is also used on the build script
// and we don't want to add tauri as a build dependency
impl ScopeObject for scope::Entry {
    type Error = Error;
    fn deserialize<R: Runtime>(
        _app: &AppHandle<R>,
        raw: Value,
    ) -> std::result::Result<Self, Self::Error> {
        match serde_json::from_value(raw.into()).map(|raw| match raw {
            scope::EntryRaw::Object { source } => source,
            scope::EntryRaw::Value(val) => val,
        }) {
            Ok(source) => Ok(Self { source }),
            Err(err) => Err(err.into()),
        }
    }
}

pub trait MedialibraryExt<R: Runtime> {
    fn medialibrary(&self) -> &Medialibrary<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MedialibraryExt<R> for T {
    fn medialibrary(&self) -> &Medialibrary<R> {
        self.state::<Medialibrary<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("medialibrary")
        .invoke_handler(tauri::generate_handler![
            commands::get_images,
            commands::get_thumbnail,
            commands::get_available_sources,
            commands::request_permissions,
            commands::get_image,
            commands::delete_image
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let medialibrary = mobile::init(app, api)?;
            #[cfg(desktop)]
            let medialibrary = desktop::init(app, api)?;
            app.manage(medialibrary);
            Ok(())
        })
        .build()
}
