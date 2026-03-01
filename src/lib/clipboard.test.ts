import { describe, it, expect, vi, beforeEach } from 'vitest';
import { copyAndDismiss, dismissWithoutCopy } from './clipboard';
import { invoke } from '@tauri-apps/api/core';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('clipboard', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe('copyAndDismiss', () => {
    it('should copy content and dismiss with auto-paste enabled', async () => {
      const content = 'Test content';
      const autoPaste = true;
      
      await copyAndDismiss(content, autoPaste);
      
      expect(invoke).toHaveBeenCalledWith('copy_and_dismiss', {
        content,
        autoPaste: true,
      });
    });

    it('should copy content and dismiss without auto-paste', async () => {
      const content = 'Test content';
      const autoPaste = false;
      
      await copyAndDismiss(content, autoPaste);
      
      expect(invoke).toHaveBeenCalledWith('copy_and_dismiss', {
        content,
        autoPaste: false,
      });
    });

    it('should handle empty content', async () => {
      const content = '';
      const autoPaste = true;
      
      await copyAndDismiss(content, autoPaste);
      
      expect(invoke).toHaveBeenCalledWith('copy_and_dismiss', {
        content: '',
        autoPaste: true,
      });
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Clipboard failed');
      (invoke as any).mockRejectedValue(error);
      
      await expect(copyAndDismiss('content', true)).rejects.toThrow('Clipboard failed');
    });
  });

  describe('dismissWithoutCopy', () => {
    it('should dismiss without copying', async () => {
      await dismissWithoutCopy();
      
      expect(invoke).toHaveBeenCalledWith('dismiss_without_copy');
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Dismiss failed');
      (invoke as any).mockRejectedValue(error);
      
      await expect(dismissWithoutCopy()).rejects.toThrow('Dismiss failed');
    });
  });

  describe('auto-paste integration', () => {
    it('should pass autoPaste=true as camelCase to Rust backend', async () => {
      await copyAndDismiss('Hello world', true);

      expect(invoke).toHaveBeenCalledTimes(1);
      const callArgs = (invoke as any).mock.calls[0];
      expect(callArgs[0]).toBe('copy_and_dismiss');
      // Tauri v2 expects camelCase keys from frontend, maps to snake_case in Rust
      expect(callArgs[1]).toHaveProperty('autoPaste', true);
    });

    it('should pass autoPaste=false correctly', async () => {
      await copyAndDismiss('Hello world', false);

      const callArgs = (invoke as any).mock.calls[0];
      expect(callArgs[1]).toHaveProperty('autoPaste', false);
    });

    it('should always include content and autoPaste in invoke payload', async () => {
      await copyAndDismiss('test', true);

      const callArgs = (invoke as any).mock.calls[0];
      const payload = callArgs[1];
      expect(Object.keys(payload).sort()).toEqual(['autoPaste', 'content']);
    });

    it('should not swallow invoke errors during auto-paste', async () => {
      (invoke as any).mockRejectedValue(new Error('Auto-paste permission denied'));

      await expect(copyAndDismiss('content', true)).rejects.toThrow('Auto-paste permission denied');
    });

    it('should handle unicode content with auto-paste', async () => {
      const unicodeContent = '你好，AI\n你好，人类';
      await copyAndDismiss(unicodeContent, true);

      expect(invoke).toHaveBeenCalledWith('copy_and_dismiss', {
        content: unicodeContent,
        autoPaste: true,
      });
    });
  });
});
