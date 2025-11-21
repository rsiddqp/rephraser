// Core Tauri application entry point
use tauri::{AppHandle, Manager, Window};
use serde::{Deserialize, Serialize};

mod ai;
mod config;
mod accessibility;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionInfo {
    pub text: String,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Style {
    Professional,
    Casual,
    Sarcasm,
}

#[tauri::command]
async fn rephrase_text(
    text: String,
    style: Style,
    provider: String,
    api_key: String,
) -> Result<String, String> {
    // Validate text length
    const MAX_TEXT_LENGTH: usize = 10000; // ~2500 tokens
    if text.len() > MAX_TEXT_LENGTH {
        return Err(format!("Text too long. Maximum {} characters allowed.", MAX_TEXT_LENGTH));
    }
    
    if text.trim().is_empty() {
        return Err("Text cannot be empty".to_string());
    }
    
    if api_key.trim().is_empty() {
        return Err("API key is required. Please configure it in Settings.".to_string());
    }
    
    ai::rephrase_text(&text, &style, &provider, &api_key)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_clipboard_text(app: AppHandle) -> Result<String, String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    
    app.clipboard()
        .read_text()
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_selected_text() -> Result<String, String> {
    let text = accessibility::get_selected_text()
        .map_err(|e| e.to_string())?;
    
    // Trim whitespace and validate
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("No text selected".to_string());
    }
    
    Ok(trimmed.to_string())
}

#[tauri::command]
async fn set_clipboard_text(app: AppHandle, text: String) -> Result<(), String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    
    app.clipboard()
        .write_text(text)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn copy_to_clipboard(
    app: AppHandle,
    text: String,
) -> Result<(), String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    
    if text.is_empty() {
        return Err("Cannot copy empty text".to_string());
    }
    
    app.clipboard()
        .write_text(text)
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))
}

#[tauri::command]
async fn show_popup_at_cursor(window: Window) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use core_graphics::display::{CGDisplay, CGPoint};
        
        // Get mouse cursor position
        let cursor_pos = {
            let source = core_graphics::event_source::CGEventSource::new(
                core_graphics::event_source::CGEventSourceStateID::CombinedSessionState
            ).map_err(|_| "Failed to get event source")?;
            
            let event = core_graphics::event::CGEvent::new(source.clone())
                .map_err(|_| "Failed to create event")?;
            event.location()
        };
        
        // Get the display that contains the cursor (not just main display)
        let cursor_point = CGPoint::new(cursor_pos.x, cursor_pos.y);
        let displays = CGDisplay::active_displays().map_err(|_| "Failed to get active displays")?;
        
        let screen_bounds = displays.iter()
            .find_map(|&display_id| {
                let display = CGDisplay::new(display_id);
                let bounds = display.bounds();
                // Check if cursor is within this display's bounds
                if cursor_point.x >= bounds.origin.x && 
                   cursor_point.x < bounds.origin.x + bounds.size.width &&
                   cursor_point.y >= bounds.origin.y && 
                   cursor_point.y < bounds.origin.y + bounds.size.height {
                    Some(bounds)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| CGDisplay::main().bounds());
        
        // Window dimensions (from tauri.conf.json)
        const WINDOW_WIDTH: i32 = 500;
        const WINDOW_HEIGHT: i32 = 600;
        const PADDING: i32 = 20;
        
        // Position window ABOVE the cursor to avoid cutting off bottom
        // Offset to the left so it's centered horizontally around cursor
        let mut window_x = (cursor_pos.x as i32) - (WINDOW_WIDTH / 2);
        let mut window_y = (cursor_pos.y as i32) - WINDOW_HEIGHT - PADDING;
        
        // Bounds checking - ensure window stays on screen
        
        // Check left edge
        if window_x < screen_bounds.origin.x as i32 + PADDING {
            window_x = screen_bounds.origin.x as i32 + PADDING;
        }
        
        // Check right edge
        let screen_right = (screen_bounds.origin.x + screen_bounds.size.width) as i32;
        if window_x + WINDOW_WIDTH > screen_right - PADDING {
            window_x = screen_right - WINDOW_WIDTH - PADDING;
        }
        
        // Check top edge - if window would go above screen, show BELOW cursor instead
        if window_y < screen_bounds.origin.y as i32 + PADDING {
            eprintln!("⚠️  Window would go off top of screen, positioning below cursor instead");
            window_y = cursor_pos.y as i32 + PADDING;
        }
        
        // Check bottom edge - make sure it doesn't go below screen
        let screen_bottom = (screen_bounds.origin.y + screen_bounds.size.height) as i32;
        if window_y + WINDOW_HEIGHT > screen_bottom - PADDING {
            // If it goes below screen, position it ABOVE cursor
            window_y = (cursor_pos.y as i32) - WINDOW_HEIGHT - PADDING;
            // If still off screen, clamp to top
            if window_y < screen_bounds.origin.y as i32 + PADDING {
                window_y = screen_bounds.origin.y as i32 + PADDING;
            }
        }
        
        eprintln!("📍 Cursor at: ({}, {})", cursor_pos.x, cursor_pos.y);
        eprintln!("📐 Screen bounds: {}x{} at ({}, {})", 
            screen_bounds.size.width, screen_bounds.size.height,
            screen_bounds.origin.x, screen_bounds.origin.y);
        eprintln!("🪟 Positioning window at: ({}, {})", window_x, window_y);
        
        window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
            x: window_x,
            y: window_y,
        })).map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
        use windows::Win32::Foundation::POINT;
        
        unsafe {
            // Get cursor position
            let mut cursor_pos = POINT { x: 0, y: 0 };
            GetCursorPos(&mut cursor_pos).ok();
            
            // Get screen dimensions
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);
            
            const WINDOW_WIDTH: i32 = 500;
            const WINDOW_HEIGHT: i32 = 600;
            const PADDING: i32 = 20;
            
            // Position ABOVE cursor, centered horizontally
            let mut window_x = cursor_pos.x - (WINDOW_WIDTH / 2);
            let mut window_y = cursor_pos.y - WINDOW_HEIGHT - PADDING;
            
            // Bounds checking
            if window_x < PADDING {
                window_x = PADDING;
            }
            if window_x + WINDOW_WIDTH > screen_width - PADDING {
                window_x = screen_width - WINDOW_WIDTH - PADDING;
            }
            if window_y < PADDING {
                window_y = cursor_pos.y + PADDING; // Show below if can't fit above
            }
            if window_y + WINDOW_HEIGHT > screen_height - PADDING {
                window_y = cursor_pos.y - WINDOW_HEIGHT - PADDING;
                if window_y < PADDING {
                    window_y = PADDING;
                }
            }
            
            window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x: window_x,
                y: window_y,
            })).map_err(|e| e.to_string())?;
        }
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        // For Linux, center on screen
        window.center().map_err(|e| e.to_string())?;
    }
    
    // Show and focus window
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    window.set_always_on_top(true).map_err(|e| e.to_string())?;
    
    Ok(())
}


#[tauri::command]
async fn hide_popup(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
fn load_config() -> Result<config::AppConfig, String> {
    config::load().map_err(|e| e.to_string())
}

#[tauri::command]
fn save_config(config: config::AppConfig) -> Result<(), String> {
    config::save(&config).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            rephrase_text,
            get_clipboard_text,
            get_selected_text,
            set_clipboard_text,
            copy_to_clipboard,
            show_popup_at_cursor,
            hide_popup,
            load_config,
            save_config,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            println!("Rephraser started successfully in development mode!");
            
            // Check if config exists, if not create with bundled default
            if let Err(_) = config::load() {
                // Try to load bundled default config for testing
                if let Some(resource_path) = app.path().resource_dir().ok() {
                    let bundled_config = resource_path.join("default-config.json");
                    if bundled_config.exists() {
                        if let Ok(content) = std::fs::read_to_string(&bundled_config) {
                            if let Ok(default_config) = serde_json::from_str::<config::AppConfig>(&content) {
                                let _ = config::save(&default_config);
                                println!("✅ Loaded bundled API key for testing");
                            }
                        }
                    }
                }
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
