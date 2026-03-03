import { beforeEach, describe, expect, it, vi } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { deleteTemplate, getTemplates, saveTemplate } from './templates';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('templates service', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it('returns empty array when get_templates fails', async () => {
    (invoke as any).mockRejectedValue(new Error('boom'));

    const result = await getTemplates();

    expect(result).toEqual([]);
    expect(invoke).toHaveBeenCalledWith('get_templates');
  });

  it('rethrows save_template errors', async () => {
    (invoke as any).mockRejectedValue(new Error('save failed'));

    await expect(
      saveTemplate({
        id: 't1',
        name: 'Template',
        content: 'x',
        createdAt: 1,
        updatedAt: 1,
      }),
    ).rejects.toThrow('save failed');
  });

  it('rethrows delete_template errors', async () => {
    (invoke as any).mockRejectedValue(new Error('delete failed'));

    await expect(deleteTemplate('t1')).rejects.toThrow('delete failed');
  });
});
