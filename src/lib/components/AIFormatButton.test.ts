import { beforeEach, describe, expect, it, vi } from 'vitest';
import { fireEvent, render, waitFor } from '@testing-library/svelte';
import AIFormatButton from './AIFormatButton.svelte';
import { formatText } from '../ai';
import { loadSettings } from '../settings';

vi.mock('../ai', () => ({
  formatText: vi.fn(),
}));

vi.mock('../settings', () => ({
  loadSettings: vi.fn(),
}));

describe('AIFormatButton', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    if (!(Element.prototype as any).animate) {
      (Element.prototype as any).animate = vi.fn(() => ({
        finished: Promise.resolve(),
        cancel: vi.fn(),
      }));
    }
  });

  it('stays disabled when API key is missing', async () => {
    (loadSettings as any).mockResolvedValue({ ai: { apiKey: '' } });

    const { getByRole } = render(AIFormatButton, {
      getContent: () => 'hello',
      setContent: vi.fn(),
      hasContent: true,
    });

    const button = getByRole('button', { name: 'OpenAI API key required' }) as HTMLButtonElement;
    await waitFor(() => expect(button.disabled).toBe(true));
  });

  it('shows loading state while formatting and updates content on success', async () => {
    (loadSettings as any).mockResolvedValue({ ai: { apiKey: 'sk-test' } });

    let resolveFormat: (value: { formatted: string; style: string }) => void = () => {};
    const pending = new Promise<{ formatted: string; style: string }>((resolve) => {
      resolveFormat = resolve;
    });
    (formatText as any).mockReturnValue(pending);

    const setContent = vi.fn();
    const { findByRole, container } = render(AIFormatButton, {
      getContent: () => 'raw text',
      setContent,
      hasContent: true,
    });

    const button = (await findByRole('button', { name: 'Format with AI' })) as HTMLButtonElement;
    await fireEvent.click(button);

    expect(button.disabled).toBe(true);
    expect(container.querySelector('.spinner')).not.toBeNull();

    resolveFormat({ formatted: 'polished text', style: 'informal' });
    await waitFor(() => expect(setContent).toHaveBeenCalledWith('polished text'));
  });

  it('shows error toast when formatting fails', async () => {
    (loadSettings as any).mockResolvedValue({ ai: { apiKey: 'sk-test' } });
    (formatText as any).mockRejectedValue(new Error('format failed'));

    const { findByRole, findByText } = render(AIFormatButton, {
      getContent: () => 'raw text',
      setContent: vi.fn(),
      hasContent: true,
    });

    const button = (await findByRole('button', { name: 'Format with AI' })) as HTMLButtonElement;
    await fireEvent.click(button);

    expect(await findByText('format failed')).toBeInTheDocument();
  });
});
