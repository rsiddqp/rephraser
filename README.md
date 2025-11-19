# Rephraser 📝

A lightweight, high-performance desktop application for macOS and Windows that lets you instantly rephrase, format, and enhance any selected text system-wide with three stylistic modes: **Professional**, **Casual**, and **Sarcasm**.

## Features ✨

- **System-Wide**: Works in any application where text can be selected
- **Instant Access**: Global hotkey (`Control+Space+R`) triggers adaptive UI
- **Multiple Styles**: Three modes for different contexts
  - **Professional**: Business-appropriate, formal tone
  - **Casual**: Friendly, conversational tone
  - **Sarcasm**: Witty, subtly sarcastic tone
- **Seamless Integration**: Non-intrusive, appears only when needed
- **Privacy-First**: No data storage, text never logged
- **Cross-Platform**: Native support for macOS and Windows

## Installation

### Prerequisites
- **macOS**: 10.15 (Catalina) or later
- **Windows**: Windows 10 or later
- **OpenAI API Key**: Get one from [OpenAI Platform](https://platform.openai.com/api-keys)

### Download
Download the latest release for your platform:
- **macOS**: `Rephraser.dmg`
- **Windows**: `Rephraser.msi`

### Setup
1. Install the application
2. Grant accessibility permissions when prompted (required for system-wide text access)
3. Open Settings and enter your OpenAI API key
4. You're ready to go!

## Usage 🚀

1. **SELECT** any text in any application (highlight it or press Cmd+A)
2. Press the keyboard shortcut: `Control+Space+R`
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
- **AI**: OpenAI GPT-4 Turbo
- **State**: Zustand

### Prerequisites
- Node.js 18+ or 20+ LTS
- Rust 1.75+ (`rustup`)
- npm or pnpm

### Setup Development Environment

```bash
# Clone repository
git clone https://github.com/yourusername/rephraser.git
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
│   ├── components/         # UI components
│   ├── store/             # State management
│   └── App.tsx            # Main app component
├── src-tauri/             # Rust backend
│   └── src/
│       ├── ai.rs          # OpenAI integration
│       ├── clipboard_ops.rs # Text capture/replacement
│       └── config.rs       # Settings management
└── memory-bank/           # Project documentation
```

## Configuration

Settings are stored in:
- **macOS**: `~/Library/Application Support/Rephraser/config.json`
- **Windows**: `%APPDATA%/Rephraser/config.json`

### Available Settings
- `hotkey`: Global shortcut (default: `Control+Space+R`)
- `default_style`: Starting style mode
- `api_key`: OpenAI API key (encrypted)
- `theme`: UI theme (light/dark/system)

## Privacy & Security 🔒

- **No Data Storage**: Text is never logged or stored
- **Local Processing**: Only sends to OpenAI API when rephrasing
- **Encrypted Keys**: API keys stored securely in OS keychain
- **No Tracking**: No analytics or telemetry
- **Open Source**: Code is auditable (planned)

## Known Limitations

- Requires accessibility permissions on macOS/Windows
- Some applications may block text injection (e.g., password managers)
- Rich text formatting may be lost (plaintext only in MVP)
- English language only (multi-language support planned)

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

- **Issues**: [GitHub Issues](https://github.com/yourusername/rephraser/issues)
- **Email**: support@rephraser.app
- **Documentation**: [docs.rephraser.app](https://docs.rephraser.app)

## Acknowledgments

Built with:
- [Tauri](https://tauri.app) - Desktop framework
- [React](https://react.dev) - UI framework
- [OpenAI](https://openai.com) - AI rephrasing
- [Tailwind CSS](https://tailwindcss.com) - Styling

---

Made with ❤️ by the Rephraser Team
