import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { applyAppearance, initAppearance, resolveTheme, STYLE_CAPS } from './appearance';
import type { Mode, StyleId } from './types';

// Mock DOM
let mockRoot: any;

describe('appearance', () => {
  beforeEach(() => {
    mockRoot = {
      style: {
        properties: new Map(),
        setProperty(key: string, value: string) {
          this.properties.set(key, value);
        },
        getPropertyValue(key: string) {
          return this.properties.get(key);
        }
      },
      classList: {
        classes: new Set(),
        add(cls: string) { this.classes.add(cls); },
        remove(cls: string) { this.classes.delete(cls); },
        contains(cls: string) { return this.classes.has(cls); }
      }
    };
    
    globalThis.document = {
      documentElement: mockRoot
    } as any;
    
    globalThis.window = {
      matchMedia: () => ({
        matches: false,
        addEventListener: () => {},
        removeEventListener: () => {},
      })
    } as any;
  });

  afterEach(() => {
    delete (globalThis as any).document;
    delete (globalThis as any).window;
  });

  describe('STYLE_CAPS', () => {
    it('defines capabilities for all styles', () => {
      expect(STYLE_CAPS['tokyo-night']).toBeDefined();
      expect(STYLE_CAPS['catppuccin']).toBeDefined();
      expect(STYLE_CAPS['nord']).toBeDefined();
      expect(STYLE_CAPS['rose-pine']).toBeDefined();
    });

    it('tokyo-night only has dark variant', () => {
      expect(STYLE_CAPS['tokyo-night'].dark).toHaveLength(1);
      expect(STYLE_CAPS['tokyo-night'].light).toHaveLength(0);
    });

    it('catppuccin has both dark and light', () => {
      expect(STYLE_CAPS['catppuccin'].dark).toContain('catppuccin-mocha');
      expect(STYLE_CAPS['catppuccin'].light).toContain('catppuccin-latte');
    });
  });

  describe('resolveTheme', () => {
    it('resolves tokyo-night dark correctly', () => {
      const result = resolveTheme('dark', 'tokyo-night', false);
      expect(result.theme).toBe('tokyo-night');
      expect(result.isFallback).toBe(false);
      expect(result.actualMode).toBe('dark');
    });

    it('falls back to dark when light requested for tokyo-night', () => {
      const result = resolveTheme('light', 'tokyo-night', false);
      expect(result.theme).toBe('tokyo-night');
      expect(result.isFallback).toBe(true);
      expect(result.actualMode).toBe('dark');
    });

    it('resolves catppuccin dark to mocha', () => {
      const result = resolveTheme('dark', 'catppuccin', false);
      expect(result.theme).toBe('catppuccin-mocha');
      expect(result.isFallback).toBe(false);
    });

    it('resolves catppuccin light to latte', () => {
      const result = resolveTheme('light', 'catppuccin', false);
      expect(result.theme).toBe('catppuccin-latte');
      expect(result.isFallback).toBe(false);
    });

    it('uses system preference in auto mode (dark)', () => {
      const result = resolveTheme('auto', 'nord', true);
      expect(result.theme).toBe('nord');
      expect(result.actualMode).toBe('dark');
    });

    it('uses system preference in auto mode (light)', () => {
      const result = resolveTheme('auto', 'nord', false);
      expect(result.theme).toBe('nord-light');
      expect(result.actualMode).toBe('light');
    });

    it('resolves rose-pine dark to moon (preferred)', () => {
      const result = resolveTheme('dark', 'rose-pine', false);
      expect(result.theme).toBe('rose-pine-moon');
    });

    it('resolves rose-pine light to dawn', () => {
      const result = resolveTheme('light', 'rose-pine', false);
      expect(result.theme).toBe('rose-pine-dawn');
    });
  });

  describe('applyAppearance', () => {
    it('applies tokyo-night dark theme correctly', () => {
      const result = applyAppearance({
        mode: 'dark',
        style: 'tokyo-night',
        bgColor: '#1e1e2e',
        opacity: 0.95
      });
      
      expect(result.theme).toBe('tokyo-night');
      expect(mockRoot.style.getPropertyValue('--texere-text')).toBe('#c0caf5');
      expect(mockRoot.style.getPropertyValue('--texere-accent')).toBe('#7aa2f7');
      expect(mockRoot.classList.contains('dark')).toBe(true);
    });

    it('applies catppuccin latte light theme correctly', () => {
      const result = applyAppearance({
        mode: 'light',
        style: 'catppuccin',
        bgColor: '#eff1f5',
        opacity: 0.96
      });
      
      expect(result.theme).toBe('catppuccin-latte');
      expect(mockRoot.style.getPropertyValue('--texere-text')).toBe('#4c4f69');
      expect(mockRoot.classList.contains('dark')).toBe(false);
    });

    it('returns fallback info when style does not support mode', () => {
      const result = applyAppearance({
        mode: 'light',
        style: 'tokyo-night',
        bgColor: '#1e1e2e',
        opacity: 0.95
      });
      
      expect(result.isFallback).toBe(true);
      expect(result.actualMode).toBe('dark');
    });

    it('sets all required CSS variables', () => {
      applyAppearance({
        mode: 'dark',
        style: 'nord',
        bgColor: '#2e3440',
        opacity: 0.95
      });

      const requiredVars = [
        '--texere-bg', '--texere-editor-bg', '--texere-surface', '--texere-surface-2',
        '--texere-text', '--texere-muted', '--texere-border', '--texere-hover',
        '--texere-statusbar-bg', '--texere-accent', '--texere-ok', '--texere-warn',
        'color-scheme'
      ];
      
      for (const varName of requiredVars) {
        expect(mockRoot.style.getPropertyValue(varName)).toBeTruthy();
      }
    });
  });

  describe('initAppearance', () => {
    it('applies settings and returns cleanup function', () => {
      const appearance = {
        mode: 'auto' as Mode,
        style: 'tokyo-night' as StyleId,
        bgColor: '#1e1e2e',
        opacity: 0.95
      };
      
      const cleanup = initAppearance(appearance);
      
      expect(typeof cleanup).toBe('function');
      expect(mockRoot.classList.contains('dark')).toBe(true);
    });
  });
});
