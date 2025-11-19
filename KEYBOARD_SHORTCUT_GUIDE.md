# Keyboard Shortcut Implementation Guide

## Overview
The Rephraser app now has **complete automatic text selection and rephrasing functionality** implemented with `Control+Space+R` as the default hotkey. This is the critical feature that makes the app truly usable - **no manual copying required!**

## What Changed

### 1. **Keyboard Shortcut: Control+Space+R**
- Changed from the previous `Cmd+R+T` / `Ctrl+R+T` 
- Now uses `Control+Space+R` (works on both macOS and Windows)
- Configured as default in the Rust backend (`src-tauri/src/config.rs`)

### 2. **Automatic Hotkey Registration**
- The app now automatically registers the global keyboard shortcut on startup
- Implemented in `src/App.tsx` using the `@tauri-apps/plugin-global-shortcut` plugin
- Hotkey is unregistered when the app closes (cleanup)

### 3. **Automatic Text Selection Capture (NEW!)**
- **No manual copying required!** Just select text and press the hotkey
- Uses macOS Accessibility API to automatically capture selected text
- Works with Cmd+A (select all) or any text selection
- Temporarily simulates Cmd+C internally and restores your clipboard

### 4. **Workflow Implementation**
The complete workflow is now functional:

1. **SELECT text** anywhere (highlight it or press Cmd+A)
2. **Press keyboard shortcut**: `Control+Space+R`
3. **App automatically**:
   - Captures the selected text (no manual copy needed!)
   - Sends it to OpenAI for rephrasing
   - Shows the result in the current style (Professional/Casual/Sarcasm)
4. **Review and copy** the rephrased text
5. **Paste it** wherever you need it

## How to Test

### Prerequisites
1. Rust must be installed (check with `rustc --version`)
2. OpenAI API key configured in Settings

### Testing Steps

1. **Start the development server:**
   ```bash
   cd "/Users/rahil/test project"
   npm run tauri dev
   ```

2. **Grant Accessibility Permissions (CRITICAL for macOS):**
   - System Preferences → Security & Privacy → Privacy → Accessibility
   - Click the lock to make changes
   - Add your Terminal app (or IDE) to the list
   - Restart the app after granting permissions

3. **Configure your API key:**
   - Click the Settings icon in the app
   - Enter your OpenAI API key
   - Click Save

4. **Test the keyboard shortcut:**
   - Open any text editor (Notes, TextEdit, VS Code, etc.)
   - Type some text: "hello this is a test message"
   - **SELECT the text** (highlight it with your mouse or press Cmd+A)
   - **DO NOT COPY IT** - just select it!
   - Press the keyboard shortcut: **Control+Space+R**
   - The app should automatically:
     - Capture the selected text (you'll see it appear in the app)
     - Rephrase the text
     - Show the result in the app window

4. **Try different styles:**
   - After rephrasing, click "Professional", "Casual", or "Sarcasm"
   - The text will be re-rephrased in that style
   - Click the "Copy" button to copy the result to clipboard
   - Paste it back into your editor

5. **Test regeneration:**
   - Click the "Rephrase" button (🔄 icon) to get alternative phrasings
   - Each click generates a new version with the same style

## Files Modified

### Backend (Rust)
- `src-tauri/src/accessibility.rs`: **NEW FILE** - Automatic text selection capture using Accessibility APIs
- `src-tauri/src/lib.rs`: Added `get_selected_text` command
- `src-tauri/src/config.rs`: Changed default hotkey to "Control+Space+R"
- `src-tauri/Cargo.toml`: Added `core-graphics` (macOS) and `windows` (Windows) dependencies
- `src-tauri/Info.plist`: **NEW FILE** - macOS accessibility permissions descriptions
- `src-tauri/capabilities/default.json`: Added permissions for global-shortcut and clipboard-manager plugins

### Frontend (TypeScript/React)
- `src/App.tsx`: 
  - Added hotkey registration on component mount
  - Implemented automatic text capture from **SELECTED TEXT** (not clipboard!)
  - Automatic rephrasing when hotkey is pressed
  - Cleanup/unregister on unmount

### Documentation
- `README.md`: Updated usage instructions and keyboard shortcut references
- `SETUP.md`: Updated testing guide and configuration examples
- `memory-bank/*.md`: Updated all references to the old hotkey
  - `projectbrief.md`
  - `productContext.md`
  - `activeContext.md`
  - `systemPatterns.md`
  - `progress.md`

### Dependencies
- Added `@tauri-apps/plugin-global-shortcut` to package.json
- Added `@tauri-apps/plugin-clipboard-manager` to package.json

## Troubleshooting

### "Hotkey not working"
- **macOS**: Grant accessibility permissions
  - System Preferences → Security & Privacy → Privacy → Accessibility
  - Add your Terminal/IDE app
  - Restart the app

- **Check console**: Look for "Hotkey triggered!" message when you press the shortcut
- **Try unregistering**: If hotkey seems stuck, restart the app

### "No text selected" error
- Make sure you've **selected/highlighted** text before pressing the hotkey
- The app now reads **selected text directly** - no manual copying needed!
- Try using Cmd+A (select all) or highlight text with your mouse

### "API key not configured" error
- Open Settings (gear icon)
- Enter your OpenAI API key from https://platform.openai.com/api-keys
- Click Save

## Next Steps

1. **Test extensively** with different text lengths and styles
2. **Verify** the hotkey works across different applications
3. **Report any issues** with specific apps where it doesn't work
4. Consider adding:
   - Visual feedback when hotkey is pressed
   - Notification/toast when rephrasing is complete
   - Option to change the hotkey in Settings UI

## Success Criteria

✅ Keyboard shortcut registers on app startup
✅ Hotkey triggers text capture from **SELECTED TEXT** (no manual copy!)
✅ Works with Cmd+A (select all) or any text selection
✅ Automatic rephrasing happens when hotkey is pressed
✅ Different styles can be selected and re-rephrased
✅ Regeneration works to get alternative phrasings
✅ Copy button works to get result back to clipboard
✅ Hotkey unregisters cleanly on app close
✅ Original clipboard is preserved (temporary Cmd+C is invisible to user)

---

**This implementation makes the app truly seamless!** The workflow is now: **SELECT text → keyboard shortcut → get revised text**. No manual copying required!

