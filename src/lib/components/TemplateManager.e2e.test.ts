import { describe, it, expect } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import TemplateManagerHarness from './TemplateManagerHarness.svelte';
import type { Template } from '../types';

const baseTemplate: Template = {
  id: 'tpl-1',
  name: 'Daily Notes',
  content: 'Today I worked on...',
  createdAt: 1700000000000,
  updatedAt: 1700000000000,
};

describe('TemplateManager e2e', () => {
  it('creates a template via UI flow', async () => {
    const { getByText, getByPlaceholderText, getByTestId } = render(TemplateManagerHarness, {
      templates: [],
    });

    await fireEvent.click(getByText('+ New Template'));
    await fireEvent.input(getByPlaceholderText('Template name...'), { target: { value: 'Standup' } });
    await fireEvent.input(getByPlaceholderText('Template content...'), { target: { value: 'Yesterday / Today / Blockers' } });
    await fireEvent.click(getByText('Save'));

    const payload = JSON.parse(getByTestId('last-create').textContent ?? '{}');
    expect(payload.name).toBe('Standup');
    expect(payload.content).toBe('Yesterday / Today / Blockers');
  });

  it('deletes a template from the list when clicking delete', async () => {
    const { getByLabelText, getByText, getByTestId } = render(TemplateManagerHarness, {
      templates: [baseTemplate],
    });

    await fireEvent.click(getByLabelText('Delete'));
    await fireEvent.click(getByText('Delete'));

    expect(getByTestId('last-delete').textContent).toBe('tpl-1');
  });

  it('closes confirmation on Escape without deleting', async () => {
    const { getByLabelText, getByRole, getByTestId } = render(TemplateManagerHarness, {
      templates: [baseTemplate],
    });

    await fireEvent.click(getByLabelText('Delete'));
    await fireEvent.keyDown(getByRole('alertdialog'), { key: 'Escape' });

    expect(getByTestId('last-delete').textContent).toBe('');
  });
});
