import { invoke } from "@tauri-apps/api/core";

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value?: string }>(
    "plugin:androidpermissionprompt|ping",
    {
      payload: {
        value,
      },
    }
  ).then((r) => (r.value ? r.value : null));
}

export async function requestStoragePermission() {
  try {
    const granted = await invoke<boolean>("request_storage_permission");
    if (granted) {
      return "Storage permission granted";
    } else {
      return "Storage permission denied";
    }
  } catch (error) {
    return `Error requesting storage permission: ${error}`;
  }
}
