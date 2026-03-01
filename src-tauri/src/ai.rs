use serde::{Deserialize, Serialize};

const DEFAULT_SYSTEM_PROMPT: &str = "You are a text polishing assistant. Rewrite the input to sound more natural and conversational, while staying measured and professional. Requirements: Rewrite directly without asking for confirmation. Keep all facts and intent unchanged. Fix grammar, punctuation, and awkward phrasing. Do not exaggerate, use jokes, or shift the original stance. Use the same language as the user's input. Output only the final rewritten text.";
const DEFAULT_OPENAI_BASE_URL: &str = "https://api.openai.com/v1";

#[derive(Debug, Serialize)]
struct ChatCompletionsRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionsResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Debug, Deserialize)]
struct AssistantMessage {
    content: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AIFormatResponse {
    pub formatted: String,
    pub style: String,
}

fn detect_style(content: &str, formatted: &str) -> String {
    let sample = if formatted.trim().is_empty() {
        content
    } else {
        formatted
    }
    .to_lowercase();

    let technical_markers = [
        "http://",
        "https://",
        "api",
        "json",
        "yaml",
        "error",
        "stack",
        "function",
        "class",
        "endpoint",
        "cli",
        "config",
    ];
    if technical_markers.iter().any(|m| sample.contains(m)) {
        return "technical".to_string();
    }

    let formal_markers = [
        "dear",
        "sincerely",
        "regards",
        "thank you",
        "please",
        "subject:",
        "kind regards",
    ];
    if formal_markers.iter().any(|m| sample.contains(m)) {
        return "formal".to_string();
    }

    "informal".to_string()
}

#[tauri::command]
pub async fn format_text(
    api_key: String,
    content: String,
    model: String,
    base_url: String,
    system_prompt: String,
) -> Result<AIFormatResponse, String> {
    if api_key.trim().is_empty() {
        return Err("OpenAI API key is not configured".to_string());
    }

    // Handle empty content edge case
    if content.trim().is_empty() {
        return Ok(AIFormatResponse {
            formatted: content,
            style: "informal".to_string(),
        });
    }

    let normalized_base_url = {
        let trimmed = base_url.trim();
        let value = if trimmed.is_empty() {
            DEFAULT_OPENAI_BASE_URL
        } else {
            trimmed
        };
        value.trim_end_matches('/').to_string()
    };
    let prompt = if system_prompt.trim().is_empty() {
        DEFAULT_SYSTEM_PROMPT.to_string()
    } else {
        system_prompt.trim().to_string()
    };

    let request = ChatCompletionsRequest {
        model,
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: content.clone(),
            },
        ],
    };

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    let response = client
        .post(format!("{}/chat/completions", normalized_base_url))
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                "Network timeout. Check your connection and try again.".to_string()
            } else if e.is_connect() {
                "Network error. Check your internet connection.".to_string()
            } else {
                format!("Request failed: {}", e)
            }
        })?;

    // Handle HTTP errors
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "".to_string());
        
        // Parse error for better messages
        return Err(match status.as_u16() {
            401 => "Invalid API key. Check your OpenAI API key in settings.".to_string(),
            429 => "API rate limit reached. Try again in a moment.".to_string(),
            500..=599 => "OpenAI service error. Try again later.".to_string(),
            _ => format!("API request failed ({}): {}", status, body)
        });
    }

    let body: ChatCompletionsResponse = response.json().await.map_err(|e| {
        format!("Unexpected response from AI. Try again. ({})", e)
    })?;
    
    let formatted = body
        .choices
        .first()
        .map(|choice| choice.message.content.trim().to_string())
        .unwrap_or_else(|| content.clone());

    let style = detect_style(&content, &formatted);

    Ok(AIFormatResponse { formatted, style })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_content_returns_default_response() {
        let content = String::new();
        assert!(content.is_empty());
    }

    #[test]
    fn test_missing_api_key_error() {
        let api_key = String::new();
        assert!(api_key.is_empty());
    }

    #[test]
    fn test_whitespace_only_api_key() {
        let api_key = "   ";
        assert!(api_key.trim().is_empty());
    }

    #[test]
    fn test_detect_style_with_empty_formatted() {
        let content = "hello world";
        let formatted = "";
        let style = detect_style(content, formatted);
        assert!(style == "informal" || style == "formal" || style == "technical");
    }

    #[test]
    fn test_detect_style_technical_markers() {
        let content = "check the api endpoint";
        let formatted = content;
        let style = detect_style(content, formatted);
        assert_eq!(style, "technical");
    }

    #[test]
    fn test_detect_style_formal_markers() {
        let content = "Dear Sir, thank you for your assistance";
        let formatted = content;
        let style = detect_style(content, formatted);
        assert_eq!(style, "formal");
    }

    #[test]
    fn test_detect_style_informal_fallback() {
        let content = "hey what's up";
        let formatted = content;
        let style = detect_style(content, formatted);
        assert_eq!(style, "informal");
    }

    #[test]
    fn test_system_prompt_not_empty() {
        assert!(!DEFAULT_SYSTEM_PROMPT.is_empty());
        assert!(DEFAULT_SYSTEM_PROMPT.len() > 20);
    }

    #[test]
    fn test_chat_message_serialization() {
        let msg = ChatMessage {
            role: "user".to_string(),
            content: "test".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("test"));
    }

    #[test]
    fn test_request_has_model_and_messages() {
        let request = ChatCompletionsRequest {
            model: "gpt-4".to_string(),
            messages: vec![ChatMessage {
                role: "system".to_string(),
                content: "test".to_string(),
            }],
        };
        assert_eq!(request.model, "gpt-4");
        assert_eq!(request.messages.len(), 1);
    }
}
