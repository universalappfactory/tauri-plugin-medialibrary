import { invoke } from "@tauri-apps/api/core";

export type ErrorKind = "XdgPicturesDirNotSet";

type InternalPluginError = {
  kind: ErrorKind;
  message: string;
};

export class PluginError extends Error {
  errorKind: string;
  constructor(error: InternalPluginError) {
    super(error.message);
    this.name = "CustomError";
    this.errorKind = error.kind;
  }
}

function isInternalError(input: unknown): input is InternalPluginError {
  const err = input as InternalPluginError;
  return err.kind !== undefined && err.message !== undefined;
}

async function invokeCommand<T>(func: () => Promise<T>): Promise<T> {
  try {
    const result = await func();
    return result;
  } catch (error) {
    console.error(error);
    if (typeof error === "string") {
      throw new PluginError({ kind: error as ErrorKind, message: "" });
    }
    if (isInternalError(error)) {
      throw new PluginError(error);
    }
    throw error;
  }
}

export interface ImageInfo {
  path: string;
  contentUri: string;
  mimeType: string;
  metaData?: Record<string, string>;
}

export interface GetImagesResult {
  items: ImageInfo[];
}

export interface GetThumbnailResponse {
  content: string;
}

export interface GetLibraryContentRequest {
  limit: number;
  offset: number;
  source: MediaLibrarySource;
  sortColumn?: SortColumn;
  sortDirection?: SortDirection;
}

export interface GetPermissionsRequest {
  source: MediaLibrarySource;
}

export type PermissionState =
  | "granted"
  | "denied"
  | "prompt"
  | "prompt-with-rationale";

export interface PermissionResponse {
  postNotification: PermissionState;
}

export async function getImages(
  request: GetLibraryContentRequest,
): Promise<GetImagesResult | null> {
  return await invokeCommand<GetImagesResult | null>(async () => {
    return await invoke("plugin:medialibrary|get_images", {
      request: request,
    });
  });
}

export async function getImage(contentUri: string): Promise<ImageInfo | null> {
  return await invokeCommand<ImageInfo | null>(async () => {
    return await invoke("plugin:medialibrary|get_image", {
      uri: contentUri,
    });
  });
}

export async function getThumbnail(
  uri: string,
): Promise<GetThumbnailResponse | null> {
  const result = await invoke("plugin:medialibrary|get_thumbnail", {
    uri: uri,
  });
  return result as GetThumbnailResponse;
}

export enum MediaLibrarySource {
  ExternalStorage = "ExternalStorage",
  VolumeExternalPrimary = "VolumeExternalPrimary",
  PictureDir = "PictureDir",
}

export enum SortDirection {
  Ascending = "Ascending",
  Descending = "Descending",
}

export enum SortColumn {
  DateAdded = "DateAdded",
  DateModified = "DateModified",
  DateTaken = "DateTaken",
}

export async function getAvailableSources(): Promise<
  MediaLibrarySource[] | null
> {
  const result = await invoke("plugin:medialibrary|get_available_sources");
  return result as MediaLibrarySource[];
}

export async function requestPermissions(
  request: GetPermissionsRequest,
): Promise<PermissionResponse | null> {
  const result = await invoke("plugin:medialibrary|request_permissions", {
    args: request,
  });
  return result as PermissionResponse;
}
