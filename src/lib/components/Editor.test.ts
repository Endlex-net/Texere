import { beforeEach, describe, expect, it, vi } from 'vitest';
import { render, waitFor } from '@testing-library/svelte';
import Editor from './Editor.svelte';

const mocked = vi.hoisted(() => {
  const view = { id: 'mock-view', focus: vi.fn() } as any;
  let content = '';
  return {
    view,
    getContentValue: () => content,
    setContentValue: (value: string) => {
      content = value;
    },
    createEditor: vi.fn(({ initialContent }: { initialContent?: string }) => {
      content = initialContent ?? '';
      return view;
    }),
    setVimMode: vi.fn(),
    setSoftWrap: vi.fn(),
    getContent: vi.fn(() => content),
    setContent: vi.fn((_view: unknown, value: string) => {
      content = value;
    }),
    clearContent: vi.fn(() => {
      content = '';
    }),
    destroyEditor: vi.fn(),
  };
});

vi.mock('../editor/createEditor', () => ({
  createEditor: mocked.createEditor,
  setVimMode: mocked.setVimMode,
  setSoftWrap: mocked.setSoftWrap,
  getContent: mocked.getContent,
  setContent: mocked.setContent,
  clearContent: mocked.clearContent,
  destroyEditor: mocked.destroyEditor,
}));

describe('Editor component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocked.setContentValue('');
  });

  it('mounts editor and supports content roundtrip via exposed methods', async () => {
    const { component } = render(Editor, {
      vimEnabled: true,
      softWrap: false,
      initialContent: 'hello',
    });

    expect(mocked.createEditor).toHaveBeenCalledWith(
      expect.objectContaining({ initialContent: 'hello', vimEnabled: true, softWrap: false }),
    );

    expect((component as any).getValue()).toBe('hello');
    (component as any).setValue('updated');

    await waitFor(() => expect((component as any).getValue()).toBe('updated'));

    (component as any).clear();
    await waitFor(() => expect((component as any).getValue()).toBe(''));
  });

  it('applies vim mode changes and cleans up on unmount', async () => {
    const view = render(Editor, {
      vimEnabled: true,
      softWrap: false,
      initialContent: 'x',
    });

    await view.rerender({ vimEnabled: false, softWrap: true, initialContent: 'x' });
    expect(mocked.setVimMode).toHaveBeenCalled();
    expect(mocked.setSoftWrap).toHaveBeenCalled();

    view.unmount();
    expect(mocked.destroyEditor).toHaveBeenCalledWith(mocked.view);
  });
});
