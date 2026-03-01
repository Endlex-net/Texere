import { invoke } from '@tauri-apps/api/core';

export async function getCursorPosition(): Promise<[number, number] | null> {
  return invoke('get_cursor_screen_position_command');
}