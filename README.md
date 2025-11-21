# Rephraser 📝

A lightweight, high-performance desktop application for macOS and Windows that lets you instantly rephrase, format, and enhance any selected text system-wide with three stylistic modes: **Professional**, **Casual**, and **Sarcasm**.

## Features ✨

- **Ready Out of the Box**: Works immediately after installation - no API key setup required
- **System-Wide**: Works in any application where text can be selected
- **Instant Access**: Global hotkey (`Cmd+Shift+R` on Mac, `Ctrl+Shift+R` on Windows)
- **Multiple Styles**: Three modes for different contexts
  - **Professional**: Business-appropriate, formal tone
  - **Casual**: Friendly, conversational tone
  - **Sarcasm**: Witty, subtly sarcastic tone
- **Optional Advanced Features**: Power users can configure their own OpenAI, Claude, Gemini, or Perplexity API keys
- **Seamless Integration**: Non-intrusive, appears only when needed
- **Privacy-First**: No data storage, text never logged
- **Cross-Platform**: Native support for macOS and Windows

## Installation

### Prerequisites
- **macOS**: 10.15 (Catalina) or later
- **Windows**: Windows 10 or later
- **No API Key Required**: Works out of the box with default proxy server
- **Optional**: Advanced users can use their own API keys:
  - [OpenAI](https://platform.openai.com/api-keys) - GPT-4o-mini
  - [Anthropic](https://console.anthropic.com/account/keys) - Claude 3.5 Sonnet
  - [Google](https://makersuite.google.com/app/apikey) - Gemini Pro
  - [Perplexity](https://www.perplexity.ai/settings/api) - Sonar

### Download
Download the latest release for your platform:
- **macOS**: `Rephraser.dmg`
- **Windows**: `Rephraser.msi`

### Setup
1. Install the application
2. Grant accessibility permissions when prompted (required for system-wide text access)
3. **Done!** The app works immediately - select text and press `Cmd+Shift+R`

**Optional - Advanced Users Only**:
4. Open Settings (⚙️ icon) to configure your own AI model
5. Select a custom provider (OpenAI, Claude, Gemini, or Perplexity)
6. Enter your API key

## Usage 🚀

1. **SELECT** any text in any application (highlight it or press Cmd+A)
2. Press the keyboard shortcut: `Cmd+Shift+R` (Mac) or `Ctrl+Shift+R` (Windows)
3. The app will **automatically capture and rephrase** your selected text
4. Review the rephrased text in the app
5. **Copy** the result to use it, or click **Rephrase** to get a different version

**No manual copying required!** Just select and press the hotkey.

### Example

**Original:**
> hey can we push this to next week im swamped

**Professional:**
> Would it be possible to reschedule this for next week? I'm currently managing several urgent priorities.

**Casual:**
> Hey! Any chance we could move this to next week? I'm pretty swamped right now!

**Sarcasm:**
> Oh sure, let's push it to next week. It's not like I have anything better to do than drown in work.

## Development

### Tech Stack
- **Frontend**: React 18 + TypeScript + Tailwind CSS
- **Backend**: Tauri (Rust)
- **AI Models**: OpenAI GPT-4o-mini, Claude 3.5 Sonnet, Gemini Pro, Perplexity Sonar
- **State**: Zustand

### Prerequisites
- Node.js 18+ or 20+ LTS
- Rust 1.75+ (`rustup`)
- npm or pnpm

### Setup Development Environment

```bash
# Clone repository
git clone https://github.com/rsiddqp/rephraser.git
cd rephraser

# Install dependencies
npm install

# Run development server
npm run tauri dev

# Build for production
npm run tauri build
```

### Project Structure

```
rephraser/
├── src/                    # React frontend
│   ├── components/         # UI components (Settings)
│   ├── store/             # State management (config only)
│   └── App.tsx            # Main app component
├── src-tauri/             # Rust backend
│   └── src/
│       ├── ai.rs          # Multi-model AI integration
│       ├── accessibility.rs # Text capture
│       ├── config.rs       # Settings management
│       └── lib.rs         # Main Tauri app
├── backend-proxy/         # Default proxy server
└── memory-bank/           # Project documentation
```

## Configuration

Settings are stored in:
- **macOS**: `~/Library/Application Support/Rephraser/config.json`
- **Windows**: `%APPDATA%/Rephraser/config.json`

### Available Settings
- `hotkey`: Global shortcut (`Cmd+Shift+R` on Mac, `Ctrl+Shift+R` on Windows)
- `model_provider`: AI model provider
  - `proxy` (default) - Works immediately, no API key required
  - `openai`, `claude`, `gemini`, `perplexity` - Use your own API key
- `default_style`: Starting style mode (professional, casual, sarcasm)
- `api_key`: Your API key (optional, only needed for custom providers)
- `theme`: UI theme (light/dark/system)

## AI Provider Details

### Default (Proxy Server)
- **Model**: OpenAI GPT-4o-mini (via proxy)
- **Setup**: None required
- **Cost**: Free for users
- **Status**: ✅ Verified working

### Advanced Options
| Provider | Model | API Endpoint | Tested |
|----------|-------|--------------|--------|
| OpenAI | gpt-4o-mini | api.openai.com | ✅ |
| Anthropic | claude-3-5-sonnet-20241022 | api.anthropic.com | ✅ |
| Google | gemini-pro | generativelanguage.googleapis.com | ✅ |
| Perplexity | sonar | api.perplexity.ai | ✅ |

## Privacy & Security 🔒

- **No Data Storage**: Text is never logged or stored
- **Local Processing**: Only sends to chosen AI provider when rephrasing
- **Secure Storage**: API keys stored in config.json on your device
- **No Tracking**: No analytics or telemetry
- **Your Choice**: Use default proxy or your own API key

## Known Limitations

- Requires accessibility permissions on macOS/Windows
- Some applications may block text injection (e.g., password managers)
- Rich text formatting may be lost (plaintext only in MVP)
- English language only (multi-language support planned)

## Troubleshooting

### App doesn't capture text
- Ensure accessibility permissions are granted in System Preferences
- Try selecting text in a different application
- Restart the app

### API errors
- Check your API key is correct in Settings
- Verify internet connection
- Switch to default Proxy Server if issues persist

### Keyboard shortcut not working
- Restart the app to re-register the shortcut
- Check no other app is using Cmd+Shift+R
- Try using the app window directly (click Rephrase button)

## Roadmap 🗺️

- [ ] Local AI model for offline mode
- [ ] Additional styles (Witty, Empathetic, Technical)
- [ ] Multi-language support
- [ ] Browser extension
- [ ] Custom style preferences
- [ ] Team/enterprise features

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) first.

## License

Proprietary - All rights reserved

## Support

- **Issues**: [GitHub Issues](https://github.com/rsiddqp/rephraser/issues)
- **Email**: support@rephraser.app
- **Documentation**: [docs.rephraser.app](https://docs.rephraser.app)

## Acknowledgments

Built with:
- [Tauri](https://tauri.app) - Desktop framework
- [React](https://react.dev) - UI framework
- [OpenAI](https://openai.com) - AI rephrasing
- [Anthropic](https://anthropic.com) - Claude AI
- [Google](https://ai.google.dev) - Gemini AI
- [Perplexity](https://perplexity.ai) - Perplexity AI
- [Tailwind CSS](https://tailwindcss.com) - Styling

---

Made with ❤️ by the Rephraser Team
