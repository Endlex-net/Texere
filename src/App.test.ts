import { fireEvent, render, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const hoisted = vi.hoisted(() => {
  const currentSettings = {
    hotkeys: { summon: 'CommandOrControl+Shift+Space', copyAndDismiss: 'CommandOrControl+Enter' },
    vim: { enabled: true },
    softWrap: false,
    ai: {
      apiKey: '',
      model: 'gpt-4o-mini',
      baseUrl: 'https://api.openai.com/v1',
      systemPrompt: 'You are a text polishing assistant.',
    },
    appearance: { bgColor: '#1e1e2e', opacity: 0.95, mode: 'auto' as const, style: 'tokyo-night' as const },
    autoPaste: false,
  };

  return {
    currentSettings,
    loadSettings: vi.fn(async () => structuredClone(currentSettings)),
    saveSettings: vi.fn(),
    getTemplates: vi.fn(async () => []),
    initAppearance: vi.fn(),
    copyAndDismiss: vi.fn(),
    dismissWithoutCopy: vi.fn(),
    startDragging: vi.fn().mockResolvedValue(undefined),
    listen: vi.fn(async () => vi.fn()),
    createEditor: vi.fn(({ vimEnabled, onVimModeChange }: { vimEnabled?: boolean; onVimModeChange?: (mode: string) => void }) => {
      onVimModeChange?.(vimEnabled ? 'normal' : 'disabled');
      return {
        focus: vi.fn(),
        destroy: vi.fn(),
        state: { doc: { toString: () => '', length: 0 } },
        dispatch: vi.fn(),
      };
    }),
    setVimMode: vi.fn((_view: unknown, enabled: boolean, onVimModeChange?: (mode: string) => void) => {
      onVimModeChange?.(enabled ? 'normal' : 'disabled');
    }),
    setSoftWrap: vi.fn(),
    getContent: vi.fn(() => ''),
    setContent: vi.fn(),
    clearContent: vi.fn(),
    destroyEditor: vi.fn(),
  };
});

vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: () => ({
    startDragging: hoisted.startDragging,
    listen: hoisted.listen,
  }),
}));

vi.mock('./lib/settings', () => ({
  loadSettings: hoisted.loadSettings,
  saveSettings: hoisted.saveSettings,
}));

vi.mock('./lib/templates', () => ({
  getTemplates: hoisted.getTemplates,
}));

vi.mock('./lib/appearance', () => ({
  initAppearance: hoisted.initAppearance,
}));

vi.mock('./lib/clipboard', () => ({
  copyAndDismiss: hoisted.copyAndDismiss,
  dismissWithoutCopy: hoisted.dismissWithoutCopy,
}));

vi.mock('./lib/editor/createEditor', () => ({
  createEditor: hoisted.createEditor,
  setVimMode: hoisted.setVimMode,
  setSoftWrap: hoisted.setSoftWrap,
  getContent: hoisted.getContent,
  setContent: hoisted.setContent,
  clearContent: hoisted.clearContent,
  destroyEditor: hoisted.destroyEditor,
}));

vi.mock('./lib/components/WindowContainer.svelte', async () => {
  const mod = await import('./test/MockWindowContainer.svelte');
  return { default: mod.default };
});

import App from './App.svelte';

describe('App window-local vim toggle', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    hoisted.currentSettings.vim.enabled = true;
    hoisted.currentSettings.softWrap = false;
  });

  it('toggles vim from the status bar without persisting settings', async () => {
    const view = render(App);

    const toggle = await view.findByRole('button', { name: 'Disable Vim mode for this window' });
    expect(toggle).toHaveTextContent('NORMAL');
    expect(toggle).toHaveAttribute('tabindex', '-1');

    await fireEvent.click(toggle);

    await waitFor(() => {
      expect(view.getByRole('button', { name: 'Enable Vim mode for this window' })).toHaveTextContent('-- VIM OFF --');
    });

    expect(hoisted.saveSettings).not.toHaveBeenCalled();
  });

  it('preserves the local override after focus-triggered settings refresh', async () => {
    const view = render(App);

    await fireEvent.click(await view.findByRole('button', { name: 'Disable Vim mode for this window' }));

    hoisted.currentSettings.vim.enabled = true;
    await fireEvent.focus(window);

    await waitFor(() => {
      expect(view.getByRole('button', { name: 'Enable Vim mode for this window' })).toHaveTextContent('-- VIM OFF --');
    });
  });
});
