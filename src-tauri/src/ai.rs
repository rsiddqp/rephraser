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
    custom_prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    match provider.to_lowercase().as_str() {
        "proxy" => rephrase_with_proxy(text, style, custom_prompt).await,
        "openai" => rephrase_with_openai(text, style, api_key, custom_prompt).await,
        "claude" | "anthropic" => rephrase_with_claude(text, style, api_key, custom_prompt).await,
        "gemini" | "google" => rephrase_with_gemini(text, style, api_key, custom_prompt).await,
        "perplexity" => rephrase_with_perplexity(text, style, api_key, custom_prompt).await,
        _ => Err(format!("Unsupported provider: {}", provider).into()),
    }
}

fn get_prompt_for_style(text: &str, style: &Style, custom_prompt: &str) -> String {
    if !custom_prompt.is_empty() {
        let instruction = format!(
            "{} IMPORTANT: Return ONLY the rephrased text, without any introduction, explanation, or preamble.",
            custom_prompt
        );
        return format!("{}\n\nText: {}", instruction, text);
    }
    
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

// Proxy server integration (default - uses server-side API key)
async fn rephrase_with_proxy(
    text: &str,
    style: &Style,
    custom_prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("🌐 Using proxy server for rephrasing");
    let client = Client::new();
    
    // Try Heroku first, fall back to Render
    const PROXY_URL_PRIMARY: &str = "https://rephraser-technology-21cddf6fbfbc.herokuapp.com/api/rephrase";
    const PROXY_URL_FALLBACK: &str = "https://rephraser-9ur5.onrender.com/api/rephrase";
    
    let proxy_url = std::env::var("REPHRASER_PROXY_URL")
        .unwrap_or_else(|_| PROXY_URL_PRIMARY.to_string());
    
    // When custom_prompt is set, embed the instruction into the text and use
    // "professional" as the style. The proxy wraps text in its own style prompt,
    // so we prepend an explicit override to ensure the custom instruction takes
    // precedence over the proxy's default "professional" instruction.
    let (effective_text, style_str) = if !custom_prompt.is_empty() {
        let full = format!(
            "[OVERRIDE: Ignore the style instruction above. Instead follow these instructions: {}]\n\n{}",
            custom_prompt, text
        );
        (full, "professional")
    } else {
        (text.to_string(), match style {
            Style::Professional => "professional",
            Style::Casual => "casual",
            Style::Sarcasm => "sarcasm",
        })
    };
    
    #[derive(Serialize)]
    struct ProxyRequest {
        text: String,
        style: String,
    }
    
    #[derive(Deserialize)]
    struct ProxyResponse {
        rephrased: String,
    }
    
    let request_body = ProxyRequest {
        text: effective_text,
        style: style_str.to_string(),
    };
    
    eprintln!("📤 Sending request to proxy: url={}, style={}, text_len={}", proxy_url, style_str, request_body.text.len());
    
    const CLIENT_ID: &str = "desktop/0.1.0";
    
    let response = client
        .post(&proxy_url)
        .header("Content-Type", "application/json")
        .header("X-Rephraser-Client", CLIENT_ID)
        .json(&request_body)
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .await;
    
    // If primary fails with a connection error, try fallback
    let response = match response {
        Ok(r) => r,
        Err(e) if e.is_connect() && proxy_url != PROXY_URL_FALLBACK => {
            eprintln!("⚠️  Primary proxy unreachable, trying fallback: {}", PROXY_URL_FALLBACK);
            client
                .post(PROXY_URL_FALLBACK)
                .header("Content-Type", "application/json")
                .header("X-Rephraser-Client", CLIENT_ID)
                .json(&request_body)
                .timeout(std::time::Duration::from_secs(60))
                .send()
                .await
                .map_err(|e| {
                    eprintln!("❌ Fallback proxy also failed: {:?}", e);
                    handle_request_error(e)
                })?
        }
        Err(e) => {
            eprintln!("❌ Proxy request failed: {:?}", e);
            return Err(handle_request_error(e).into());
        }
    };
    
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

async fn rephrase_with_openai(
    text: &str,
    style: &Style,
    api_key: &str,
    custom_prompt: &str,
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
                content: get_prompt_for_style(text, style, custom_prompt),
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

async fn rephrase_with_claude(
    text: &str,
    style: &Style,
    api_key: &str,
    custom_prompt: &str,
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
    
    let request = ClaudeRequest {
        model: "claude-sonnet-4-6".to_string(),
        max_tokens: 2048,
        messages: vec![
            ClaudeMessage {
                role: "user".to_string(),
                content: get_prompt_for_style(text, style, custom_prompt),
            },
        ],
    };
    
    eprintln!("📤 Sending request to Claude API: model=claude-sonnet-4-6, text_len={}", text.len());
    
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

async fn rephrase_with_gemini(
    text: &str,
    style: &Style,
    api_key: &str,
    custom_prompt: &str,
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
                        text: get_prompt_for_style(text, style, custom_prompt),
                    },
                ],
            },
        ],
    };
    
    let url = format!("https://generativelanguage.googleapis.com/v1/models/gemini-2.5-flash:generateContent?key={}", api_key);
    
    eprintln!("📤 Sending request to Gemini: model=gemini-2.5-flash, text_len={}", text.len());
    
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

async fn rephrase_with_perplexity(
    text: &str,
    style: &Style,
    api_key: &str,
    custom_prompt: &str,
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
    
    let request = PerplexityRequest {
        model: "sonar".to_string(),
        messages: vec![
            PerplexityMessage {
                role: "user".to_string(),
                content: get_prompt_for_style(text, style, custom_prompt),
            },
        ],
    };
    
    eprintln!("📤 Sending request to Perplexity: model=sonar, text_len={}", text.len());
    
    let response = client
        .post("https://api.perplexity.ai/v1/sonar")
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
        "Request timed out. The AI service may be slow — please try again.".to_string()
    } else if e.is_connect() {
        format!(
            "Cannot connect to API server. This may be caused by a network issue \
             or the app's security policy blocking the request. \
             Please check your internet connection. (Details: {})",
            e
        )
    } else if e.is_request() {
        format!(
            "Failed to send request. The API domain may not be allowed by the app's \
             security policy. Please update the app or contact support. (Details: {})",
            e
        )
    } else {
        format!("Network error: {}", e)
    }
}

fn handle_api_error(status: u16, provider: &str) -> String {
    match status {
        400 => format!(
            "{}: Bad request. The text may be too long or contain unsupported characters.",
            provider
        ),
        401 => format!(
            "{}: Invalid API key. Please verify your key in Settings and ensure it has not expired.",
            provider
        ),
        403 => format!(
            "{}: Access denied. Your API key may lack the required permissions, \
             or the model may not be available on your plan.",
            provider
        ),
        404 => format!(
            "{}: Model or endpoint not found. The model may have been deprecated — \
             please check for app updates.",
            provider
        ),
        429 => format!(
            "{}: Rate limit exceeded. Please wait a moment and try again, \
             or check your API plan's usage limits.",
            provider
        ),
        500..=599 => format!(
            "{}: Service temporarily unavailable (HTTP {}). Please try again in a few seconds.",
            provider, status
        ),
        _ => format!(
            "{}: Unexpected error (HTTP {}). Please try again or check your API key.",
            provider, status
        ),
    }
}



