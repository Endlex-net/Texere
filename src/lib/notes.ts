import { invoke } from "@tauri-apps/api/core";
import type { Note } from "./types";

export async function getNotes(): Promise<Note[]> {
  try {
    return await invoke<Note[]>("get_notes");
  } catch (error) {
    console.error("Failed to load notes:", error);
    return [];
  }
}

/**
 * Upsert a note. Pass `id: ""` to create a new note (Rust will generate the UUID).
 * Returns the saved note (including the generated id on creation).
 *
 * Pass `windowLabel` when naming a fresh temporary window so the note→window mapping
 * is registered atomically inside Rust before the tray entry becomes visible (C3).
 */
export async function saveNote(
  note: Partial<Note> & { name: string; content: string },
  windowLabel?: string,
): Promise<Note> {
  const payload: Note = {
    id: note.id ?? "",
    name: note.name,
    content: note.content,
    createdAt: note.createdAt ?? 0,
    updatedAt: note.updatedAt ?? 0,
  };
  return await invoke<Note>("save_note", {
    note: payload,
    windowLabel: windowLabel ?? null,
  });
}

export async function deleteNote(id: string): Promise<void> {
  await invoke("delete_note", { id });
}
