import { invoke } from "@tauri-apps/api/core";
import type { Template } from "./types";

export async function getTemplates(): Promise<Template[]> {
  try {
    return await invoke<Template[]>("get_templates");
  } catch (error) {
    console.error("Failed to load templates:", error);
    return [];
  }
}

export async function saveTemplate(template: Template): Promise<void> {
  try {
    await invoke("save_template", { template });
  } catch (error) {
    console.error("Failed to save template:", error);
    throw error;
  }
}

export async function deleteTemplate(id: string): Promise<void> {
  try {
    await invoke("delete_template", { id });
  } catch (error) {
    console.error("Failed to delete template:", error);
    throw error;
  }
}
