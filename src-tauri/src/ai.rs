// AI rephrasing module - OpenAI integration
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::Style;

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

fn get_prompt(text: &str, style: &Style) -> String {
    match style {
        Style::Professional => format!(
            "Rephrase the following text in a professional, formal tone suitable for business communication. \
            Maintain the core message but improve clarity and professionalism. Do not add any preamble or \
            explanation, just return the rephrased text:\n\n{}",
            text
        ),
        Style::Casual => format!(
            "Rephrase the following text in a casual, friendly tone suitable for informal communication. \
            Make it conversational and approachable. Do not add any preamble or explanation, \
            just return the rephrased text:\n\n{}",
            text
        ),
        Style::Sarcasm => format!(
            "Rephrase the following text with subtle sarcasm while maintaining the surface-level message. \
            Keep it witty but not offensive. Do not add any preamble or explanation, \
            just return the rephrased text:\n\n{}",
            text
        ),
    }
}

pub async fn rephrase_with_openai(
    text: &str,
    style: &Style,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let prompt = get_prompt(text, style);
    
    // Calculate appropriate max_tokens based on input length
    // Rule of thumb: 1 token ≈ 4 characters, allow 1.5x for rephrasing
    let estimated_tokens = (text.len() / 4) as u32;
    let max_tokens = (estimated_tokens as f32 * 1.5).ceil() as u32;
    let max_tokens = max_tokens.max(150).min(2000); // Between 150 and 2000 tokens
    
    let request = OpenAIRequest {
        model: "gpt-4o-mini".to_string(), // More cost-effective and faster
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a professional writing assistant that helps rephrase text in different styles. \
                         Always return only the rephrased text without any additional explanation or preamble.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        temperature: 0.7,
        max_tokens,
    };
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(30)) // Longer timeout for large text
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                "Request timed out. The text might be too long or the network is slow.".to_string()
            } else if e.is_connect() {
                "Cannot connect to OpenAI. Please check your internet connection.".to_string()
            } else {
                format!("Network error: {}", e)
            }
        })?;
    
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        let error_msg = match status.as_u16() {
            401 => "Invalid API key. Please check your OpenAI API key in Settings.".to_string(),
            429 => "Rate limit exceeded. Please wait a moment and try again.".to_string(),
            500..=599 => "OpenAI server error. Please try again in a moment.".to_string(),
            _ => format!("OpenAI API error ({}): {}", status, error_text),
        };
        return Err(error_msg.into());
    }
    
    let data: OpenAIResponse = response.json().await?;
    
    let rephrased = data.choices
        .first()
        .and_then(|choice| Some(choice.message.content.trim().to_string()))
        .ok_or("No response from OpenAI")?;
    
    Ok(rephrased)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prompt_generation() {
        let text = "Hello world";
        let prompt = get_prompt(text, &Style::Professional);
        assert!(prompt.contains("Hello world"));
        assert!(prompt.contains("professional"));
    }
}


