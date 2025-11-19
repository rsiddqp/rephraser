# Quick Start Guide

Get Rephraser running in 5 minutes!

## Prerequisites

- macOS 10.15+ or Windows 10+
- Node.js 18+ installed
- 5 minutes of your time

## Step 1: Install Rust (2 minutes)

```bash
# macOS/Linux - paste this in terminal:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Press **1** for default installation, then:

```bash
source ~/.cargo/env
```

**Windows**: Download and run https://rustup.rs/

## Step 2: Install Dependencies (1 minute)

```bash
cd "/Users/rahil/test project"
npm install
```

## Step 3: Run the App (2 minutes)

```bash
npm run tauri dev
```

**First time**: Takes 2-5 minutes to compile. Be patient!

**macOS Users**: Grant accessibility permissions when prompted:
- System Preferences → Security & Privacy → Privacy → Accessibility
- Add your terminal app
- Restart the command

## Step 4: Configure API Key (30 seconds)

1. Get API key from: https://platform.openai.com/api-keys

2. Create config file:
   ```bash
   # macOS:
   mkdir -p ~/Library/Application\ Support/Rephraser
   nano ~/Library/Application\ Support/Rephraser/config.json
   ```

3. Paste this (replace with your key):
   ```json
   {
     "hotkey": "CommandOrControl+R+T",
     "default_style": "professional",
     "api_provider": "openai",
     "api_key": "sk-YOUR-KEY-HERE",
     "theme": "system",
     "start_on_login": false,
     "auto_update": true
   }
   ```

4. Save (Ctrl+X, Y, Enter)

## Step 5: Test It! (30 seconds)

1. App should be running (invisible - that's normal!)
2. Open any text editor (Notes, TextEdit, VS Code, etc.)
3. Type: `hello world this is a test`
4. Select all and copy (Cmd+C or Ctrl+C)
5. Press **Cmd+R+T** (macOS) or **Ctrl+R+T** (Windows)
6. Popup should appear! 🎉

**Try the styles**:
- Click "Professional" → formal business tone
- Click "Casual" → friendly conversational tone  
- Click "Sarcasm" → witty, subtly sarcastic tone

**Replace text**:
- Click "Replace" button
- Original text should be replaced! ✨

---

## 🎯 That's It!

You now have a working AI text rephraser that works system-wide!

**Next Steps**:
- Try it in different apps (Chrome, Slack, email clients)
- Experiment with different text styles
- Read `TESTING.md` for more advanced usage

**Having Issues?**
- Check `SETUP.md` for detailed troubleshooting
- Check `TESTING.md` for common problems
- See `PROJECT_STATUS.md` for known limitations

---

**Pro Tips**:
- Keep the terminal window running (that's your app!)
- Press Cmd/Ctrl+C in terminal to stop the app
- Edit code and it auto-reloads (frontend) or restarts (backend)
- For production build: `npm run tauri build`

Enjoy! 🚀


