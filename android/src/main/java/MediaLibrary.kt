package de.universalappfactory.medialibrary

import android.content.ContentResolver
import android.content.ContentUris
import android.database.Cursor
import android.graphics.Bitmap
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.provider.MediaStore
import android.util.Base64
import android.util.Log
import android.util.Size
import app.tauri.plugin.JSObject
import java.io.ByteArrayOutputStream

// Available media library sources
// @see https://developer.android.com/training/data-storage/shared/media
enum class MediaLibrarySource {
    ExternalStorage,
    VolumeExternalPrimary
}

class MediaLibrary(private val contentResolver: ContentResolver) {

    companion object {
        private const val TAG = "MediaLibrary"
        private val THUMBNAIL_SIZE = Size(200, 200)
    }

    private fun getImageSource(imageSource: String): Uri? {
        val source = MediaLibrarySource.valueOf(imageSource)
        return if (Build.VERSION.SDK_INT > Build.VERSION_CODES.Q) {
            when (source) {
                MediaLibrarySource.ExternalStorage ->
                        MediaStore.Images.Media.getContentUri(MediaStore.VOLUME_EXTERNAL)
                MediaLibrarySource.VolumeExternalPrimary ->
                        MediaStore.Images.Media.getContentUri(MediaStore.VOLUME_EXTERNAL_PRIMARY)
            }
        } else {
            // Both enum values map to the same URI on older versions
            MediaStore.Images.Media.EXTERNAL_CONTENT_URI
        }
    }

    fun getQuery(limit: Int, offset: Int, imageSource: String): Cursor? {
        val projection =
                arrayOf(
                        if (Build.VERSION.SDK_INT > Build.VERSION_CODES.Q)
                                MediaStore.Images.Media.RELATIVE_PATH
                        else MediaStore.Images.Media.DATA,
                        MediaStore.Images.Media._ID,
                        MediaStore.Images.Media.MIME_TYPE,
                        MediaStore.Images.ImageColumns.DATE_TAKEN,
                )

        val imageCollection = getImageSource(imageSource) ?: return null

        // https://developer.android.com/reference/android/content/ContentProvider#query(android.net.Uri,%20java.lang.String[],%20android.os.Bundle,%20android.os.CancellationSignal)
        // Not sure which api level is correct here, query using bundle is added in 26 but limit and
        // offset does not seem to work
        // In higher version (35 in our tests) it worked
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            val queryArgs =
                    Bundle().apply {
                        putStringArray(
                                ContentResolver.QUERY_ARG_SORT_COLUMNS,
                                arrayOf(MediaStore.Images.Media.DATE_TAKEN)
                        )

                        putInt(
                                ContentResolver.QUERY_ARG_SORT_DIRECTION,
                                ContentResolver.QUERY_SORT_DIRECTION_DESCENDING
                        )

                        putInt(ContentResolver.QUERY_ARG_LIMIT, limit)
                        putInt(ContentResolver.QUERY_ARG_OFFSET, offset)
                    }

            contentResolver.query(
                    imageCollection,
                    projection,
                    queryArgs,
                    null,
            )
        } else {
            val sort = MediaStore.Images.ImageColumns.DATE_TAKEN
            contentResolver.query(
                    imageCollection,
                    projection,
                    null,
                    null,
                    "$sort DESC LIMIT $limit OFFSET $offset"
            )
        }
    }

    fun getThumbnail(uri: Uri): Bitmap? =
            try {
                contentResolver.loadThumbnail(uri, THUMBNAIL_SIZE, null)
            } catch (e: Exception) {
                Log.e(TAG, "Failed to get thumbnail for URI: $uri", e)
                null
            }

    fun getThumbnailAsBase64(uri: Uri): String? =
            getThumbnail(uri)?.let { thumbnail ->
                ByteArrayOutputStream().use { outputStream ->
                    thumbnail.compress(Bitmap.CompressFormat.JPEG, 80, outputStream)
                    Base64.encodeToString(outputStream.toByteArray(), Base64.DEFAULT)
                }
            }

    private fun createImageJSObjectFromCursor(cursor: Cursor): JSObject {
        val idIndex = cursor.getColumnIndexOrThrow(MediaStore.Images.Media._ID)
        val dataColumnIndex =
                cursor.getColumnIndexOrThrow(
                        if (Build.VERSION.SDK_INT > Build.VERSION_CODES.Q)
                                MediaStore.Images.Media.RELATIVE_PATH
                        else MediaStore.Images.Media.DATA
                )
        val mimeTypeColumnIndex = cursor.getColumnIndexOrThrow(MediaStore.Images.Media.MIME_TYPE)

        val ret = JSObject()
        val imageId = cursor.getLong(idIndex)
        val imagePath = cursor.getString(dataColumnIndex)
        val mimeType = cursor.getString(mimeTypeColumnIndex)

        val contentUri =
                ContentUris.withAppendedId(MediaStore.Images.Media.EXTERNAL_CONTENT_URI, imageId)

        ret.put("path", imagePath)
        ret.put("contentUri", contentUri.toString())
        ret.put("mimeType", mimeType)

        return ret
    }

    fun getAllImages(limit: Int, offset: Int, imageSource: String): List<JSObject> {
        val imageList = mutableListOf<JSObject>()

        getQuery(limit, offset, imageSource)?.use { cursor ->
            while (cursor.moveToNext()) {
                val ret = createImageJSObjectFromCursor(cursor)
                imageList.add(ret)
            }
        }
        return imageList
    }

    fun getImage(contentUriString: String): JSObject? {
        try {
            val uri = Uri.parse(contentUriString)
            val projection =
                    arrayOf(
                            if (Build.VERSION.SDK_INT > Build.VERSION_CODES.Q)
                                    MediaStore.Images.Media.RELATIVE_PATH
                            else MediaStore.Images.Media.DATA,
                            MediaStore.Images.Media._ID,
                            MediaStore.Images.Media.MIME_TYPE,
                            MediaStore.Images.ImageColumns.DATE_TAKEN,
                    )

            contentResolver.query(uri, projection, null, null, null)?.use { cursor ->
                if (cursor.moveToFirst()) {
                    val ret = createImageJSObjectFromCursor(cursor)
                    return ret
                }
            }
        } catch (e: Exception) {
            Log.e(TAG, "Failed to get image info for URI: $contentUriString", e)
        }
        return null
    }
}
