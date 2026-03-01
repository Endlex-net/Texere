import { EditorView, keymap } from '@codemirror/view';
import { EditorState, Compartment } from '@codemirror/state';
import { basicSetup } from 'codemirror';
import { markdown } from '@codemirror/lang-markdown';
import { vim, getCM } from '@replit/codemirror-vim';

// Compartment for dynamic vim mode toggle
const vimCompartment = new Compartment();

export interface EditorOptions {
  element: HTMLElement;
  vimEnabled?: boolean;
  initialContent?: string;
  onChange?: (content: string) => void;
  onVimModeChange?: (mode: string) => void;
}

/**
 * Create a new CodeMirror editor instance
 */
export function createEditor(options: EditorOptions): EditorView {
  const { element, vimEnabled = true, initialContent = '', onChange, onVimModeChange } = options;

  // Vim extension - must be first for keystroke priority
  const vimExtension = vimEnabled ? vim() : [];

  const extensions = [
    // Vim mode first!
    vimCompartment.of(vimExtension),
    // Basic editor setup
    basicSetup,
    // Markdown highlighting
    markdown(),
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
        caretColor: 'var(--texere-text, #cdd6f4)',
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
    const cm = getCM(view);
    if (cm) {
      if (onVimModeChange) onVimModeChange(cm.state.vim?.mode || 'normal');
      cm.on('vim-mode-change', (e: any) => onVimModeChange?.(e.mode));
    }
  } else {
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
    const cm = getCM(view);
    if (cm) {
      if (onVimModeChange) onVimModeChange(cm.state.vim?.mode || 'normal');
      cm.on('vim-mode-change', (e: any) => onVimModeChange?.(e.mode));
    }
  } else {
    onVimModeChange?.('disabled');
  }
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
  view.destroy();
}
