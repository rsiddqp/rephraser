# Testing Guide: Rephraser

## Prerequisites

Before testing, ensure you have:

1. **Rust Toolchain** (required for Tauri)
   ```bash
   # Install Rust via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Node.js** 18+ or 20+ LTS
   ```bash
   node --version  # Should be v18.x or v20.x
   npm --version
   ```

3. **OpenAI API Key**
   - Get one from https://platform.openai.com/api-keys
   - Required for text rephrasing functionality

## Setup

1. **Install Dependencies**
   ```bash
   cd /Users/rahil/test\ project
   npm install
   ```

2. **Verify Frontend Builds**
   ```bash
   npm run build
   # Should complete without errors
   ```

3. **Run Development Server**
   ```bash
   npm run tauri dev
   ```
   
   **First-time permissions (macOS only)**:
   - System will ask for Accessibility permissions
   - Go to: System Preferences → Security & Privacy → Privacy → Accessibility
   - Add "Rephraser" or the terminal app you're running from
   - Restart the app after granting permissions

## Test Plan

### 1. Initial Launch Test
- [ ] Application window is invisible on launch (as expected)
- [ ] No errors in terminal
- [ ] System tray icon appears (if implemented)

### 2. Global Hotkey Test
- [ ] Open any text editor (Notes, TextEdit, VS Code, Chrome, etc.)
- [ ] Type or select some text: "hello this is a test"
- [ ] Copy to clipboard (Cmd+C or Ctrl+C)
- [ ] Press hotkey: **Cmd+R+T** (macOS) or **Ctrl+R+T** (Windows)
- [ ] Popup window should appear near center of screen
- [ ] Popup should show the selected text in "Original" section

### 3. API Configuration Test
**If API key not configured:**
- [ ] Settings button or prompt appears
- [ ] Enter OpenAI API key
- [ ] Click Save
- [ ] Config saved to: `~/Library/Application Support/Rephraser/config.json` (macOS)

**If API key already configured:**
- [ ] Rephrasing should start automatically

### 4. Style Selector Test
With popup visible and text loaded:
- [ ] Click "Professional" button
  - Should highlight (blue background)
  - Should trigger rephrasing
  - Output should be formal/business-like
- [ ] Click "Casual" button
  - Should highlight
  - Should trigger rephrasing
  - Output should be friendly/conversational
- [ ] Click "Sarcasm" button
  - Should highlight
  - Should trigger rephrasing
  - Output should be witty/subtly sarcastic

### 5. Rephrasing Test
- [ ] Loading spinner appears while waiting for API
- [ ] Rephrased text appears in "Rephrased" section
- [ ] Text is different from original
- [ ] Text maintains core meaning
- [ ] No errors displayed

### 6. Rephrase Button Test
- [ ] Click the "Rephrase" button (refresh icon)
- [ ] Should generate a different version
- [ ] Multiple clicks should produce variations
- [ ] Loading state shows during regeneration

### 7. Replace Button Test
**Critical Test - Text Replacement**
- [ ] Select text in a text editor
- [ ] Copy to clipboard (Cmd+C)
- [ ] Press hotkey (Cmd+R+T)
- [ ] Wait for rephrasing
- [ ] Click "Replace" button
- [ ] **Expected**: Original text should be replaced with rephrased text
- [ ] **Expected**: Popup should close automatically
- [ ] **Expected**: Cursor should remain in the text field

**If Replace doesn't work:**
- Check terminal for errors
- The clipboard paste simulation may need adjustment
- See Known Issues section below

### 8. Close Button Test
- [ ] Click X button in popup
- [ ] Popup should disappear
- [ ] Original text should remain unchanged
- [ ] Can invoke popup again with hotkey

### 9. Multi-Application Test
Test in various applications:
- [ ] **Browser** (Chrome/Firefox/Safari)
  - Select text in address bar
  - Select text in Gmail/web email
  - Select text in any textarea
- [ ] **Notes App** (macOS Notes or Notepad on Windows)
- [ ] **VS Code / Text Editor**
- [ ] **Slack / Discord / Messaging App**

### 10. Error Handling Test
- [ ] **No text selected**: Press hotkey without selecting text
  - Should show error or do nothing gracefully
- [ ] **Invalid API key**: Configure wrong API key
  - Should show error message
  - Should not crash
- [ ] **Network offline**: Disconnect internet, try rephrasing
  - Should show timeout error after 10 seconds
  - Should not crash

### 11. Edge Cases
- [ ] **Very short text**: Single word like "hello"
  - Should rephrase to synonym
- [ ] **Very long text**: Copy 2-3 paragraphs
  - Should handle without crashing
  - May be slow (acceptable)
- [ ] **Special characters**: Text with emojis, symbols, unicode
  - Should preserve or handle gracefully
- [ ] **Multiple languages**: Non-English text (e.g., Spanish)
  - May not work well (expected limitation for MVP)

## Known Issues & Workarounds

### Issue 1: Text Replacement Not Working
**Symptom**: Replace button doesn't update the original text

**Likely Cause**: Clipboard paste simulation may not work in all apps

**Workaround**:
1. Note the rephrased text
2. Manually paste it (Cmd+V) after copying from popup
3. Or: Add `enigo` crate for more reliable key simulation

**Debug**:
- Check terminal for errors from `clipboard_ops.rs`
- Check if accessibility permissions are granted
- Try in different applications (some may block automation)

### Issue 2: Popup Doesn't Appear
**Symptom**: Hotkey pressed but nothing happens

**Possible Causes**:
1. Hotkey conflict with another app
   - Try changing hotkey in config.json
2. No text in clipboard
   - Make sure to copy text first (Cmd+C)
3. Accessibility permissions not granted
   - Check System Preferences → Privacy → Accessibility

**Debug**:
- Check terminal for errors
- Verify hotkey is registered: should see message on app start

### Issue 3: API Errors
**Symptom**: "Failed to rephrase text" or timeout errors

**Possible Causes**:
1. Invalid API key
2. Rate limit exceeded (OpenAI limits)
3. Network issues
4. API endpoint down

**Debug**:
- Verify API key works in OpenAI Playground
- Check OpenAI status page
- Try again after a few minutes

### Issue 4: Slow Response Time
**Symptom**: Takes >5 seconds to rephrase

**Expected**: GPT-4 Turbo typically responds in 0.5-2 seconds

**Possible Causes**:
- Network latency
- OpenAI API load
- Very long text (>500 words)

**Acceptable**: Up to 5 seconds for long text

## Performance Benchmarks

Target performance (to verify):

| Metric | Target | Acceptable | Poor |
|--------|--------|------------|------|
| Hotkey → Popup | <150ms | <500ms | >1s |
| API Response | <1s | <3s | >5s |
| Text Replace | <200ms | <500ms | >1s |
| Total Flow | <2s | <4s | >6s |

## Reporting Issues

When reporting issues, include:

1. **OS & Version**: macOS 14.0, Windows 11, etc.
2. **Application**: Which app you were testing in
3. **Steps to Reproduce**: Exact steps that triggered the issue
4. **Expected vs Actual**: What should happen vs what happened
5. **Terminal Output**: Any error messages
6. **Screenshot**: If UI issue

## Success Criteria

The MVP is considered successful if:

- [x] Frontend builds without errors ✅
- [ ] Backend compiles without errors (requires Rust)
- [ ] Global hotkey triggers popup
- [ ] All three styles produce different outputs
- [ ] Text replacement works in ≥2 different applications
- [ ] No crashes during normal usage
- [ ] Performance meets "Acceptable" benchmarks

## Next Steps After Testing

1. **Fix critical bugs** found during testing
2. **Implement Windows support** (clipboard/key simulation)
3. **Add accessibility API** for true text capture (not just clipboard)
4. **Improve positioning** (popup near cursor, not center)
5. **Add keychain encryption** for API keys
6. **Optimize performance** (caching, reduce latency)
7. **Build production binaries** (`npm run tauri build`)

---

**Ready to Test?**

```bash
# Start the app
npm run tauri dev

# In another terminal, watch for errors
# Then follow the test plan above
```

Good luck! 🚀


