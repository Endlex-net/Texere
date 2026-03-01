import { invoke } from '@tauri-apps/api/core';
import {
  DEFAULT_AI_MODEL,
  DEFAULT_AI_SYSTEM_PROMPT,
  DEFAULT_OPENAI_BASE_URL,
  loadSettings,
} from './settings';

export interface AIFormatResponse {
  formatted: string;
  style: 'formal' | 'informal' | 'technical' | 'casual';
}

export async function formatText(content: string): Promise<AIFormatResponse> {
  const settings = await loadSettings();
  
  if (!settings.ai.apiKey) {
    throw new Error('OpenAI API key is not configured. Please add it in settings.');
  }
  
  return invoke('format_text', {
    apiKey: settings.ai.apiKey,
    content,
    model: settings.ai.model || DEFAULT_AI_MODEL,
    baseUrl: settings.ai.baseUrl || DEFAULT_OPENAI_BASE_URL,
    systemPrompt: settings.ai.systemPrompt || DEFAULT_AI_SYSTEM_PROMPT,
  });
}
