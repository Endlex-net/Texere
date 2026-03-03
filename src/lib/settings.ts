import { invoke } from "@tauri-apps/api/core";
import type { Mode, StyleId, TexereSettings } from "./types";

export const DEFAULT_OPENAI_BASE_URL = 'https://api.openai.com/v1';
export const DEFAULT_AI_MODEL = 'gpt-4o-mini';
export const DEFAULT_AI_SYSTEM_PROMPT = "You are a text polishing assistant. Rewrite the input to sound more natural and conversational, while staying measured and professional. Requirements: Rewrite directly without asking for confirmation. Keep all facts and intent unchanged. Fix grammar, punctuation, and awkward phrasing. Do not exaggerate, use jokes, or shift the original stance. Use the same language as the user's input. Output only the final rewritten text.";

export function getDefault(): TexereSettings {
  return {
    hotkeys: {
      summon: 'CommandOrControl+Shift+Space',
      copyAndDismiss: 'CommandOrControl+Enter'
    },
    vim: {
      enabled: true
    },
    softWrap: false,
    ai: {
      apiKey: '',
      model: DEFAULT_AI_MODEL,
      baseUrl: DEFAULT_OPENAI_BASE_URL,
      systemPrompt: DEFAULT_AI_SYSTEM_PROMPT,
    },
    appearance: {
      bgColor: '#1e1e2e',
      opacity: 0.95,
      mode: 'auto',
      style: 'tokyo-night',
    },
    autoPaste: false
  };
}

export async function loadSettings(): Promise<TexereSettings> {
  try {
    const loaded = await invoke<TexereSettings>("get_settings");
    return normalizeSettings(loaded);
  } catch (error) {
    console.error("Failed to load settings, returning defaults:", error);
    return getDefault();
  }
}

export async function saveSettings(settings: TexereSettings): Promise<void> {
  try {
    const normalized = normalizeSettings(settings);
    await invoke("set_settings", { settings: normalized });
  } catch (error) {
    console.error("Failed to save settings:", error);
    throw error;
  }
}

export async function canEnableAutoPaste(): Promise<boolean> {
  try {
    return await invoke<boolean>("can_enable_auto_paste");
  } catch (error) {
    console.error("Failed to check auto-paste permission:", error);
    return false;
  }
}

const VALID_MODES: readonly Mode[] = ['dark', 'light', 'auto'];
const VALID_STYLES: readonly StyleId[] = ['tokyo-night', 'catppuccin', 'rose-pine', 'nord'];

function isValidMode(m: string): m is Mode {
  return VALID_MODES.includes(m as Mode);
}

function isValidStyle(s: string): s is StyleId {
  return VALID_STYLES.includes(s as StyleId);
}

const LEGACY_THEME_MAP: Record<string, { mode: Mode; style: StyleId }> = {
  'auto': { mode: 'auto', style: 'tokyo-night' },
  'dark': { mode: 'dark', style: 'catppuccin' },  // legacy dark
  'light': { mode: 'light', style: 'catppuccin' }, // legacy light
  'catppuccin-mocha': { mode: 'dark', style: 'catppuccin' },
  'catppuccin-latte': { mode: 'light', style: 'catppuccin' },
  'rose-pine': { mode: 'dark', style: 'rose-pine' },
  'rose-pine-moon': { mode: 'dark', style: 'rose-pine' },
  'rose-pine-dawn': { mode: 'light', style: 'rose-pine' },
  'tokyo-night': { mode: 'dark', style: 'tokyo-night' },
  'nord': { mode: 'dark', style: 'nord' },
  'nord-light': { mode: 'light', style: 'nord' },
};

function normalizeSettings(input: TexereSettings): TexereSettings {
  const defaults = getDefault();
  const opacity = Number(input?.appearance?.opacity);
  const autoPaste =
    typeof input?.autoPaste === 'boolean'
      ? input.autoPaste
      : defaults.autoPaste;
  const softWrap =
    typeof input?.softWrap === 'boolean'
      ? input.softWrap
      : defaults.softWrap;

  // Handle appearance settings
  let mode: Mode = input?.appearance?.mode;
  let style: StyleId = input?.appearance?.style;

  // If new fields missing, try to migrate from legacy theme
  if (!mode || !isValidMode(mode)) {
    const legacyTheme = input?.appearance?.theme;
    if (legacyTheme && LEGACY_THEME_MAP[legacyTheme]) {
      const migrated = LEGACY_THEME_MAP[legacyTheme];
      mode = migrated.mode;
      style = migrated.style ?? defaults.appearance.style;
    } else {
      mode = defaults.appearance.mode;
      style = defaults.appearance.style;
    }
  }

  // Validate style
  if (!style || !isValidStyle(style)) {
    style = defaults.appearance.style;
  }

  const aiModel = String(input?.ai?.model ?? defaults.ai.model).trim() || defaults.ai.model;
  const aiBaseUrl = String(input?.ai?.baseUrl ?? defaults.ai.baseUrl).trim() || defaults.ai.baseUrl;
  const aiSystemPrompt = String(input?.ai?.systemPrompt ?? defaults.ai.systemPrompt).trim() || defaults.ai.systemPrompt;

  return {
    hotkeys: {
      summon: String(input?.hotkeys?.summon ?? defaults.hotkeys.summon),
      copyAndDismiss: String(input?.hotkeys?.copyAndDismiss ?? defaults.hotkeys.copyAndDismiss),
    },
    vim: {
      enabled: Boolean(input?.vim?.enabled),
    },
    softWrap,
    ai: {
      apiKey: String(input?.ai?.apiKey ?? defaults.ai.apiKey),
      model: aiModel,
      baseUrl: aiBaseUrl,
      systemPrompt: aiSystemPrompt,
    },
    appearance: {
      bgColor: String(input?.appearance?.bgColor ?? defaults.appearance.bgColor),
      opacity: Number.isFinite(opacity) ? opacity : defaults.appearance.opacity,
      mode,
      style,
      // Don't include theme in output - it's legacy only
    },
    autoPaste,
  };
}
