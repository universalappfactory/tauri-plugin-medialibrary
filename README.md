# Tauri Media Library Plugin

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | x         |
| Android  | ✓         |
| iOS      | x         |

This plugin is aimed to give you easy access to the system's "media library".
It returns a list of image metadata such as path, uri (e.g. the content uri on android) and mime type.

It has also functions to get a thumbnail for a image.

## Android

On android the plugin reads the content of the android media store.

## Linux

On Linux it reads pictures from the XDG_PICTURE directory.

## Windows

On windows it uses the:
```
https://tauri.app/reference/javascript/api/namespacepath/#picturedir
```

### Thumbnails

Thumbnails can be read using the https://crates.io/crates/thumbcache crate.
When using thumbcache, you have to use the `thumb_cache` feature

```
cargo build --features thumb_cache
```

## IOs

Not implemented yet

## Install

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`

```toml
[dependencies]
# cargo crate
tauri-plugin-medialibrary = "0.12.0"
# alternatively with Git:
tauri-plugin-medialibrary = { git = "https://github.com/universalappfactory/tauri-plugin-medialibrary" }
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```sh
pnpm add @universalappfactory/tauri-plugin-medialibrary
# or
npm add @universalappfactory/tauri-plugin-medialibrary
# or
yarn add @universalappfactory/tauri-plugin-medialibrary

# alternatively with Git:
pnpm add https://github.com/universalappfactory/tauri-plugin-medialibrary
# or
npm add https://github.com/universalappfactory/tauri-plugin-medialibrary
# or
yarn add https://github.com/universalappfactory/tauri-plugin-medialibrary
```

## Usage

First you need to register the core plugin with Tauri:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_medialibrary::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Next you have to configure the permissions:

You must configure, which image sources are allowed, e.g you can add a

`src-tauri/capabilities/default.json`
```
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    {
      "identifier": "medialibrary:global-scope",
      "allow": [
        {
          "source": "PictureDir"
        }
      ]
    },
    "medialibrary:allow-get-images",
    "medialibrary:allow-get-image",
    "medialibrary:allow-get-thumbnail",
    "medialibrary:allow-get-available-sources",
    "medialibrary:allow-request-permissions",
  ]
}
```
and

`src-tauri/capabilities/default.android.json`

```
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    {
      "identifier": "medialibrary:global-scope",
      "allow": [
        {
          "source": "ExternalStorage"
        },
        {
          "source": "VolumeExternalPrimary"
        }
      ]
    },
    "medialibrary:allow-get-images",
    "medialibrary:allow-get-image",
    "medialibrary:allow-get-thumbnail",
    "medialibrary:allow-get-available-sources",
    "medialibrary:allow-request-permissions",
  ]
}
```

Depending on the platform, you have the following options at the moment:

### Android
- ExternalStorage
- VolumeExternalPrimary

### Linux
- PictureDir

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```javascript
import {
  getAvailableSources,
  getImages,
  GetLibraryContentRequest,
  ImageInfo,
  MediaLibrarySource,
  PermissionResponse,
  PluginError,
  requestPermissions,
  getImage,
} from "@universalappfactory/tauri-plugin-medialibrary";

// request permissions (needed for accessing the android media library)
const permissions = await requestPermissions();
if (permissions && permissions.postNotification === "denied") {
  throw new Error("Permission denied");
}

const request: GetLibraryContentRequest = {
  limit: 10,
  offset: 0,
  source: MediaLibrarySource.ExternalStorage,
};

const result = await getImages(request);
```

## Open an image with the default application

You can use the `opener` plugin to open an image with the default application:

https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/opener

It is also able to open android content uris.

## Reading an image

For reading images, you can use the tauri fs plugin:

https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/fs

E.g. in rust
```
let mut options = OpenOptions::new();
options.read(true);

#[cfg(target_os = "android")]
match self.app_handle.fs().open(path, options) {
    Ok(file) => {
      ...
    }
}
```

## Example Application

An example application is available [here](https://github.com/universalappfactory/tauri-plugin-medialibrary-example).
