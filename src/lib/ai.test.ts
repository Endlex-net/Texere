import { beforeEach, describe, expect, it, vi } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { formatText } from './ai';
import {
  DEFAULT_AI_MODEL,
  DEFAULT_AI_SYSTEM_PROMPT,
  DEFAULT_OPENAI_BASE_URL,
  loadSettings,
} from './settings';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('./settings', async (importOriginal) => {
  const actual = await importOriginal<typeof import('./settings')>();
  return {
    ...actual,
    loadSettings: vi.fn(),
  };
});

describe('ai.formatText', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it('throws when API key is missing', async () => {
    (loadSettings as any).mockResolvedValue({
      ai: { apiKey: '', model: DEFAULT_AI_MODEL, baseUrl: DEFAULT_OPENAI_BASE_URL, systemPrompt: DEFAULT_AI_SYSTEM_PROMPT },
    });

    await expect(formatText('hello')).rejects.toThrow('OpenAI API key is not configured');
    expect(invoke).not.toHaveBeenCalled();
  });

  it('uses derived defaults in invoke payload when settings fields are blank', async () => {
    (loadSettings as any).mockResolvedValue({
      ai: { apiKey: 'sk-test', model: '', baseUrl: '', systemPrompt: '' },
    });
    (invoke as any).mockResolvedValue({ formatted: 'hello', style: 'informal' });

    await formatText('hello');

    expect(invoke).toHaveBeenCalledWith('format_text', {
      apiKey: 'sk-test',
      content: 'hello',
      model: DEFAULT_AI_MODEL,
      baseUrl: DEFAULT_OPENAI_BASE_URL,
      systemPrompt: DEFAULT_AI_SYSTEM_PROMPT,
    });
  });

  it('propagates invoke errors', async () => {
    (loadSettings as any).mockResolvedValue({
      ai: {
        apiKey: 'sk-test',
        model: DEFAULT_AI_MODEL,
        baseUrl: DEFAULT_OPENAI_BASE_URL,
        systemPrompt: DEFAULT_AI_SYSTEM_PROMPT,
      },
    });
    (invoke as any).mockRejectedValue(new Error('network'));

    await expect(formatText('hello')).rejects.toThrow('network');
  });
});
