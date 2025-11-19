// Accessibility API for capturing selected text from active applications
use std::error::Error;

#[cfg(target_os = "macos")]
pub fn get_selected_text() -> Result<String, Box<dyn Error>> {
    use cocoa::appkit::NSPasteboardTypeString;
    use cocoa::base::{id, nil};
    use cocoa::foundation::{NSAutoreleasePool, NSString};
    use objc::{class, msg_send, sel, sel_impl};

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        
        eprintln!("🔍 Starting text capture...");
        
        // Save current clipboard
        let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
        let old_contents: id = msg_send![pasteboard, stringForType: NSPasteboardTypeString];
        let old_string = if old_contents != nil {
            let c_str = NSString::UTF8String(old_contents);
            if !c_str.is_null() {
                let old = std::ffi::CStr::from_ptr(c_str).to_string_lossy().into_owned();
                eprintln!("💾 Original clipboard saved: {} chars", old.len());
                Some(old)
            } else {
                eprintln!("💾 No original clipboard content");
                None
            }
        } else {
            eprintln!("💾 Clipboard is empty");
            None
        };

        // Simulate Cmd+C to copy selected text
        eprintln!("⌨️  Simulating Cmd+C...");
        simulate_copy_command()?;
        
        // Longer delay to ensure the copy completes
        eprintln!("⏳ Waiting for clipboard update...");
        std::thread::sleep(std::time::Duration::from_millis(200));
        
        // Get the newly copied text
        let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
        let new_contents: id = msg_send![pasteboard, stringForType: NSPasteboardTypeString];
        
        let selected_text = if new_contents != nil {
            let c_str = NSString::UTF8String(new_contents);
            if !c_str.is_null() {
                let text = std::ffi::CStr::from_ptr(c_str).to_string_lossy().into_owned();
                eprintln!("📋 Clipboard after Cmd+C: {} chars", text.len());
                
                // Check if it's the same as original (meaning nothing was selected/copied)
                if let Some(ref old) = old_string {
                    if &text == old {
                        eprintln!("❌ Clipboard unchanged - no text was selected or Cmd+C failed");
                        return Err("No text was copied. Please make sure text is selected, or grant accessibility permissions in System Preferences → Security & Privacy → Accessibility.".into());
                    }
                }
                
                if text.trim().is_empty() {
                    eprintln!("❌ Clipboard contains only whitespace");
                    return Err("Selected text is empty. Please select some text first.".into());
                }
                
                // Safe substring that respects UTF-8 character boundaries
                let preview = text.chars().take(50).collect::<String>();
                eprintln!("✅ Successfully captured: {}...", preview);
                text
            } else {
                eprintln!("❌ Failed to convert clipboard to UTF8 string");
                return Err("Failed to read copied text from clipboard.".into());
            }
        } else {
            eprintln!("❌ Clipboard is nil after Cmd+C");
            return Err("No text was copied. Make sure text is selected or grant accessibility permissions.".into());
        };

        // Restore original clipboard if it was different
        if let Some(old) = old_string {
            if old != selected_text {
                eprintln!("♻️  Restoring original clipboard...");
                let ns_string = NSString::alloc(nil);
                let ns_string = NSString::init_str(ns_string, &old);
                let _: () = msg_send![pasteboard, clearContents];
                let _: () = msg_send![pasteboard, setString:ns_string forType:NSPasteboardTypeString];
                eprintln!("✅ Original clipboard restored");
            }
        }

        Ok(selected_text)
    }
}

#[cfg(target_os = "macos")]
fn simulate_copy_command() -> Result<(), Box<dyn Error>> {
    use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation, CGKeyCode};
    use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

    eprintln!("🎯 Creating event source...");
    let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
        .map_err(|_| {
            eprintln!("❌ Failed to create event source - accessibility permissions may be missing!");
            "Failed to create event source. Please grant accessibility permissions in System Preferences → Security & Privacy → Accessibility."
        })?;
    eprintln!("✅ Event source created");

    // Key code for 'C' is 8
    let key_code: CGKeyCode = 8;

    eprintln!("⌨️  Creating Cmd+C key events...");
    // Create key down event with Cmd modifier
    let key_down = CGEvent::new_keyboard_event(source.clone(), key_code, true)
        .map_err(|_| "Failed to create key down event")?;
    key_down.set_flags(CGEventFlags::CGEventFlagCommand);

    // Create key up event with Cmd modifier
    let key_up = CGEvent::new_keyboard_event(source, key_code, false)
        .map_err(|_| "Failed to create key up event")?;
    key_up.set_flags(CGEventFlags::CGEventFlagCommand);

    eprintln!("📤 Posting Cmd+C events...");
    // Post the events
    key_down.post(CGEventTapLocation::HID);
    key_up.post(CGEventTapLocation::HID);
    eprintln!("✅ Cmd+C events posted");

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn get_selected_text() -> Result<String, Box<dyn Error>> {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_CONTROL, VK_C,
    };
    use windows::Win32::System::DataExchange::{CloseClipboard, GetClipboardData, OpenClipboard};
    use windows::Win32::System::Memory::GlobalLock;
    use std::ptr;

    unsafe {
        // Simulate Ctrl+C
        let mut inputs = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_CONTROL,
                        wScan: 0,
                        dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_C,
                        wScan: 0,
                        dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_C,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_CONTROL,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];

        SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32);

        // Wait for clipboard to update
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Read from clipboard
        if OpenClipboard(None).is_ok() {
            let h_data = GetClipboardData(1); // CF_TEXT = 1
            if !h_data.is_invalid() {
                let p_data = GlobalLock(h_data.0);
                if !p_data.is_null() {
                    let c_str = std::ffi::CStr::from_ptr(p_data as *const i8);
                    let text = c_str.to_string_lossy().into_owned();
                    CloseClipboard().ok();
                    return Ok(text);
                }
            }
            CloseClipboard().ok();
        }

        Err("Failed to read clipboard".into())
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn get_selected_text() -> Result<String, Box<dyn Error>> {
    Err("Platform not supported".into())
}

