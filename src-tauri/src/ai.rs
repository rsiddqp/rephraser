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
        Style::Professional => "Rephrase the following text in a professional, formal tone suitable for business communication. Maintain the core message but improve clarity and professionalism. IMPORTANT: Return ONLY the rephrased text, without any introduction, explanation, or preamble.",
        Style::Casual => "Rephrase the following text in a casual, friendly tone suitable for informal communication. Make it conversational and approachable. IMPORTANT: Return ONLY the rephrased text, without any introduction, explanation, or preamble.",
        Style::Sarcasm => "Rephrase the following text with subtle sarcasm while maintaining the surface-level message. Keep it witty but not offensive. IMPORTANT: Return ONLY the rephrased text, without any introduction, explanation, or preamble.",
    };
    
    format!("{}\n\nText: {}", style_instruction, text)
}

// Helper function to clean up AI responses that include preambles
fn strip_preamble(text: &str) -> String {
    let text = text.trim();
    
    // Common preamble patterns to remove
    let preamble_patterns = [
        "Certainly. Here is a professionally rephrased version of your text:",
        "Certainly. Here is a casually rephrased version of your text:",
        "Certainly. Here is a rephrased version of your text:",
        "Here is a professionally rephrased version:",
        "Here is a casually rephrased version:",
        "Here is a rephrased version:",
        "Here's a professional version:",
        "Here's a casual version:",
        "Here's the rephrased text:",
        "Certainly! Here is",
        "Sure! Here is",
        "Of course! Here is",
    ];
    
    let mut result = text.to_string();
    
    // Remove preamble patterns (case insensitive)
    for pattern in &preamble_patterns {
        let pattern_lower = pattern.to_lowercase();
        let result_lower = result.to_lowercase();
        
        if let Some(pos) = result_lower.find(&pattern_lower) {
            // Remove everything up to and including the pattern
            result = result[pos + pattern.len()..].trim().to_string();
        }
    }
    
    // Remove common separators that appear after preambles
    let separators = ["---", "***", "===", "..."];
    for sep in &separators {
        if result.starts_with(sep) {
            result = result[sep.len()..].trim().to_string();
        }
    }
    
    // Remove leading/trailing quotes if present
    if (result.starts_with('"') && result.ends_with('"')) || 
       (result.starts_with('\'') && result.ends_with('\'')) {
        result = result[1..result.len()-1].to_string();
    }
    
    result.trim().to_string()
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
    eprintln!("🤖 Using OpenAI for rephrasing");
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
    
    eprintln!("📤 Sending request to OpenAI: model=gpt-4o-mini, text_len={}", text.len());
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| {
            eprintln!("❌ OpenAI request failed: {:?}", e);
            handle_request_error(e)
        })?;
    
    let status = response.status();
    eprintln!("📥 OpenAI response status: {}", status);
    
    if !status.is_success() {
        let error = handle_api_error(status.as_u16(), "OpenAI");
        eprintln!("❌ OpenAI error: {}", error);
        return Err(error.into());
    }
    
    let data: OpenAIResponse = response.json().await?;
    let rephrased = data.choices.get(0)
        .and_then(|c| Some(c.message.content.trim().to_string()))
        .ok_or("No response from OpenAI")?;
    
    eprintln!("✅ OpenAI rephrase successful, result_len={}", rephrased.len());
    
    // Clean any preamble
    let cleaned = strip_preamble(&rephrased);
    eprintln!("✂️  Cleaned preamble: original_len={}, cleaned_len={}", rephrased.len(), cleaned.len());
    
    Ok(cleaned)
}

// Anthropic Claude integration
async fn rephrase_with_claude(
    text: &str,
    style: &Style,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("🤖 Using Anthropic Claude for rephrasing");
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
    
    // Use the latest Claude 3.5 Sonnet model
    // Model names: claude-3-5-sonnet-20241022 (latest as of Nov 2024)
    let request = ClaudeRequest {
        model: "claude-3-5-sonnet-20241022".to_string(),
        max_tokens: 2048, // Increased for longer rephrases
        messages: vec![
            ClaudeMessage {
                role: "user".to_string(),
                content: get_prompt_for_style(text, style),
            },
        ],
    };
    
    eprintln!("📤 Sending request to Claude API: model=claude-3-5-sonnet-20241022, text_len={}", text.len());
    
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| {
            eprintln!("❌ Claude request failed: {:?}", e);
            handle_request_error(e)
        })?;
    
    let status = response.status();
    eprintln!("📥 Claude response status: {}", status);
    
    if !status.is_success() {
        let error = handle_api_error(status.as_u16(), "Claude");
        eprintln!("❌ Claude error: {}", error);
        return Err(error.into());
    }
    
    let data: ClaudeResponse = response.json().await?;
    let rephrased = data.content.get(0)
        .map(|c| c.text.trim().to_string())
        .ok_or("No response from Claude")?;
    
    eprintln!("✅ Claude rephrase successful, result_len={}", rephrased.len());
    
    // Clean any preamble
    let cleaned = strip_preamble(&rephrased);
    eprintln!("✂️  Cleaned preamble: original_len={}, cleaned_len={}", rephrased.len(), cleaned.len());
    
    Ok(cleaned)
}

// Google Gemini integration
async fn rephrase_with_gemini(
    text: &str,
    style: &Style,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("🤖 Using Google Gemini for rephrasing");
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
    
    eprintln!("📤 Sending request to Gemini: model=gemini-pro, text_len={}", text.len());
    
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| {
            eprintln!("❌ Gemini request failed: {:?}", e);
            handle_request_error(e)
        })?;
    
    let status = response.status();
    eprintln!("📥 Gemini response status: {}", status);
    
    if !status.is_success() {
        let error = handle_api_error(status.as_u16(), "Gemini");
        eprintln!("❌ Gemini error: {}", error);
        return Err(error.into());
    }
    
    let data: GeminiResponse = response.json().await?;
    let rephrased = data.candidates.get(0)
        .and_then(|c| c.content.parts.get(0))
        .map(|p| p.text.trim().to_string())
        .ok_or("No response from Gemini")?;
    
    eprintln!("✅ Gemini rephrase successful, result_len={}", rephrased.len());
    
    // Clean any preamble
    let cleaned = strip_preamble(&rephrased);
    eprintln!("✂️  Cleaned preamble: original_len={}, cleaned_len={}", rephrased.len(), cleaned.len());
    
    Ok(cleaned)
}

// Perplexity integration
async fn rephrase_with_perplexity(
    text: &str,
    style: &Style,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("🤖 Using Perplexity for rephrasing");
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
    
    // Use "sonar" model - tested and verified working
    let request = PerplexityRequest {
        model: "sonar".to_string(),
        messages: vec![
            PerplexityMessage {
                role: "user".to_string(),
                content: get_prompt_for_style(text, style),
            },
        ],
    };
    
    eprintln!("📤 Sending request to Perplexity: model=sonar, text_len={}", text.len());
    
    let response = client
        .post("https://api.perplexity.ai/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| {
            eprintln!("❌ Perplexity request failed: {:?}", e);
            handle_request_error(e)
        })?;
    
    let status = response.status();
    eprintln!("📥 Perplexity response status: {}", status);
    
    if !status.is_success() {
        let error = handle_api_error(status.as_u16(), "Perplexity");
        eprintln!("❌ Perplexity error: {}", error);
        return Err(error.into());
    }
    
    let data: PerplexityResponse = response.json().await?;
    let rephrased = data.choices.get(0)
        .map(|c| c.message.content.trim().to_string())
        .ok_or("No response from Perplexity")?;
    
    eprintln!("✅ Perplexity rephrase successful, result_len={}", rephrased.len());
    
    // Strip any preamble text that Perplexity might add
    let cleaned = strip_preamble(&rephrased);
    eprintln!("✂️  Cleaned preamble: original_len={}, cleaned_len={}", rephrased.len(), cleaned.len());
    
    Ok(cleaned)
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



