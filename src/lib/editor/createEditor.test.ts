import { beforeEach, describe, expect, it, vi } from 'vitest';

const hoisted = vi.hoisted(() => ({
  updateListener: null as null | ((update: { docChanged: boolean; state: { doc: { toString: () => string } } }) => void),
  modeListener: null as null | ((event: { mode: string }) => void),
  dispatchCalls: [] as any[],
}));

vi.mock('@codemirror/state', () => {
  class Compartment {
    of(value: unknown) {
      return { kind: 'of', value };
    }
    reconfigure(value: unknown) {
      return { kind: 'reconfigure', value };
    }
  }

  const EditorState = {
    create: ({ doc, extensions }: { doc: string; extensions: unknown[] }) => ({
      doc: {
        value: doc,
        length: doc.length,
        toString() {
          return this.value;
        },
      },
      extensions,
    }),
  };

  const Prec = {
    highest: (value: unknown) => value,
  };

  return { Compartment, EditorState, Prec };
});

vi.mock('@codemirror/view', () => {
  class EditorView {
    state: any;
    parent: HTMLElement;
    constructor({ state, parent }: { state: any; parent: HTMLElement }) {
      this.state = state;
      this.parent = parent;
    }
    dispatch(payload: any) {
      hoisted.dispatchCalls.push(payload);
      if (payload?.changes) {
        this.state.doc.value = payload.changes.insert;
        this.state.doc.length = payload.changes.insert.length;
      }
    }
    focus() {}
    destroy() {}
  }

  (EditorView as any).theme = vi.fn(() => ({ kind: 'theme' }));
  (EditorView as any).lineWrapping = { kind: 'line-wrapping' };
  (EditorView as any).updateListener = {
    of: (callback: any) => {
      hoisted.updateListener = callback;
      return { kind: 'listener' };
    },
  };

  return { EditorView, keymap: {} };
});

vi.mock('codemirror', () => ({ basicSetup: { kind: 'basic-setup' } }));
vi.mock('@codemirror/lang-markdown', () => ({ markdown: () => ({ kind: 'markdown' }) }));
vi.mock('@replit/codemirror-vim', () => ({
  vim: () => ({ kind: 'vim' }),
  getCM: () => ({
    state: { vim: { mode: 'normal' } },
    on: (_event: string, cb: (event: { mode: string }) => void) => {
      hoisted.modeListener = cb;
    },
  }),
}));

import { clearContent, createEditor, getContent, setContent, setSoftWrap, setVimMode } from './createEditor';

describe('createEditor helpers', () => {
  beforeEach(() => {
    hoisted.updateListener = null;
    hoisted.modeListener = null;
    hoisted.dispatchCalls = [];
  });

  it('loads initial content', () => {
    const element = document.createElement('div');
    const view = createEditor({ element, initialContent: 'hello' });

    expect(getContent(view)).toBe('hello');
  });

  it('emits change callback when update listener receives docChanged', () => {
    const onChange = vi.fn();
    const element = document.createElement('div');
    createEditor({ element, onChange });

    expect(hoisted.updateListener).not.toBeNull();
    hoisted.updateListener?.({
      docChanged: true,
      state: { doc: { toString: () => 'next value' } },
    });

    expect(onChange).toHaveBeenCalledWith('next value');
  });

  it('toggles vim mode and reports disabled/enabled modes', () => {
    const onMode = vi.fn();
    const element = document.createElement('div');
    const view = createEditor({ element, vimEnabled: true, onVimModeChange: onMode });

    setVimMode(view, false, onMode);
    expect(onMode).toHaveBeenCalledWith('disabled');

    setVimMode(view, true, onMode);
    expect(onMode).toHaveBeenCalledWith('normal');
  });

  it('supports setContent and clearContent helpers', () => {
    const element = document.createElement('div');
    const view = createEditor({ element, initialContent: 'start' });

    setContent(view, 'changed');
    expect(getContent(view)).toBe('changed');

    clearContent(view);
    expect(getContent(view)).toBe('');
  });

  it('supports soft wrap reconfiguration', () => {
    const element = document.createElement('div');
    const view = createEditor({ element, softWrap: false });
    const before = getContent(view);

    setSoftWrap(view, true);
    const after = getContent(view);

    expect(hoisted.dispatchCalls.length).toBeGreaterThan(0);
    expect(hoisted.dispatchCalls[0]).toEqual(
      expect.objectContaining({
        effects: expect.objectContaining({ kind: 'reconfigure', value: { kind: 'line-wrapping' } }),
      }),
    );
    expect(after).toBe(before);
  });
});
