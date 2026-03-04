import type { Mode, StyleId, ThemeId, TexereSettings } from "./types";

// ---------------------------------------------------------------------------
// Style Capabilities
// ---------------------------------------------------------------------------

export const STYLE_CAPS: Record<StyleId, { dark: ThemeId[]; light: ThemeId[] }> = {
  'tokyo-night': {
    dark: ['tokyo-night'],
    light: [], // no light variant - will fallback
  },
  'catppuccin': {
    dark: ['catppuccin-mocha'],
    light: ['catppuccin-latte'],
  },
  'nord': {
    dark: ['nord'],
    light: ['nord-light'],
  },
  'rose-pine': {
    dark: ['rose-pine-moon', 'rose-pine'], // prefer moon
    light: ['rose-pine-dawn'],
  },
};

// ---------------------------------------------------------------------------
// Theme Palette System
// ---------------------------------------------------------------------------

interface ThemePalette {
  bg: string;
  editorBg: string;
  surface: string;
  surface2: string;
  text: string;
  caret: string;
  muted: string;
  border: string;
  hover: string;
  statusbarBg: string;
  accent: string;
  ok: string;
  warn: string;
  colorScheme: 'dark' | 'light';
}


// ---------------------------------------------------------------------------
// Palettes — official colors from each theme project
// ---------------------------------------------------------------------------

const THEMES: Record<ThemeId, ThemePalette> = {

  // ── Night ────────────────────────────────────────────────────────────────

  'catppuccin-mocha': {
    bg:          'rgba(30, 30, 46, 0.95)',   // #1e1e2e
    editorBg:    'rgba(24, 24, 37, 0.92)',   // #181825 (mantle)
    surface:     'rgba(49, 50, 68, 0.78)',   // #313244 (surface0)
    surface2:    'rgba(69, 71, 90, 0.72)',   // #45475a (surface1)
    text:        '#cdd6f4',                  // text
    caret:       '#f5e0dc',                  // rosewater (high-contrast caret)
    muted:       '#a6adc8',                  // subtext0
    border:      'rgba(205, 214, 244, 0.20)',
    hover:       'rgba(137, 180, 250, 0.20)',
    statusbarBg: 'rgba(17, 17, 27, 0.55)',   // #11111b (crust)
    accent:      '#89b4fa',                  // blue
    ok:          '#a6e3a1',                  // green
    warn:        '#f9e2af',                  // yellow
    colorScheme: 'dark',
  },

  'rose-pine': {
    bg:          'rgba(25, 23, 36, 0.95)',   // #191724 (base)
    editorBg:    'rgba(25, 23, 36, 0.92)',   // #191724
    surface:     'rgba(31, 29, 46, 0.78)',   // #1f1d2e (surface)
    surface2:    'rgba(38, 35, 58, 0.72)',   // #26233a (overlay)
    text:        '#e0def4',                  // text
    caret:       '#ebbcba',                  // rose
    muted:       '#908caa',                  // subtle
    border:      'rgba(224, 222, 244, 0.15)',
    hover:       'rgba(196, 167, 231, 0.18)',
    statusbarBg: 'rgba(25, 23, 36, 0.60)',
    accent:      '#c4a7e7',                  // iris
    ok:          '#31748f',                  // pine
    warn:        '#f6c177',                  // gold
    colorScheme: 'dark',
  },

  'rose-pine-moon': {
    bg:          'rgba(35, 33, 54, 0.95)',   // #232136 (base)
    editorBg:    'rgba(35, 33, 54, 0.92)',   // #232136
    surface:     'rgba(42, 39, 63, 0.78)',   // #2a273f (surface)
    surface2:    'rgba(57, 53, 82, 0.72)',   // #393552 (overlay)
    text:        '#e0def4',                  // text
    caret:       '#ea9a97',                  // love
    muted:       '#908caa',                  // subtle
    border:      'rgba(224, 222, 244, 0.15)',
    hover:       'rgba(196, 167, 231, 0.18)',
    statusbarBg: 'rgba(35, 33, 54, 0.60)',
    accent:      '#c4a7e7',                  // iris
    ok:          '#3e8fb0',                  // pine
    warn:        '#f6c177',                  // gold
    colorScheme: 'dark',
  },

  'tokyo-night': {
    bg:          'rgba(26, 27, 38, 0.95)',   // #1a1b26 (bg)
    editorBg:    'rgba(26, 27, 38, 0.92)',   // #1a1b26
    surface:     'rgba(36, 40, 59, 0.78)',   // #24283b (bg_highlight)
    surface2:    'rgba(41, 46, 66, 0.72)',   // #292e42
    text:        '#c0caf5',                  // fg
    caret:       '#bb9af7',                  // purple
    muted:       '#565f89',                  // comment
    border:      'rgba(192, 202, 245, 0.15)',
    hover:       'rgba(122, 162, 247, 0.18)',
    statusbarBg: 'rgba(22, 22, 30, 0.55)',   // #16161e (bg_dark)
    accent:      '#7aa2f7',                  // blue
    ok:          '#9ece6a',                  // green
    warn:        '#e0af68',                  // yellow
    colorScheme: 'dark',
  },

  'nord': {
    bg:          'rgba(46, 52, 64, 0.95)',   // #2e3440 (nord0)
    editorBg:    'rgba(46, 52, 64, 0.92)',   // #2e3440
    surface:     'rgba(59, 66, 82, 0.78)',   // #3b4252 (nord1)
    surface2:    'rgba(67, 76, 94, 0.72)',   // #434c5e (nord2)
    text:        '#eceff4',                  // nord6
    caret:       '#88c0d0',                  // nord8
    muted:       '#d8dee9',                  // nord4
    border:      'rgba(236, 239, 244, 0.15)',
    hover:       'rgba(136, 192, 208, 0.18)',
    statusbarBg: 'rgba(46, 52, 64, 0.60)',
    accent:      '#88c0d0',                  // nord8 (frost)
    ok:          '#a3be8c',                  // nord14 (green)
    warn:        '#ebcb8b',                  // nord13 (yellow)
    colorScheme: 'dark',
  },

  // ── Light ────────────────────────────────────────────────────────────────

  'catppuccin-latte': {
    bg:          'rgba(239, 241, 245, 0.96)', // #eff1f5 (base)
    editorBg:    'rgba(230, 233, 239, 0.94)', // #e6e9ef (mantle)
    surface:     'rgba(204, 208, 218, 0.80)', // #ccd0da (surface0)
    surface2:    'rgba(188, 192, 204, 0.75)', // #bcc0cc (surface1)
    text:        '#4c4f69',                   // text
    caret:       '#1e66f5',                   // blue
    muted:       '#6c6f85',                   // subtext0
    border:      'rgba(76, 79, 105, 0.20)',
    hover:       'rgba(30, 102, 245, 0.12)',
    statusbarBg: 'rgba(220, 224, 232, 0.70)', // #dce0e8 (crust)
    accent:      '#1e66f5',                   // blue
    ok:          '#40a02b',                   // green
    warn:        '#df8e1d',                   // yellow
    colorScheme: 'light',
  },

  'rose-pine-dawn': {
    bg:          'rgba(250, 244, 237, 0.96)', // #faf4ed (base)
    editorBg:    'rgba(255, 250, 243, 0.94)', // #fffaf3 (surface)
    surface:     'rgba(242, 233, 222, 0.80)', // #f2e9de (overlay)
    surface2:    'rgba(232, 222, 207, 0.75)', // (derived)
    text:        '#575279',                   // text
    caret:       '#907aa9',                   // iris
    muted:       '#9893a5',                   // subtle
    border:      'rgba(87, 82, 121, 0.18)',
    hover:       'rgba(144, 122, 169, 0.14)',
    statusbarBg: 'rgba(250, 244, 237, 0.70)',
    accent:      '#907aa9',                   // iris
    ok:          '#286983',                   // pine
    warn:        '#ea9d34',                   // gold
    colorScheme: 'light',
  },

  'nord-light': {
    bg:          'rgba(236, 239, 244, 0.96)', // #eceff4 (nord6)
    editorBg:    'rgba(229, 233, 240, 0.94)', // #e5e9f0 (nord5)
    surface:     'rgba(216, 222, 233, 0.80)', // #d8dee9 (nord4)
    surface2:    'rgba(200, 207, 221, 0.75)', // (derived)
    text:        '#2e3440',                   // nord0
    caret:       '#5e81ac',                   // nord10
    muted:       '#4c566a',                   // nord3
    border:      'rgba(46, 52, 64, 0.18)',
    hover:       'rgba(94, 129, 172, 0.14)',
    statusbarBg: 'rgba(216, 222, 233, 0.70)',
    accent:      '#5e81ac',                   // nord10
    ok:          '#a3be8c',                   // nord14
    warn:        '#ebcb8b',                   // nord13
    colorScheme: 'light',
  },
};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * Resolved theme information
 */
export interface ResolveResult {
  theme: ThemeId;
  isFallback: boolean; // true if requested mode not available
  actualMode: 'dark' | 'light';
}

/**
 * Resolve (Mode, Style) to a concrete ThemeId.
 */
export function resolveTheme(
  mode: Mode,
  style: StyleId,
  systemPrefersDark: boolean
): ResolveResult {
  const targetMode = mode === 'auto' ? (systemPrefersDark ? 'dark' : 'light') : mode;
  const caps = STYLE_CAPS[style];

  // Try requested mode first
  if (caps[targetMode].length > 0) {
    return {
      theme: caps[targetMode][0],
      isFallback: false,
      actualMode: targetMode,
    };
  }

  // Fallback to other mode
  const otherMode = targetMode === 'dark' ? 'light' : 'dark';
  return {
    theme: caps[otherMode][0],
    isFallback: true,
    actualMode: otherMode,
  };
}

/**
 * Get the palette for a given theme ID.
 */
export function getThemePalette(theme: ThemeId): ThemePalette {
  return THEMES[theme];
}

/**
 * Apply the appearance settings to the document root.
 */
export function applyAppearance(
  appearance: { bgColor: string; opacity: number; mode: Mode; style: StyleId }
): ResolveResult {
  const root = document.documentElement;
  const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  const resolved = resolveTheme(appearance.mode, appearance.style, systemDark);
  const palette = THEMES[resolved.theme];

  root.style.setProperty('--texere-bg', palette.bg);
  root.style.setProperty('--texere-editor-bg', palette.editorBg);
  root.style.setProperty('--texere-surface', palette.surface);
  root.style.setProperty('--texere-surface-2', palette.surface2);
  root.style.setProperty('--texere-text', palette.text);
  root.style.setProperty('--texere-caret', palette.caret);
  root.style.setProperty('--texere-muted', palette.muted);
  root.style.setProperty('--texere-border', palette.border);
  root.style.setProperty('--texere-hover', palette.hover);
  root.style.setProperty('--texere-statusbar-bg', palette.statusbarBg);
  root.style.setProperty('--texere-accent', palette.accent);
  root.style.setProperty('--texere-ok', palette.ok);
  root.style.setProperty('--texere-warn', palette.warn);
  root.style.setProperty('color-scheme', palette.colorScheme);

  if (palette.colorScheme === 'dark') {
    root.classList.add('dark');
  } else {
    root.classList.remove('dark');
  }

  return resolved;
}

/**
 * Initialize appearance and listen for system theme changes.
 * Returns a cleanup function to remove the listener.
 */
export function initAppearance(appearance: TexereSettings['appearance']) {
  applyAppearance(appearance);

  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  const listener = () => {
    if (appearance && appearance.mode === 'auto') {
      applyAppearance(appearance);
    }
  };

  mediaQuery.addEventListener('change', listener);

  return () => {
    mediaQuery.removeEventListener('change', listener);
  };
}
