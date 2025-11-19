// AI rephrasing module - Proxy integration
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::Style;

pub async fn rephrase_with_openai(
    text: &str,
    style: &Style,
    _api_key: &str, // API key not used anymore - kept for compatibility
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // Use proxy server instead of calling OpenAI directly
    const PROXY_URL: &str = "https://rephraser-9ur5.onrender.com/api/rephrase";
    
    let style_str = match style {
        Style::Professional => "professional",
        Style::Casual => "casual",
        Style::Sarcasm => "sarcasm",
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
    
    let request = ProxyRequest {
        text: text.to_string(),
        style: style_str.to_string(),
    };
    
    let response = client
        .post(PROXY_URL)
        .header("Content-Type", "application/json")
        .json(&request)
        .timeout(std::time::Duration::from_secs(60)) // Increased for cold starts
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                "Request timed out. The text might be too long or the network is slow.".to_string()
            } else if e.is_connect() {
                "Cannot connect to server. Please check your internet connection.".to_string()
            } else {
                format!("Network error: {}", e)
            }
        })?;
    
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        let error_msg = match status.as_u16() {
            429 => "Service is busy. Please wait a moment and try again.".to_string(),
            500..=599 => "Service temporarily unavailable. Please try again in a moment.".to_string(),
            _ => format!("Service error ({}): {}", status, error_text),
        };
        return Err(error_msg.into());
    }
    
    let data: ProxyResponse = response.json().await?;
    
    Ok(data.rephrased.trim().to_string())
}



