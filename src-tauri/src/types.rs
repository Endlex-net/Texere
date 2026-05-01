use serde::{Deserialize, Serialize};

fn default_openai_base_url() -> String {
    "https://api.openai.com/v1".into()
}

fn default_ai_model() -> String {
    "gpt-4o-mini".into()
}

fn default_ai_system_prompt() -> String {
    "You are a text polishing assistant. Rewrite the input to sound more natural and conversational, while staying measured and professional. Requirements: Rewrite directly without asking for confirmation. Keep all facts and intent unchanged. Fix grammar, punctuation, and awkward phrasing. Do not exaggerate, use jokes, or shift the original stance. Use the same language as the user's input. Output only the final rewritten text.".into()
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct Hotkeys {
    pub summon: String,
    pub copy_and_dismiss: String,
}

impl Default for Hotkeys {
    fn default() -> Self {
        Self {
            summon: "CommandOrControl+Shift+Space".into(),
            copy_and_dismiss: "CommandOrControl+Enter".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct VimSettings {
    pub enabled: bool,
}

impl Default for VimSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct AiSettings {
    #[serde(default)]
    pub api_key: String,
    #[serde(default = "default_ai_model")]
    pub model: String,
    #[serde(default = "default_openai_base_url")]
    pub base_url: String,
    #[serde(default = "default_ai_system_prompt")]
    pub system_prompt: String,
}

impl Default for AiSettings {
    fn default() -> Self {
        Self {
            api_key: "".into(),
            model: default_ai_model(),
            base_url: default_openai_base_url(),
            system_prompt: default_ai_system_prompt(),
        }
    }
}

/// Concrete theme palette identifiers
/// Used after resolving (mode, style) → theme
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    #[serde(alias = "dark")]
    CatppuccinMocha,
    #[serde(alias = "light")]
    CatppuccinLatte,
    RosePine,
    RosePineMoon,
    RosePineDawn,
    TokyoNight,
    Nord,
    NordLight,
}

/// Mode selector: dark/light/auto
/// Determines which variant of a style to use
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[default]
    Auto,
    Dark,
    Light,
}

/// Style selector: the color palette family
/// Default is TokyoNight
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Style {
    #[default]
    TokyoNight,
    Catppuccin,
    RosePine,
    Nord,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct AppearanceSettings {
    pub bg_color: String,
    pub opacity: f32,
    pub mode: Mode,
    pub style: Style,
    /// Legacy field for backward compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<Theme>,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            bg_color: "#1e1e2e".into(),
            opacity: 0.95,
            mode: Mode::Auto,
            style: Style::TokyoNight,
            theme: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct TexereSettings {
    pub hotkeys: Hotkeys,
    pub vim: VimSettings,
    pub soft_wrap: bool,
    pub ai: AiSettings,
    pub appearance: AppearanceSettings,
    pub auto_paste: bool,
}

impl Default for TexereSettings {
    fn default() -> Self {
        Self {
            hotkeys: Hotkeys::default(),
            vim: VimSettings::default(),
            soft_wrap: false,
            ai: AiSettings::default(),
            appearance: AppearanceSettings::default(),
            auto_paste: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub id: String,
    pub name: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    pub name: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditorState {
    pub content: String,
    pub word_count: usize,
    pub char_count: usize,
    pub vim_mode: VimMode,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AiFormatRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AiStyle {
    Formal,
    Informal,
    Technical,
    Casual,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AiFormatResponse {
    pub formatted: String,
    pub style: AiStyle,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_texere_settings_roundtrip() {
        let settings = TexereSettings {
            hotkeys: Hotkeys {
                summon: "Alt+Space".into(),
                copy_and_dismiss: "Cmd+Enter".into(),
            },
            vim: VimSettings { enabled: true },
            soft_wrap: true,
            ai: AiSettings {
                api_key: "test-key".into(),
                model: "gpt-4".into(),
                base_url: "https://api.openai.com/v1".into(),
                system_prompt: "You are a text polishing assistant".into(),
            },
            appearance: AppearanceSettings {
                bg_color: "#000000".into(),
                opacity: 0.8,
                mode: Mode::Dark,
                style: Style::TokyoNight,
                theme: None,
            },
            auto_paste: true,
        };

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: TexereSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    #[test]
    fn test_legacy_settings_without_mode_style_parses() {
        let legacy = r##"{
            "hotkeys": {
                "summon": "CommandOrControl+Shift+Enter",
                "copyAndDismiss": "CommandOrControl+Enter"
            },
            "vim": { "enabled": false },
            "ai": {
                "apiKey": "",
                "model": "gpt-4.1-mini",
                "baseUrl": "https://api.openai-proxy.test/v1",
                "systemPrompt": "   "
            },
            "appearance": {
                "bgColor": "#ff0000",
                "opacity": 0.85,
                "theme": "dark"
            },
            "autoPaste": true
        }"##;

        let parsed: TexereSettings = serde_json::from_str(legacy).unwrap();
        assert_eq!(parsed.hotkeys.summon, "CommandOrControl+Shift+Enter");
        assert_eq!(parsed.soft_wrap, false);
        assert_eq!(parsed.appearance.mode, Mode::Auto);
        assert_eq!(parsed.appearance.style, Style::TokyoNight);
        assert_eq!(parsed.appearance.theme, Some(Theme::CatppuccinMocha));
    }

    #[test]
    fn test_template_roundtrip() {
        let template = Template {
            id: "1".into(),
            name: "Test".into(),
            content: "Content".into(),
            created_at: 123456789,
            updated_at: 123456789,
        };

        let json = serde_json::to_string(&template).unwrap();
        let deserialized: Template = serde_json::from_str(&json).unwrap();
        assert_eq!(template, deserialized);
    }

    #[test]
    fn test_note_roundtrip() {
        let note = crate::types::Note {
            id: "abc-123".into(),
            name: "My Note".into(),
            content: "Hello world".into(),
            created_at: 1700000000000,
            updated_at: 1700000001000,
        };

        let json = serde_json::to_string(&note).unwrap();
        let deserialized: crate::types::Note = serde_json::from_str(&json).unwrap();
        assert_eq!(note, deserialized);
    }

    #[test]
    fn test_note_camel_case_serialization() {
        let note = crate::types::Note {
            id: "x".into(),
            name: "N".into(),
            content: "C".into(),
            created_at: 1,
            updated_at: 2,
        };
        let json = serde_json::to_string(&note).unwrap();
        assert!(json.contains("\"createdAt\""));
        assert!(json.contains("\"updatedAt\""));
    }

    #[test]
    fn test_editor_state_roundtrip() {
        let state = EditorState {
            content: "Hello".into(),
            word_count: 1,
            char_count: 5,
            vim_mode: VimMode::Insert,
        };

        let json = serde_json::to_string(&state).unwrap();
        let deserialized: EditorState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, deserialized);
    }

    #[test]
    fn test_ai_format_request_roundtrip() {
        let req = AiFormatRequest {
            content: "raw".into(),
        };
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: AiFormatRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_ai_format_response_roundtrip() {
        let resp = AiFormatResponse {
            formatted: "formatted".into(),
            style: AiStyle::Technical,
        };

        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: AiFormatResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }
}
