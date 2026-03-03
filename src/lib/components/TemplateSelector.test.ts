import { describe, expect, it, vi } from 'vitest';
import { fireEvent, render } from '@testing-library/svelte';
import TemplateSelector from './TemplateSelector.svelte';

const templates = [
  {
    id: 't1',
    name: 'Standup',
    content: 'Yesterday / Today / Blockers',
    createdAt: 1,
    updatedAt: 1,
  },
  {
    id: 't2',
    name: 'Retro',
    content: 'Went well / Improve / Action items',
    createdAt: 2,
    updatedAt: 2,
  },
];

describe('TemplateSelector', () => {
  it('calls onSelect when clicking a template item', async () => {
    const onSelect = vi.fn();
    const onClose = vi.fn();
    const { getByText } = render(TemplateSelector, { templates, onSelect, onClose });

    await fireEvent.click(getByText('Standup'));
    expect(onSelect).toHaveBeenCalledWith(expect.objectContaining({ id: 't1' }));
  });

  it('shows missing-template fallback when search has no matches', async () => {
    const { getByPlaceholderText, findByText } = render(TemplateSelector, {
      templates,
      onSelect: vi.fn(),
      onClose: vi.fn(),
    });

    const search = getByPlaceholderText('Search templates...');
    await fireEvent.input(search, { target: { value: 'xyz' } });

    expect(await findByText('No templates found matching "xyz"')).toBeInTheDocument();
  });
});
