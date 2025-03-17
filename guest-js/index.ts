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

export async function requestAudioPermission() {
  try {
    const granted = await invoke<boolean>("request_read_audio_permission");
    if (granted) {
      return "Audio permission granted";
    } else {
      return "Audio permission denied";
    }
  } catch (error) {
    return `Error requesting Audio permission: ${error}`;
  }
}
