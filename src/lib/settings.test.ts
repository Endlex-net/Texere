import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
  canEnableAutoPaste,
  loadSettings,
  saveSettings,
  getDefault,
  DEFAULT_AI_MODEL,
  DEFAULT_AI_SYSTEM_PROMPT,
  DEFAULT_OPENAI_BASE_URL,
} from "./settings";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("settings", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it("should return default settings with mode and style", () => {
    const defaults = getDefault();
    expect(defaults.hotkeys.summon).toBe('CommandOrControl+Shift+Space');
    expect(defaults.vim.enabled).toBe(true);
    expect(defaults.softWrap).toBe(false);
    expect(defaults.autoPaste).toBe(false);
    expect(defaults.appearance.mode).toBe('auto');
    expect(defaults.appearance.style).toBe('tokyo-night');
    expect(defaults.ai.model).toBe(DEFAULT_AI_MODEL);
    expect(defaults.ai.baseUrl).toBe(DEFAULT_OPENAI_BASE_URL);
    expect(defaults.ai.systemPrompt).toBe(DEFAULT_AI_SYSTEM_PROMPT);
  });

  it("should fill missing ai fields from defaults", async () => {
    const mockSettings = {
      ...getDefault(),
      ai: {
        apiKey: 'sk-test',
        model: '',
      },
    };

    (invoke as any).mockResolvedValue(mockSettings);
    const settings = await loadSettings();

    expect(settings.ai.model).toBe(DEFAULT_AI_MODEL);
    expect(settings.ai.baseUrl).toBe(DEFAULT_OPENAI_BASE_URL);
    expect(settings.ai.systemPrompt).toBe(DEFAULT_AI_SYSTEM_PROMPT);
  });

  it("should migrate legacy theme to mode+style", async () => {
    const mockSettings = {
      ...getDefault(),
      appearance: {
        ...getDefault().appearance,
        theme: 'catppuccin-mocha', // legacy theme
        mode: undefined as any,
        style: undefined as any,
      }
    };
    (invoke as any).mockResolvedValue(mockSettings);

    const settings = await loadSettings();
    expect(settings.appearance.mode).toBe('dark');
    expect(settings.appearance.style).toBe('catppuccin');
  });

  it("should load settings via invoke", async () => {
    const mockSettings = getDefault();
    mockSettings.autoPaste = false;
    (invoke as any).mockResolvedValue(mockSettings);

    const settings = await loadSettings();
    expect(invoke).toHaveBeenCalledWith("get_settings");
    expect(settings.autoPaste).toBe(false);
  });

  it("should return defaults if load fails", async () => {
    (invoke as any).mockRejectedValue(new Error("Failed"));
    const settings = await loadSettings();
    expect(settings).toEqual(getDefault());
  });

  it("should save settings via invoke", async () => {
    (invoke as any).mockResolvedValue(undefined);
    const settings = getDefault();
    await saveSettings(settings);
    expect(invoke).toHaveBeenCalledWith("set_settings", { settings });
  });

  it("should query backend for auto-paste permission", async () => {
    (invoke as any).mockResolvedValue(true);
    const granted = await canEnableAutoPaste();
    expect(invoke).toHaveBeenCalledWith("can_enable_auto_paste");
    expect(granted).toBe(true);
  });

  it("should normalize settings before save", async () => {
    (invoke as any).mockResolvedValue(undefined);
    const settings = getDefault();
    (settings.appearance as { opacity: unknown }).opacity = "0.8";
    await saveSettings(settings);
    expect(invoke).toHaveBeenCalledWith("set_settings", {
      settings: {
        ...settings,
        appearance: {
          ...settings.appearance,
          opacity: 0.8,
        },
      },
    });
  });

  describe('autoPaste settings', () => {
    it('should default autoPaste to false', () => {
      const defaults = getDefault();
      expect(defaults.autoPaste).toBe(false);
    });

    it('should preserve autoPaste=false when loading settings', async () => {
      const mockSettings = {
        ...getDefault(),
        autoPaste: false,
      };
      (invoke as any).mockResolvedValue(mockSettings);

      const settings = await loadSettings();
      expect(settings.autoPaste).toBe(false);
    });

    it('should preserve autoPaste=true when loading settings', async () => {
      const mockSettings = {
        ...getDefault(),
        autoPaste: true,
      };
      (invoke as any).mockResolvedValue(mockSettings);

      const settings = await loadSettings();
      expect(settings.autoPaste).toBe(true);
    });

    it('should default autoPaste to false when missing', async () => {
      const mockSettings = {
        ...getDefault(),
        autoPaste: undefined,
      };
      (invoke as any).mockResolvedValue(mockSettings);

      const settings = await loadSettings();
      expect(settings.autoPaste).toBe(false);
    });

    it('should save autoPaste setting correctly', async () => {
      (invoke as any).mockResolvedValue(undefined);
      const settings = {
        ...getDefault(),
        autoPaste: false,
      };
      
      await saveSettings(settings);
      
      expect(invoke).toHaveBeenCalledWith("set_settings", {
        settings: expect.objectContaining({
          autoPaste: false,
        }),
      });
    });
  });

  describe('softWrap settings', () => {
    it('should default softWrap to false', () => {
      const defaults = getDefault();
      expect(defaults.softWrap).toBe(false);
    });

    it('should preserve softWrap=true when loading settings', async () => {
      const mockSettings = {
        ...getDefault(),
        softWrap: true,
      };
      (invoke as any).mockResolvedValue(mockSettings);

      const settings = await loadSettings();
      expect(settings.softWrap).toBe(true);
    });

    it('should default softWrap to false when missing', async () => {
      const mockSettings = {
        ...getDefault(),
        softWrap: undefined,
      };
      (invoke as any).mockResolvedValue(mockSettings);

      const settings = await loadSettings();
      expect(settings.softWrap).toBe(false);
    });
  });

  describe('appearance settings', () => {
    it('should default appearance.mode to auto', () => {
      const defaults = getDefault();
      expect(defaults.appearance.mode).toBe('auto');
    });

    it('should default appearance.style to tokyo-night', () => {
      const defaults = getDefault();
      expect(defaults.appearance.style).toBe('tokyo-night');
    });

    it('should migrate legacy dark theme correctly', async () => {
      const mockSettings = {
        ...getDefault(),
        appearance: {
          ...getDefault().appearance,
          theme: 'dark',
          mode: undefined as any,
          style: undefined as any,
        }
      };
      (invoke as any).mockResolvedValue(mockSettings);

      const settings = await loadSettings();
      expect(settings.appearance.mode).toBe('dark');
      expect(settings.appearance.style).toBe('catppuccin');
    });

    it('should migrate legacy light theme correctly', async () => {
      const mockSettings = {
        ...getDefault(),
        appearance: {
          ...getDefault().appearance,
          theme: 'light',
          mode: undefined as any,
          style: undefined as any,
        }
      };
      (invoke as any).mockResolvedValue(mockSettings);

      const settings = await loadSettings();
      expect(settings.appearance.mode).toBe('light');
      expect(settings.appearance.style).toBe('catppuccin');
    });

    it('should migrate legacy tokyo-night theme correctly', async () => {
      const mockSettings = {
        ...getDefault(),
        appearance: {
          ...getDefault().appearance,
          theme: 'tokyo-night',
          mode: undefined as any,
          style: undefined as any,
        }
      };
      (invoke as any).mockResolvedValue(mockSettings);

      const settings = await loadSettings();
      expect(settings.appearance.mode).toBe('dark');
      expect(settings.appearance.style).toBe('tokyo-night');
    });
  });
});
