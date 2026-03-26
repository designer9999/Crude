export const IMAGE_EXTS = new Set([".jpg", ".jpeg", ".png", ".gif", ".webp", ".bmp", ".ico", ".svg"]);
export const VIDEO_EXTS = new Set([".mp4", ".webm", ".mov", ".avi", ".mkv", ".m4v", ".ogv"]);

export function isImage(nameOrExt: string): boolean {
  const ext = nameOrExt.includes(".") ? nameOrExt.slice(nameOrExt.lastIndexOf(".")).toLowerCase() : nameOrExt;
  return IMAGE_EXTS.has(ext);
}

export function isVideo(nameOrExt: string): boolean {
  const ext = nameOrExt.includes(".") ? nameOrExt.slice(nameOrExt.lastIndexOf(".")).toLowerCase() : nameOrExt;
  return VIDEO_EXTS.has(ext);
}

export function fileSizeStr(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}
