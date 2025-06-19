package de.universalappfactory.medialibrary

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.net.Uri
import android.os.Build
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.Permission
import app.tauri.annotation.PermissionCallback
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class GetImagesArgs(val limit: Int, val offset: Int, val source: String) {
    constructor() : this(10, 0, "")
}

@InvokeArg
class GetThumbnailArgs(val uri: String) {
    constructor() : this("")
}

@InvokeArg
class RequestPermissionArgs(val source: String) {
    constructor() : this("")
}

private const val EXTERNAL_STORAGE_ALIAS = "externalStorage"
private const val WRITE_EXTERNAL_STORAGE_ALIAS = "writeExternalStorage"
private const val MEDIA_IMAGES_ALIAS = "readMediaImages"

@TauriPlugin(
        permissions =
                [
                        Permission(
                                strings = [Manifest.permission.READ_EXTERNAL_STORAGE],
                                alias = EXTERNAL_STORAGE_ALIAS
                        ),
                        Permission(
                                strings = [Manifest.permission.WRITE_EXTERNAL_STORAGE],
                                alias = WRITE_EXTERNAL_STORAGE_ALIAS
                        ),
                        Permission(
                                strings = ["android.permission.READ_MEDIA_IMAGES"],
                                alias = MEDIA_IMAGES_ALIAS
                        )]
)
class MediaLibraryPlugin(private val activity: Activity) : Plugin(activity) {

    private var requestPermissionResponse: JSObject? = null

    @Command
    fun getImages(invoke: Invoke) {
        val args = invoke.parseArgs(GetImagesArgs::class.java)

        // requestPermissions(invoke)
        val mediaLibaray = MediaLibrary(activity.contentResolver)

        val ret = JSObject()
        ret.put("items", JSArray(mediaLibaray.getAllImages(args.limit, args.offset, args.source)))
        invoke.resolve(ret)
    }

    @Command
    fun getThumbnailAsBase64(invoke: Invoke) {
        val args = invoke.parseArgs(GetThumbnailArgs::class.java)

        // requestPermissions(invoke)
        val mediaLibaray = MediaLibrary(activity.contentResolver)

        val uri = Uri.parse(args.uri)
        val content = mediaLibaray.getThumbnailAsBase64(uri)

        val ret = JSObject()
        ret.put("content", content ?: "")
        invoke.resolve(ret)
    }

    @PermissionCallback
    private fun storagePermissionCallback(invoke: Invoke) {
        val permissionsResultJSON = JSObject()
        permissionsResultJSON.put("postNotification", getPermissionState(EXTERNAL_STORAGE_ALIAS))
        invoke.resolve(permissionsResultJSON)
    }

    @SuppressLint("ObsoleteSdkInt")
    @Command
    override fun requestPermissions(invoke: Invoke) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            requestPermissionForAlias(EXTERNAL_STORAGE_ALIAS, invoke, "storagePermissionCallback")
        } else {
            requestPermissionForAlias(MEDIA_IMAGES_ALIAS, invoke, "storagePermissionCallback")
        }
    }
}
