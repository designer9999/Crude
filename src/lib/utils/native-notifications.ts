import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/plugin-notification";

let permissionResolved = false;
let permissionGranted = false;

const INCOMING_NOTIFICATION_ID = 1001;
const INCOMING_NOTIFICATION_TAG = "landrop-incoming";
const INCOMING_NOTIFICATION_GROUP = "landrop";

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
  await sendNotification({
    id: INCOMING_NOTIFICATION_ID,
    tag: INCOMING_NOTIFICATION_TAG,
    group: INCOMING_NOTIFICATION_GROUP,
    title,
    body,
  });
}
