import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import SettingsPanel from './SettingsPanel.svelte';
import * as settingsMod from '../settings';
import * as templatesMod from '../templates';

vi.mock('../appearance', () => ({
  applyAppearance: vi.fn(),
}));

vi.mock('../settings', () => ({
  loadSettings: vi.fn(),
  saveSettings: vi.fn(),
  canEnableAutoPaste: vi.fn().mockResolvedValue(true),
  DEFAULT_AI_MODEL: 'gpt-4o-mini',
  DEFAULT_AI_SYSTEM_PROMPT: 'You are a text polishing assistant.',
  DEFAULT_OPENAI_BASE_URL: 'https://api.openai.com/v1',
}));

vi.mock('../templates', () => ({
  getTemplates: vi.fn(),
  saveTemplate: vi.fn(),
  deleteTemplate: vi.fn(),
}));

const baseSettings = {
  hotkeys: { summon: 'CommandOrControl+Shift+Space', copyAndDismiss: 'CommandOrControl+Enter' },
  vim: { enabled: true },
  softWrap: false,
  ai: {
    apiKey: '',
    model: 'gpt-4o-mini',
    baseUrl: 'https://api.openai.com/v1',
    systemPrompt: 'You are a text polishing assistant.',
  },
  appearance: { bgColor: '#1e1e2e', opacity: 0.95, mode: 'auto', style: 'tokyo-night' },
  autoPaste: false,
};

describe('SettingsPanel soft wrap e2e', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    (settingsMod.loadSettings as any).mockResolvedValue(baseSettings);
    (settingsMod.saveSettings as any).mockResolvedValue(undefined);
    (templatesMod.getTemplates as any).mockResolvedValue([]);
  });

  it('toggles soft-wrap and persists through save payload', async () => {
    const { findByText, getByLabelText } = render(SettingsPanel);
    await findByText('Editor');

    const softWrapToggle = getByLabelText('Soft Wrap (display only)') as HTMLInputElement;
    expect(softWrapToggle.checked).toBe(false);

    await fireEvent.click(softWrapToggle);
    await new Promise((r) => setTimeout(r, 600));

    expect(settingsMod.saveSettings).toHaveBeenCalled();
    const lastCall = (settingsMod.saveSettings as any).mock.calls.at(-1)?.[0];
    expect(lastCall.softWrap).toBe(true);
  });

  it('reloads with soft-wrap enabled after restart-like re-open', async () => {
    (settingsMod.loadSettings as any)
      .mockResolvedValueOnce(baseSettings)
      .mockResolvedValueOnce({ ...baseSettings, softWrap: true });

    const first = render(SettingsPanel);
    await first.findByText('Editor');
    first.unmount();

    const second = render(SettingsPanel);
    await second.findByText('Editor');
    const softWrapToggle = second.getByLabelText('Soft Wrap (display only)') as HTMLInputElement;
    expect(softWrapToggle.checked).toBe(true);
  });
});
