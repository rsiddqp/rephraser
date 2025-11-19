# Setup Guide: Rephraser Development Environment

This guide will help you set up the development environment for Rephraser from scratch.

## Prerequisites Check

Before starting, verify you have the following installed:

```bash
# Check Node.js (required: 18+ or 20+)
node --version

# Check npm
npm --version

# Check Rust (if not installed, follow steps below)
rustc --version
cargo --version
```

## Step 1: Install Rust (if not already installed)

Rust is required for Tauri to compile the native backend.

### macOS / Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts (press 1 for default installation).

After installation completes:
```bash
source ~/.cargo/env  # or restart your terminal
rustc --version      # verify installation
```

### Windows
Download and run: https://rustup.rs/

Follow the installer instructions.

After installation, restart your terminal and verify:
```bash
rustc --version
cargo --version
```

## Step 2: Install Platform-Specific Dependencies

### macOS
Install Xcode Command Line Tools (if not already installed):
```bash
xcode-select --install
```

No additional dependencies required for macOS.

### Windows
Install:
1. **Microsoft Visual Studio C++ Build Tools**
   - Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
   - Install "Desktop development with C++" workload

2. **WebView2** (usually pre-installed on Windows 10/11)
   - If needed: https://developer.microsoft.com/en-us/microsoft-edge/webview2/

### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

## Step 3: Clone/Navigate to Project

```bash
cd "/Users/rahil/test project"  # or your project path
```

## Step 4: Install Node Dependencies

```bash
npm install
```

This will install:
- React 19
- Tauri API v2
- Tailwind CSS v3
- Zustand (state management)
- Lucide React (icons)
- All other dependencies

**Expected output**: ~245 packages installed

## Step 5: Verify Rust Dependencies

Rust dependencies are defined in `src-tauri/Cargo.toml` and will be downloaded automatically on first build.

To pre-download (optional):
```bash
cd src-tauri
cargo fetch
cd ..
```

## Step 6: Verify Frontend Build

```bash
npm run build
```

**Expected output**:
```
✓ built in ~1s
dist/index.html                   0.47 kB
dist/assets/index-*.css          11.59 kB
dist/assets/index-*.js          201.86 kB
```

If this succeeds, your frontend is working! ✅

## Step 7: First Run (Development Mode)

```bash
npm run tauri dev
```

**First-time compilation**:
- Will take 2-5 minutes (compiling Rust dependencies)
- Subsequent runs are much faster (<30 seconds)

**What should happen**:
1. Rust backend compiles
2. Vite dev server starts on http://localhost:1420
3. Application window opens (invisible initially - this is correct!)
4. Terminal shows: "Listening on http://localhost:1420"

### macOS: Accessibility Permissions

On first run, you'll need to grant accessibility permissions:

1. **System Preferences** → **Security & Privacy** → **Privacy** → **Accessibility**
2. Click the lock to make changes (enter password)
3. Add your terminal app (Terminal.app, iTerm2, VS Code, etc.)
4. Restart the application

These permissions allow the app to:
- Listen for global hotkey (Control+Space+R)
- Read clipboard content
- Simulate keyboard input for text replacement

## Step 8: Configure OpenAI API Key

1. Get an API key from: https://platform.openai.com/api-keys
2. The app will create a config file at:
   - **macOS**: `~/Library/Application Support/Rephraser/config.json`
   - **Windows**: `%APPDATA%/Rephraser/config.json`

3. You can either:
   - Use the Settings UI in the app (when implemented)
   - Or manually create the config file:

```json
{
  "hotkey": "Control+Space+R",
  "default_style": "professional",
  "api_provider": "openai",
  "api_key": "sk-your-key-here",
  "theme": "system",
  "start_on_login": false,
  "auto_update": true
}
```

## Step 9: Test the Application

Follow the testing guide in `TESTING.md`.

Quick smoke test:
1. With app running (`npm run tauri dev`)
2. **Grant accessibility permissions** (macOS: System Preferences → Security & Privacy → Accessibility → Add Terminal)
3. Open any text editor
4. Type: "hello this is a test"
5. **SELECT the text** (highlight it or press Cmd+A) - do NOT copy it manually
6. Press hotkey: **Control+Space+R**
7. App should automatically capture the selected text and rephrase it

## Development Workflow

### Running the App
```bash
npm run tauri dev
```

### Making Changes

**Frontend Changes (React/TypeScript)**:
- Edit files in `src/`
- Hot reload is enabled (changes appear automatically)
- No restart needed

**Backend Changes (Rust)**:
- Edit files in `src-tauri/src/`
- App automatically recompiles and restarts
- Takes ~10-30 seconds

**Configuration Changes**:
- Edit `src-tauri/tauri.conf.json`
- Requires restart

### Building for Production

```bash
npm run tauri build
```

**Output locations**:
- **macOS**: `src-tauri/target/release/bundle/dmg/Rephraser_0.1.0_*.dmg`
- **Windows**: `src-tauri/target/release/bundle/msi/Rephraser_0.1.0_*.msi`
- **Linux**: `src-tauri/target/release/bundle/appimage/Rephraser_0.1.0_*.AppImage`

## Troubleshooting

### "command not found: rustc"
**Fix**: Rust not installed. Follow Step 1.

### "error: linker `cc` not found"
**Fix (macOS)**: Install Xcode Command Line Tools
```bash
xcode-select --install
```

**Fix (Windows)**: Install Visual Studio C++ Build Tools

**Fix (Linux)**: Install build-essential
```bash
sudo apt install build-essential
```

### "Failed to resolve module" or "Cannot find module"
**Fix**: Reinstall node modules
```bash
rm -rf node_modules package-lock.json
npm install
```

### Tauri compilation errors
**Fix**: Clear Cargo cache and rebuild
```bash
cd src-tauri
cargo clean
cd ..
npm run tauri dev
```

### "error: could not compile `tauri` due to previous error"
**Fix**: Update Rust
```bash
rustup update stable
```

### Application doesn't respond to hotkey
**Fix**: Check accessibility permissions (macOS) or run as administrator (Windows)

### "API key not configured" error
**Fix**: Add API key to config file (see Step 8)

## IDE Setup

### VS Code (Recommended)

Install these extensions:
- **rust-analyzer**: Rust language support
- **Tauri**: Tauri-specific tooling
- **ES7+ React/Redux/React-Native snippets**: React snippets
- **Tailwind CSS IntelliSense**: Tailwind autocomplete
- **Prettier**: Code formatting

### Settings (`.vscode/settings.json`)
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "[typescript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[typescriptreact]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  }
}
```

## Additional Resources

- **Tauri Docs**: https://tauri.app/v1/guides/
- **Rust Book**: https://doc.rust-lang.org/book/
- **React Docs**: https://react.dev/
- **Tailwind Docs**: https://tailwindcss.com/docs

## Project Structure Reference

```
rephraser/
├── src/                        # React frontend
│   ├── components/
│   │   ├── Popup.tsx           # Main popup UI
│   │   └── Settings.tsx        # Settings window
│   ├── store/
│   │   └── appStore.ts         # Zustand state
│   ├── App.tsx                 # Main app component
│   └── main.tsx                # React entry point
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── lib.rs              # Tauri app setup
│   │   ├── ai.rs               # OpenAI integration
│   │   ├── clipboard_ops.rs    # Clipboard handling
│   │   └── config.rs           # Config management
│   ├── Cargo.toml              # Rust dependencies
│   └── tauri.conf.json         # Tauri configuration
├── memory-bank/                # Project documentation
├── package.json                # Node dependencies
├── tailwind.config.js          # Tailwind configuration
├── TESTING.md                  # Testing guide
└── README.md                   # Project readme

```

---

**Setup Complete! 🎉**

You're now ready to develop and test Rephraser.

Next steps:
1. Run `npm run tauri dev`
2. Follow `TESTING.md` to verify functionality
3. Start developing new features!


