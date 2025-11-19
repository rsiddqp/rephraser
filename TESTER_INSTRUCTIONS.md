# Rephraser - Installation Instructions for Testers

## macOS Installation (Unsigned App)

Since this is a test build without code signing, you'll need to bypass macOS Gatekeeper.

### Method 1: Using the ZIP file (Recommended)

1. **Download and extract** `Rephraser.zip`
2. **Right-click** on `Rephraser.app` → Select **"Open"**
3. Click **"Open"** again when macOS warns about unsigned app
4. Done! The app will now open normally

### Method 2: Remove quarantine attribute

If the ZIP method doesn't work:

```bash
# Open Terminal and run:
cd ~/Downloads  # or wherever you extracted the app
xattr -cr Rephraser.app
```

Then open the app normally.

### Method 3: System Settings

1. Try to open `Rephraser.app`
2. When it says "damaged", go to **System Settings** → **Privacy & Security**
3. Scroll down, you'll see "Rephraser was blocked"
4. Click **"Open Anyway"**

---

## ⚙️ Setup (Required)

### 1. Grant Accessibility Permissions

**CRITICAL**: The app needs this to capture selected text.

1. **System Preferences** → **Security & Privacy** → **Privacy** → **Accessibility**
2. Click 🔒 lock icon (enter your password)
3. Click **+** button
4. Navigate to Applications and add **Rephraser.app**
5. Make sure the checkbox next to Rephraser is ✅ checked

### 2. First Launch

- The app will open with the API key already configured (no setup needed!)
- You're ready to use it immediately

---

## 🚀 How to Use

### The Workflow (Super Fast!)

1. **Select text** anywhere (TextEdit, Chrome, Slack, etc.)
   - Highlight text with mouse, or press **Cmd+A** to select all

2. **Press the keyboard shortcut**: **Cmd+Shift+R**
   - Window pops up above your selected text
   - Text is automatically captured and rephrased

3. **Review the rephrased text**
   - Choose style: Professional / Casual / Sarcasm
   - Click "Rephrase" button for alternative versions

4. **Copy and continue working**
   - Click "Copy" button OR press **Cmd+C**
   - Window instantly disappears
   - Paste the rephrased text wherever you need it

### Example:

**Original:** 
> hey can we push this to next week im swamped

**After Cmd+Shift+R (Professional style):**
> Could you please inform me of your availability this week? Regardless of your current interest level, I am eager to provide further details on the project in development.

**Press Cmd+C** → Window closes → Paste the result!

---

## 🎨 Features

- **3 Styles**: Professional, Casual, Sarcasm
- **System-wide**: Works in ANY app
- **Smart positioning**: Appears on your active screen, above selected text
- **Auto-scroll**: Result is always visible
- **One-click copy & dismiss**: Cmd+C copies and closes instantly
- **No data logging**: Your text is never stored

---

## ⌨️ Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Trigger rephrasing | **Cmd+Shift+R** |
| Copy result & dismiss | **Cmd+C** |
| Close window | **Esc** (or click outside) |

---

## 🐛 Troubleshooting

### "Nothing happens when I press Cmd+Shift+R"

**Solution**: Grant accessibility permissions
- System Preferences → Security & Privacy → Privacy → Accessibility
- Add Rephraser.app to the list
- Restart the app

### "No text selected" error

**Solution**: Make sure text is actually selected/highlighted before pressing the shortcut

### Window appears on wrong screen

The app should detect your active screen automatically. If not, try:
- Moving your cursor to the screen you want
- Selecting text on that screen
- Then press the shortcut

### "Failed to rephrase text"

Check your internet connection. The app uses OpenAI's API which requires internet.

---

## 💡 Tips

- Works great with **Cmd+A** (select all) for full paragraphs
- Try the **Rephrase button** multiple times for different variations
- Switch between styles to see different tones
- The window auto-scrolls to show the rephrased result

---

## 🔒 Privacy

- Your API usage is tracked under the shared test API key
- Text is sent to OpenAI for processing (not stored by Rephraser)
- No analytics or tracking in the app
- Original clipboard is always restored after text capture

---

## 📧 Feedback

Report issues or suggestions to: [your email/slack/contact]

---

**Enjoy testing Rephraser!** 🎉

