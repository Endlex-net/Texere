import { invoke } from "@tauri-apps/api/core";

/**
 * Copy content to clipboard, close the window, and optionally auto-paste
 * into the previously focused application.
 */
export async function copyAndDismiss(
  content: string,
  autoPaste: boolean
): Promise<void> {
  await invoke("copy_and_dismiss", {
    content,
    autoPaste,
  });
}

/**
 * Close the window without copying anything.
 */
export async function dismissWithoutCopy(): Promise<void> {
  await invoke("dismiss_without_copy");
}
