import { invoke } from "@tauri-apps/api/core";

export async function processEvent(
  event: Uint8Array
): Promise<Uint8Array | null> {
  return await invoke<Uint8Array | null >(
    "plugin:crux|process_event",
    event
  );
}

export async function view(
): Promise<Uint8Array | null> {
  return await invoke<Uint8Array | null >(
    "plugin:crux|view"
  );
}
