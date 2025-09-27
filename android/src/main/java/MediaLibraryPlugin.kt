package de.universalappfactory.medialibrary

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.app.RecoverableSecurityException
import android.net.Uri
import android.os.Build
import android.provider.MediaStore
import androidx.activity.result.ActivityResult
import androidx.activity.result.IntentSenderRequest
import app.tauri.annotation.ActivityCallback
import app.tauri.annotation.Command
import app.tauri.annotation.Permission
import app.tauri.annotation.PermissionCallback
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

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
    private var deleteRequestInvoke: Invoke? = null

    companion object {
        private const val DELETE_REQUEST_CODE = 1001
        private const val RECOVERABLE_DELETE_REQUEST_CODE = 1002
    }

    @Command
    fun getImages(invoke: Invoke) {
        val args = invoke.parseArgs(GetImagesArgs::class.java)

        val mediaLibaray = MediaLibrary(activity.contentResolver, activity)

        val ret = JSObject()
        ret.put("items", JSArray(mediaLibaray.getAllImages(args)))
        invoke.resolve(ret)
    }

    @Command
    fun getImage(invoke: Invoke) {
        val args = invoke.parseArgs(GetImageArgs::class.java)

        val mediaLibaray = MediaLibrary(activity.contentResolver, activity)
        val ret = mediaLibaray.getImage(args.uri)
        invoke.resolve(ret)
    }

    @Command
    fun executeRecoverableDeleteRequest(invoke: Invoke) {
        val args = invoke.parseArgs(DeleteImageArgs::class.java)
        try {
            val uri = Uri.parse(args.uri)

            try {
                val mediaLibaray = MediaLibrary(activity.contentResolver, activity)
                val result = mediaLibaray.deleteImage(args.uri)
                invoke.resolve(result)
            } catch (securityException: RecoverableSecurityException) {
                val urisToDelete = listOf(uri)
                val pendingIntent =
                        MediaStore.createDeleteRequest(activity.contentResolver, urisToDelete)

                val request = IntentSenderRequest.Builder(pendingIntent.getIntentSender()).build()
                startIntentSenderForResult(invoke, request, "deleteActivityResult")
            }
        } catch (e: Exception) {
            invoke.reject("Failed to handle recoverable delete request: ${e.message}")
        }
    }

    @ActivityCallback
    fun deleteActivityResult(invoke: Invoke, result: ActivityResult) {
        val resultCode = result.resultCode
        if (resultCode == Activity.RESULT_CANCELED) {
            invoke.reject(
                    "deleteActivityResult canceled",
            )
        } else {
            invoke.resolve()
        }
    }

    @Command
    fun getThumbnailAsBase64(invoke: Invoke) {
        val args = invoke.parseArgs(GetThumbnailArgs::class.java)

        // requestPermissions(invoke)
        val mediaLibaray = MediaLibrary(activity.contentResolver, activity)

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
