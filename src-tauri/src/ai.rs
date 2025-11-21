// AI rephrasing module - Universal LLM integration
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::Style;

// Universal rephrase function supporting multiple LLM providers
pub async fn rephrase_text(
    text: &str,
    style: &Style,
    provider: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    match provider.to_lowercase().as_str() {
        "proxy" => rephrase_with_proxy(text, style).await,
        "openai" => rephrase_with_openai(text, style, api_key).await,
        "claude" | "anthropic" => rephrase_with_claude(text, style, api_key).await,
        "gemini" | "google" => rephrase_with_gemini(text, style, api_key).await,
        "perplexity" => rephrase_with_perplexity(text, style, api_key).await,
        _ => Err(format!("Unsupported provider: {}", provider).into()),
    }
}

fn get_prompt_for_style(text: &str, style: &Style) -> String {
    let style_instruction = match style {
        Style::Professional => "Rephrase the following text in a professional, formal tone suitable for business communication. Maintain the core message but improve clarity and professionalism.",
        Style::Casual => "Rephrase the following text in a casual, friendly tone suitable for informal communication. Make it conversational and approachable.",
        Style::Sarcasm => "Rephrase the following text with subtle sarcasm while maintaining the surface-level message. Keep it witty but not offensive.",
    };
    
    format!("{}\n\nText: {}", style_instruction, text)
}

// Proxy server integration (default - uses your API key)
async fn rephrase_with_proxy(
    text: &str,
    style: &Style,
) -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("🌐 Using proxy server for rephrasing");
    let client = Client::new();
    
    const PROXY_URL: &str = "https://rephraser-9ur5.onrender.com/api/rephrase";
    
    let style_str = match style {
        Style::Professional => "professional",
        Style::Casual => "casual",
        Style::Sarcasm => "sarcasm",
    };
    
    eprintln!("📤 Sending request to proxy: style={}, text_len={}", style_str, text.len());
    
    #[derive(Serialize)]
    struct ProxyRequest {
        text: String,
        style: String,
    }
    
    #[derive(Deserialize)]
    struct ProxyResponse {
        rephrased: String,
    }
    
    let request = ProxyRequest {
        text: text.to_string(),
        style: style_str.to_string(),
    };
    
    let response = client
        .post(PROXY_URL)
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .await
        .map_err(|e| {
            eprintln!("❌ Proxy request failed: {:?}", e);
            handle_request_error(e)
        })?;
    
    let status = response.status();
    eprintln!("📥 Proxy response status: {}", status);
    
    if !status.is_success() {
        let error = handle_api_error(status.as_u16(), "Proxy Server");
        eprintln!("❌ Proxy error: {}", error);
        return Err(error.into());
    }
    
    let data: ProxyResponse = response.json().await?;
    eprintln!("✅ Proxy rephrase successful, result_len={}", data.rephrased.len());
    
    Ok(data.rephrased.trim().to_string())
}

// OpenAI GPT-4 integration
async fn rephrase_with_openai(
    text: &str,
    style: &Style,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    #[derive(Serialize, Deserialize)]
    struct OpenAIMessage {
        role: String,
        content: String,
    }
    
    #[derive(Serialize)]
    struct OpenAIRequest {
        model: String,
        messages: Vec<OpenAIMessage>,
        temperature: f32,
    }
    
    #[derive(Deserialize)]
    struct OpenAIResponse {
        choices: Vec<OpenAIChoice>,
    }
    
    #[derive(Deserialize)]
    struct OpenAIChoice {
        message: OpenAIMessage,
    }
    
    let request = OpenAIRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: "You are a helpful writing assistant. Rephrase text according to the user's instructions.".to_string(),
            },
            OpenAIMessage {
                role: "user".to_string(),
                content: get_prompt_for_style(text, style),
            },
        ],
        temperature: 0.7,
    };
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| handle_request_error(e))?;
    
    if !response.status().is_success() {
        return Err(handle_api_error(response.status().as_u16(), "OpenAI").into());
    }
    
    let data: OpenAIResponse = response.json().await?;
    let rephrased = data.choices.get(0)
        .and_then(|c| Some(c.message.content.trim().to_string()))
        .ok_or("No response from OpenAI")?;
    
    Ok(rephrased)
}

// Anthropic Claude integration
async fn rephrase_with_claude(
    text: &str,
    style: &Style,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    #[derive(Serialize, Deserialize)]
    struct ClaudeMessage {
        role: String,
        content: String,
    }
    
    #[derive(Serialize)]
    struct ClaudeRequest {
        model: String,
        max_tokens: u32,
        messages: Vec<ClaudeMessage>,
    }
    
    #[derive(Deserialize)]
    struct ClaudeResponse {
        content: Vec<ClaudeContent>,
    }
    
    #[derive(Deserialize)]
    struct ClaudeContent {
        text: String,
    }
    
    let request = ClaudeRequest {
        model: "claude-3-5-sonnet-20241022".to_string(),
        max_tokens: 1024,
        messages: vec![
            ClaudeMessage {
                role: "user".to_string(),
                content: get_prompt_for_style(text, style),
            },
        ],
    };
    
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| handle_request_error(e))?;
    
    if !response.status().is_success() {
        return Err(handle_api_error(response.status().as_u16(), "Claude").into());
    }
    
    let data: ClaudeResponse = response.json().await?;
    let rephrased = data.content.get(0)
        .map(|c| c.text.trim().to_string())
        .ok_or("No response from Claude")?;
    
    Ok(rephrased)
}

// Google Gemini integration
async fn rephrase_with_gemini(
    text: &str,
    style: &Style,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    #[derive(Serialize, Deserialize)]
    struct GeminiPart {
        text: String,
    }
    
    #[derive(Serialize, Deserialize)]
    struct GeminiContent {
        parts: Vec<GeminiPart>,
    }
    
    #[derive(Serialize)]
    struct GeminiRequest {
        contents: Vec<GeminiContent>,
    }
    
    #[derive(Deserialize)]
    struct GeminiResponse {
        candidates: Vec<GeminiCandidate>,
    }
    
    #[derive(Deserialize)]
    struct GeminiCandidate {
        content: GeminiContent,
    }
    
    let request = GeminiRequest {
        contents: vec![
            GeminiContent {
                parts: vec![
                    GeminiPart {
                        text: get_prompt_for_style(text, style),
                    },
                ],
            },
        ],
    };
    
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", api_key);
    
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| handle_request_error(e))?;
    
    if !response.status().is_success() {
        return Err(handle_api_error(response.status().as_u16(), "Gemini").into());
    }
    
    let data: GeminiResponse = response.json().await?;
    let rephrased = data.candidates.get(0)
        .and_then(|c| c.content.parts.get(0))
        .map(|p| p.text.trim().to_string())
        .ok_or("No response from Gemini")?;
    
    Ok(rephrased)
}

// Perplexity integration
async fn rephrase_with_perplexity(
    text: &str,
    style: &Style,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    #[derive(Serialize, Deserialize)]
    struct PerplexityMessage {
        role: String,
        content: String,
    }
    
    #[derive(Serialize)]
    struct PerplexityRequest {
        model: String,
        messages: Vec<PerplexityMessage>,
    }
    
    #[derive(Deserialize)]
    struct PerplexityResponse {
        choices: Vec<PerplexityChoice>,
    }
    
    #[derive(Deserialize)]
    struct PerplexityChoice {
        message: PerplexityMessage,
    }
    
    let request = PerplexityRequest {
        model: "llama-3.1-sonar-small-128k-online".to_string(),
        messages: vec![
            PerplexityMessage {
                role: "system".to_string(),
                content: "You are a helpful writing assistant. Rephrase text according to the user's instructions.".to_string(),
            },
            PerplexityMessage {
                role: "user".to_string(),
                content: get_prompt_for_style(text, style),
            },
        ],
    };
    
    let response = client
        .post("https://api.perplexity.ai/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| handle_request_error(e))?;
    
    if !response.status().is_success() {
        return Err(handle_api_error(response.status().as_u16(), "Perplexity").into());
    }
    
    let data: PerplexityResponse = response.json().await?;
    let rephrased = data.choices.get(0)
        .map(|c| c.message.content.trim().to_string())
        .ok_or("No response from Perplexity")?;
    
    Ok(rephrased)
}

// Helper functions for error handling
fn handle_request_error(e: reqwest::Error) -> String {
    if e.is_timeout() {
        "Request timed out. Please check your internet connection or try again.".to_string()
    } else if e.is_connect() {
        "Cannot connect to API server. Please check your internet connection.".to_string()
    } else {
        format!("Network error: {}", e)
    }
}

fn handle_api_error(status: u16, provider: &str) -> String {
    match status {
        401 => format!("{} API key is invalid. Please check your API key in Settings.", provider),
        429 => format!("{} rate limit exceeded. Please wait a moment and try again.", provider),
        500..=599 => format!("{} service is temporarily unavailable. Please try again later.", provider),
        _ => format!("{} API error (status {}). Please try again.", provider, status),
    }
}



