import { beforeEach, describe, expect, it, vi } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { getCursorPosition } from './cursorPosition';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('cursorPosition', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it('returns cursor tuple from backend', async () => {
    (invoke as any).mockResolvedValue([120, 340]);

    const result = await getCursorPosition();

    expect(invoke).toHaveBeenCalledWith('get_cursor_screen_position_command');
    expect(result).toEqual([120, 340]);
  });

  it('propagates backend errors', async () => {
    (invoke as any).mockRejectedValue(new Error('permission denied'));

    await expect(getCursorPosition()).rejects.toThrow('permission denied');
  });
});
