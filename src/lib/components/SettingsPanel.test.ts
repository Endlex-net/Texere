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

const baseTemplate = {
  id: 'tpl-1',
  name: 'Daily Notes',
  content: 'Today I worked on...',
  createdAt: 1700000000000,
  updatedAt: 1700000000000,
};

describe('SettingsPanel', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    (settingsMod.loadSettings as any).mockResolvedValue({
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
      autoPaste: true
    });
    (templatesMod.getTemplates as any).mockResolvedValue([]);
    (templatesMod.saveTemplate as any).mockResolvedValue(undefined);
    (templatesMod.deleteTemplate as any).mockResolvedValue(undefined);
  });

  it('renders settings panel and loads settings', async () => {
    const { getByText, findByText } = render(SettingsPanel);
    
    // initially shows loading
    expect(getByText('Loading settings...')).toBeInTheDocument();
    
    // then shows settings
    await findByText('Hotkeys');
    expect(getByText('Editor')).toBeInTheDocument();
    expect(getByText('AI Assistant')).toBeInTheDocument();
    expect(getByText('Appearance')).toBeInTheDocument();
    expect(getByText('Auto-paste')).toBeInTheDocument();
  });

  it('saves settings on change', async () => {
    const { getByLabelText, findByText } = render(SettingsPanel);
    
    await findByText('Hotkeys');
    
    // Change a setting
    const vimToggle = getByLabelText('Vim Mode').nextElementSibling?.querySelector('input');
    if (vimToggle) {
        await fireEvent.click(vimToggle);
    }
    
    // Wait for debounce
    await new Promise(r => setTimeout(r, 600));
    
    expect(settingsMod.saveSettings).toHaveBeenCalled();
  });

  it('deletes a template item through settings panel flow', async () => {
    (templatesMod.getTemplates as any)
      .mockResolvedValueOnce([baseTemplate])
      .mockResolvedValueOnce([]);

    const { findByText, getByLabelText, getByRole, queryByText } = render(SettingsPanel);

    await findByText('Templates');
    await findByText('Daily Notes');

    await fireEvent.click(getByLabelText('Delete'));
    const dialog = getByRole('alertdialog');
    const confirmButton = dialog.querySelector('.btn-danger') as HTMLButtonElement | null;
    expect(confirmButton).not.toBeNull();
    if (confirmButton) {
      await fireEvent.click(confirmButton);
    }

    expect(templatesMod.deleteTemplate).toHaveBeenCalledWith('tpl-1');
    expect(await findByText('Saved')).toBeInTheDocument();
    expect(queryByText('Daily Notes')).not.toBeInTheDocument();
  });
});
