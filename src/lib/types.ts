/**
 * Concrete theme palette identifiers
 * Used internally after resolving (mode, style) → theme
 */
export type ThemeId =
  | 'catppuccin-mocha'
  | 'catppuccin-latte'
  | 'rose-pine'
  | 'rose-pine-moon'
  | 'rose-pine-dawn'
  | 'tokyo-night'
  | 'nord'
  | 'nord-light';

/**
 * Mode selector: dark/light/auto
 * Determines which variant of a style to use
 */
export type Mode = 'dark' | 'light' | 'auto';

/**
 * Style selector: the color palette family
 * Default is 'tokyo-night'
 */
export type StyleId = 'tokyo-night' | 'catppuccin' | 'rose-pine' | 'nord';

export interface TexereSettings {
  hotkeys: { summon: string; copyAndDismiss: string; };
  vim: { enabled: boolean; };
  softWrap: boolean;
  ai: { apiKey: string; model: string; baseUrl: string; systemPrompt: string; };
  appearance: {
    bgColor: string;
    opacity: number;
    mode: Mode;
    style: StyleId;
    // Legacy field for backward compatibility - migrated on load
    theme?: 'auto' | ThemeId;
  };
  autoPaste: boolean;
}

export interface Template {
  id: string;
  name: string;
  content: string;
  createdAt: number;
  updatedAt: number;
}

export interface Note {
  id: string;
  name: string;
  content: string;
  createdAt: number;
  updatedAt: number;
}

export interface EditorState {
  content: string;
  wordCount: number;
  charCount: number;
  vimMode: 'normal' | 'insert' | 'visual' | 'disabled';
}

export interface AIFormatRequest {
  content: string;
}

export interface AIFormatResponse {
  formatted: string;
  style: 'formal' | 'informal' | 'technical' | 'casual';
}
