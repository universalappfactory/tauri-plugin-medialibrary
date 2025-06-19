const COMMANDS: &[&str] = &[
    "get_images",
    "get_thumbnail",
    "get_available_sources",
    "request_permissions",
];

#[derive(schemars::JsonSchema)]
// #[serde(untagged)]
#[allow(unused)]
pub enum MediaLibrarySource {
    #[cfg(not(target_os = "android"))]
    PictureDir,

    #[cfg(target_os = "android")]
    ExternalStorage,
}

/// HTTP scope entry.
#[derive(schemars::JsonSchema)]
#[serde(untagged)]
#[allow(unused)]
enum MediaLibraryScopeEntry {
    /// A URL that can be accessed by the webview when using the HTTP APIs.
    /// Wildcards can be used following the URL pattern standard.
    ///
    /// See [the URL Pattern spec](https://urlpattern.spec.whatwg.org/) for more information.
    ///
    /// Examples:
    ///
    /// - "https://*" : allows all HTTPS origin on port 443
    ///
    /// - "https://*:*" : allows all HTTPS origin on any port
    ///
    /// - "https://*.github.com/tauri-apps/tauri": allows any subdomain of "github.com" with the "tauri-apps/api" path
    ///
    /// - "https://myapi.service.com/users/*": allows access to any URLs that begins with "https://myapi.service.com/users/"
    Value(MediaLibrarySource),
    Object {
        /// A URL that can be accessed by the webview when using the HTTP APIs.
        /// Wildcards can be used following the URL pattern standard.
        ///
        /// See [the URL Pattern spec](https://urlpattern.spec.whatwg.org/) for more information.
        ///
        /// Examples:
        ///
        /// - "https://*" : allows all HTTPS origin on port 443
        ///
        /// - "https://*:*" : allows all HTTPS origin on any port
        ///
        /// - "https://*.github.com/tauri-apps/tauri": allows any subdomain of "github.com" with the "tauri-apps/api" path
        ///
        /// - "https://myapi.service.com/users/*": allows access to any URLs that begins with "https://myapi.service.com/users/"
        source: MediaLibrarySource,
    },
}

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .global_scope_schema(schemars::schema_for!(MediaLibraryScopeEntry))
        .build();
}
