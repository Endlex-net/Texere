import { EditorView, keymap } from '@codemirror/view';
import { EditorState, Compartment } from '@codemirror/state';
import { basicSetup } from 'codemirror';
import { markdown } from '@codemirror/lang-markdown';
import { vim, getCM } from '@replit/codemirror-vim';
import { indentWithTab } from '@codemirror/commands';

// Compartment for dynamic vim mode toggle
const vimCompartment = new Compartment();
const softWrapCompartment = new Compartment();
const vimModeHandlers = new WeakMap<EditorView, (event: { mode?: string }) => void>();

export interface EditorOptions {
  element: HTMLElement;
  vimEnabled?: boolean;
  softWrap?: boolean;
  initialContent?: string;
  onChange?: (content: string) => void;
  onVimModeChange?: (mode: string) => void;
}

/**
 * Create a new CodeMirror editor instance
 */
export function createEditor(options: EditorOptions): EditorView {
  const { element, vimEnabled = true, softWrap = false, initialContent = '', onChange, onVimModeChange } = options;

  // Vim extension - must be first for keystroke priority
  const vimExtension = vimEnabled ? vim() : [];
  const softWrapExtension = softWrap ? EditorView.lineWrapping : [];

  const extensions = [
    // Vim mode first!
    vimCompartment.of(vimExtension),
    softWrapCompartment.of(softWrapExtension),
    // Basic editor setup
    basicSetup,
    // Markdown highlighting
    markdown(),
    // Indent with Tab when vim is disabled
    keymap.of([indentWithTab]),
    // Editor styling
    EditorView.theme({
      '&': {
        height: '100%',
        fontSize: '14px',
        backgroundColor: 'transparent',
        color: 'var(--texere-text, #cdd6f4)',
      },
      '.cm-editor': {
        backgroundColor: 'var(--texere-editor-bg, rgba(24, 24, 37, 0.92))',
      },
      '.cm-scroller': {
        backgroundColor: 'var(--texere-editor-bg, rgba(24, 24, 37, 0.92))',
      },
      '.cm-content': {
        fontFamily: 'Menlo, Monaco, "Courier New", monospace',
        padding: '12px',
        caretColor: 'var(--texere-caret, var(--texere-text, #cdd6f4))',
      },
      '.cm-cursor, .cm-dropCursor': {
        borderLeftColor: 'var(--texere-caret, var(--texere-text, #cdd6f4)) !important',
      },
      '&.cm-focused .cm-cursor, &.cm-focused .cm-dropCursor': {
        borderLeftColor: 'var(--texere-caret, var(--texere-text, #cdd6f4)) !important',
      },
      '&.cm-focused.cm-fat-cursor .cm-cursor': {
        backgroundColor: 'var(--texere-caret, var(--texere-text, #cdd6f4)) !important',
      },
      '.cm-fat-cursor': {
        backgroundColor: 'var(--texere-caret, var(--texere-text, #cdd6f4)) !important',
      },
      '.cm-gutters': {
        backgroundColor: 'transparent',
        border: 'none',
        color: 'color-mix(in srgb, var(--texere-text, #cdd6f4) 55%, transparent)',
      },
      '.cm-activeLine': {
        backgroundColor: 'color-mix(in srgb, var(--texere-text, #cdd6f4) 10%, transparent)',
      },
      '.cm-activeLineGutter': {
        backgroundColor: 'transparent',
      },
    }),
    // Change listener
    EditorView.updateListener.of((update) => {
      if (update.docChanged && onChange) {
        onChange(update.state.doc.toString());
      }
    }),
  ];

  const state = EditorState.create({
    doc: initialContent,
    extensions,
  });

  const view = new EditorView({
    state,
    parent: element,
  });

  if (vimEnabled) {
    syncVimModeListener(view, onVimModeChange);
    const cm = getCM(view);
    onVimModeChange?.(cm?.state.vim?.mode || 'normal');
  } else {
    syncVimModeListener(view);
    onVimModeChange?.('disabled');
  }

  return view;
}


/**
 * Toggle vim mode on/off
 */
export function setVimMode(view: EditorView, enabled: boolean, onVimModeChange?: (mode: string) => void): void {
  view.dispatch({
    effects: vimCompartment.reconfigure(enabled ? vim() : []),
  });

  if (enabled) {
    syncVimModeListener(view, onVimModeChange);
    const cm = getCM(view);
    onVimModeChange?.(cm?.state.vim?.mode || 'normal');
  } else {
    syncVimModeListener(view);
    onVimModeChange?.('disabled');
  }
}

function syncVimModeListener(view: EditorView, onVimModeChange?: (mode: string) => void): void {
  const cm = getCM(view);
  const existingHandler = vimModeHandlers.get(view);

  if (cm && existingHandler) {
    cm.off('vim-mode-change', existingHandler);
  }

  if (!cm || !onVimModeChange) {
    vimModeHandlers.delete(view);
    return;
  }

  const handler = (event: { mode?: string }) => {
    onVimModeChange(event.mode || 'normal');
  };

  cm.on('vim-mode-change', handler);
  vimModeHandlers.set(view, handler);
}

/**
 * Toggle soft wrap on/off
 */
export function setSoftWrap(view: EditorView, enabled: boolean): void {
  view.dispatch({
    effects: softWrapCompartment.reconfigure(enabled ? EditorView.lineWrapping : []),
  });
}

/**
 * Get current editor content
 */
export function getContent(view: EditorView): string {
  return view.state.doc.toString();
}

/**
 * Set editor content
 */
export function setContent(view: EditorView, content: string): void {
  view.dispatch({
    changes: {
      from: 0,
      to: view.state.doc.length,
      insert: content,
    },
  });
}

/**
 * Clear editor content
 */
export function clearContent(view: EditorView): void {
  setContent(view, '');
}

/**
 * Destroy editor instance
 */
export function destroyEditor(view: EditorView): void {
  syncVimModeListener(view);
  view.destroy();
}
