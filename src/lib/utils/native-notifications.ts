import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/plugin-notification";

let permissionResolved = false;
let permissionGranted = false;

async function ensurePermission(): Promise<boolean> {
  if (permissionResolved) return permissionGranted;

  permissionGranted = await isPermissionGranted();
  if (!permissionGranted) {
    permissionGranted = (await requestPermission()) === "granted";
  }

  permissionResolved = true;
  return permissionGranted;
}

export async function sendNativeNotification(title: string, body: string): Promise<void> {
  if (!await ensurePermission()) return;
  await sendNotification({ title, body });
}
