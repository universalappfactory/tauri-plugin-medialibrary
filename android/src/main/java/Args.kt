package de.universalappfactory.medialibrary

import app.tauri.annotation.InvokeArg

enum class SortColumn {
    DateAdded,
    DateModified,
    DateTaken
}

enum class SortDirection {
    Ascending,
    Descending
}

@InvokeArg
class GetImagesArgs(
        val limit: Int,
        val offset: Int,
        val source: String,
        val sortColumn: SortColumn?,
        val sortDirection: SortDirection?
) {
    constructor() : this(10, 0, "", SortColumn.DateAdded, SortDirection.Ascending)
}

@InvokeArg
class GetThumbnailArgs(val uri: String) {
    constructor() : this("")
}

@InvokeArg
class RequestPermissionArgs(val source: String) {
    constructor() : this("")
}

@InvokeArg
class GetImageArgs(val uri: String) {
    constructor() : this("")
}

@InvokeArg
class DeleteImageArgs(val uri: String) {
    constructor() : this("")
}
