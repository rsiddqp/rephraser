# Technical Context: Rephraser

## Technology Stack

### Desktop Framework: **Tauri** (Recommended)

**Rationale**:
- **Lightweight**: 3-5MB binaries (vs. 50-100MB for Electron)
- **Performance**: Rust backend, native webview (no bundled Chromium)
- **Security**: Built-in security best practices, sandboxed by default
- **Cross-Platform**: Single codebase for macOS/Windows/Linux
- **Modern**: Great TypeScript support, active development

**Alternative**: Electron (if Tauri limitations encountered)

### Frontend Framework: **React 18 + TypeScript**

**UI Libraries**:
- **Styling**: Tailwind CSS 3.x
- **Animations**: Framer Motion
- **Icons**: Lucide React or Heroicons
- **State**: Zustand (lightweight state management)
- **Forms**: React Hook Form (if needed for settings)

### Backend (Rust - Tauri)**:
- **Hotkey Management**: `tauri-plugin-global-shortcut`
- **Clipboard**: `tauri-plugin-clipboard`
- **Window Management**: `tauri::window`
- **System Tray**: `tauri::SystemTray`
- **Auto-Updater**: `tauri-plugin-updater`
- **HTTP Client**: `reqwest` for API calls
- **Async Runtime**: `tokio`

### Platform-Specific APIs

#### macOS
- **Accessibility**: `objc` crate bindings to `NSAccessibility`
- **Hotkeys**: `tauri-plugin-global-shortcut` (wraps `CGEventTap`)
- **Permissions**: Info.plist entries for accessibility

#### Windows
- **UI Automation**: `windows-rs` crate for Win32/WinRT APIs
- **Hotkeys**: `RegisterHotKey` via `windows-rs`
- **Permissions**: Manifest for UI Automation access

### AI/ML Stack

**Universal Multi-Model Support**
- **Architecture**: Provider-agnostic AI module supporting multiple LLM APIs
- **Supported Providers**:
  - **OpenAI**: GPT-4o-mini via `https://api.openai.com/v1/chat/completions`
  - **Anthropic**: Claude 3.5 Sonnet via `https://api.anthropic.com/v1/messages`
  - **Google**: Gemini Pro via `https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent`
  - **Perplexity**: Llama 3.1 Sonar via `https://api.perplexity.ai/chat/completions`
- **Client**: `reqwest` HTTP client (Rust)
- **Authentication**: User-provided API keys (stored in config.json)
- **Timeout**: 30 seconds for all providers
- **Error Handling**: Provider-specific error messages for better UX

**Implementation Details**:
- Single unified `rephrase_text()` function routes to provider-specific handlers
- Style-specific prompts generated consistently across all providers
- Standardized request/response structures for each API
- Graceful error handling with user-friendly messages
- Extensible design allows easy addition of new providers

**Future Considerations**:
- Local model support (deferred to Phase 3+)
- Response caching for performance optimization
- Rate limiting and cost tracking per user

## Development Environment

### Prerequisites
- **Node.js**: 18.x or 20.x LTS
- **Rust**: 1.75+ (install via `rustup`)
- **Tauri CLI**: `npm install -g @tauri-apps/cli`
- **pnpm**: Preferred package manager (faster than npm/yarn)

### Project Structure
```
rephraser/
├── src/                        # React frontend
│   ├── components/
│   │   ├── Popup.tsx           # Main popup UI
│   │   ├── StyleSelector.tsx   # Mode pills
│   │   ├── TextPreview.tsx     # Rephrased text display
│   │   └── Settings.tsx        # Settings window
│   ├── hooks/
│   │   ├── useRephrase.ts      # AI rephrasing logic
│   │   ├── useHotkey.ts        # Hotkey bindings
│   │   └── useSelection.ts     # Text selection state
│   ├── store/
│   │   └── appStore.ts         # Zustand store
│   ├── services/
│   │   ├── aiService.ts        # AI API client
│   │   └── configService.ts    # Settings management
│   ├── App.tsx
│   ├── main.tsx
│   └── styles/
│       └── globals.css
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── main.rs             # App entry point
│   │   ├── hotkey.rs           # Global hotkey listener
│   │   ├── accessibility.rs    # Text selection/replacement
│   │   ├── window.rs           # Popup window management
│   │   ├── ai/
│   │   │   ├── mod.rs
│   │   │   ├── openai.rs       # OpenAI client
│   │   │   ├── anthropic.rs    # Anthropic client
│   │   │   └── local.rs        # Local model interface
│   │   └── utils/
│   │       ├── clipboard.rs    # Clipboard operations
│   │       └── config.rs       # Config file management
│   ├── Cargo.toml
│   ├── tauri.conf.json         # Tauri configuration
│   └── icons/                  # App icons
├── package.json
├── tsconfig.json
├── tailwind.config.js
├── vite.config.ts
└── README.md
```

### Build Commands
```bash
# Development
pnpm install
pnpm tauri dev

# Production build
pnpm tauri build  # Creates installer in src-tauri/target/release/bundle/

# Platform-specific
pnpm tauri build --target aarch64-apple-darwin  # macOS Apple Silicon
pnpm tauri build --target x86_64-apple-darwin   # macOS Intel
pnpm tauri build --target x86_64-pc-windows-msvc # Windows 64-bit
```

## Configuration Files

### `tauri.conf.json`
```json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devPath": "http://localhost:1420",
    "distDir": "../dist"
  },
  "package": {
    "productName": "Rephraser",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "clipboard": {
        "all": true
      },
      "globalShortcut": {
        "all": true
      },
      "window": {
        "all": false,
        "create": true,
        "center": true,
        "close": true,
        "hide": true,
        "show": true,
        "setPosition": true,
        "setSize": true
      },
      "shell": {
        "open": true
      }
    },
    "bundle": {
      "active": true,
      "identifier": "com.rephraser.app",
      "icon": [
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "macOS": {
        "minimumSystemVersion": "10.15",
        "exceptionDomain": ""
      },
      "windows": {
        "wix": {
          "language": "en-US"
        }
      }
    },
    "security": {
      "csp": "default-src 'self'; connect-src https://api.openai.com https://api.anthropic.com"
    },
    "systemTray": {
      "iconPath": "icons/tray-icon.png",
      "iconAsTemplate": true
    },
    "updater": {
      "active": true,
      "endpoints": ["https://releases.rephraser.app/{{target}}/{{current_version}}"],
      "pubkey": "YOUR_PUBLIC_KEY"
    },
    "windows": [
      {
        "title": "Rephraser",
        "width": 400,
        "height": 300,
        "resizable": false,
        "fullscreen": false,
        "decorations": false,
        "transparent": true,
        "alwaysOnTop": true,
        "visible": false,
        "skipTaskbar": true
      }
    ]
  }
}
```

### `package.json` (key dependencies)
```json
{
  "name": "rephraser",
  "version": "0.1.0",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "tauri": "tauri"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "zustand": "^4.4.0",
    "framer-motion": "^10.16.0",
    "lucide-react": "^0.294.0",
    "@tauri-apps/api": "^1.5.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.2.0",
    "autoprefixer": "^10.4.16",
    "postcss": "^8.4.32",
    "tailwindcss": "^3.3.6",
    "typescript": "^5.3.0",
    "vite": "^5.0.0"
  }
}
```

### `Cargo.toml` (key dependencies)
```toml
[package]
name = "rephraser"
version = "0.1.0"
edition = "2021"

[dependencies]
tauri = { version = "1.5", features = ["api-all", "system-tray", "updater"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
anyhow = "1.0"
thiserror = "1.0"
enigo = "0.1"  # For keyboard simulation

[target.'cfg(target_os = "macos")'.dependencies]
objc = "0.2"
cocoa = "0.25"
core-graphics = "0.23"

[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.52", features = ["Win32_UI_Accessibility", "Win32_UI_Input_KeyboardAndMouse"] }
```

## API Integration

### Universal AI Integration
```typescript
// Frontend invocation - works with any provider
import { invoke } from '@tauri-apps/api';

export async function rephraseText(
  text: string,
  style: 'professional' | 'casual' | 'sarcasm',
  provider: string,
  apiKey: string
): Promise<string> {
  const result = await invoke<string>('rephrase_text', {
    text,
    style,
    provider, // 'openai', 'claude', 'gemini', 'perplexity'
    apiKey
  });
  return result;
}
```

```rust
// src-tauri/src/ai.rs - Universal AI module
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::Style;

// Main entry point - routes to appropriate provider
pub async fn rephrase_text(
    text: &str,
    style: &Style,
    provider: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    match provider.to_lowercase().as_str() {
        "openai" => rephrase_with_openai(text, style, api_key).await,
        "claude" | "anthropic" => rephrase_with_claude(text, style, api_key).await,
        "gemini" | "google" => rephrase_with_gemini(text, style, api_key).await,
        "perplexity" => rephrase_with_perplexity(text, style, api_key).await,
        _ => Err(format!("Unsupported provider: {}", provider).into()),
    }
}

// Provider-specific implementations handle API differences
// Each follows the same pattern:
// 1. Build provider-specific request structure
// 2. Make HTTP request with appropriate headers/auth
// 3. Parse provider-specific response format
// 4. Return standardized string result
```

### Key Design Principles:
- **Separation of Concerns**: Each provider has its own implementation
- **Consistent Interface**: All providers use the same function signature
- **Error Handling**: Provider-specific error messages guide users
- **Extensibility**: Adding new providers requires minimal code changes
- **Security**: API keys never leave the Rust backend

## Coding Standards

### TypeScript
- Strict mode enabled
- No `any` types (use `unknown` if needed)
- Functional components with hooks (no class components)
- Named exports preferred over default exports
- 2-space indentation

### Rust
- Clippy lints enabled (`#![warn(clippy::all)]`)
- Error handling with `Result<T, E>` (no panics in production)
- Async/await for I/O operations
- 4-space indentation
- Comprehensive error types

### Testing
- **Frontend**: Vitest + React Testing Library
- **Backend**: `cargo test` with unit and integration tests
- **E2E**: Playwright for cross-platform UI testing
- Target: >80% code coverage

### Git Workflow
- Branch naming: `feature/`, `bugfix/`, `release/`
- Conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`
- PR required for main branch
- Automated CI/CD with GitHub Actions

## Deployment & Distribution

### macOS
- **Format**: `.dmg` installer
- **Signing**: Apple Developer certificate required
- **Notarization**: Required for Gatekeeper bypass
- **Distribution**: Direct download + Mac App Store (optional)

### Windows
- **Format**: `.msi` installer (WiX Toolset)
- **Signing**: Code signing certificate (DigiCert, etc.)
- **Distribution**: Direct download + Microsoft Store (optional)

### Auto-Updates
- **Backend**: GitHub Releases or custom server
- **Tauri Updater**: Built-in update checker
- **Strategy**: Check on launch + every 24 hours
- **User Control**: Optional auto-install or manual approval

## Privacy & Compliance

### Data Handling
- **Text Processing**: Ephemeral, never logged
- **Analytics**: Optional, opt-in only
- **Telemetry**: Crash reports (via Sentry), opt-out available
- **API Keys**: Stored in OS keychain (macOS Keychain, Windows Credential Manager)

### Licenses
- **App**: Proprietary (closed source)
- **Dependencies**: MIT/Apache 2.0 compatible only
- **Attribution**: Third-party licenses displayed in About section

## Performance Optimization

### Startup Time
- Lazy load heavy dependencies
- Defer local model loading until first offline use
- Minimal main thread work

### Memory Management
- Release popup window when hidden (not just invisible)
- Clear AI response cache after 5 minutes
- Limit clipboard history to last 10 items

### Bundle Size
- Tree-shake unused dependencies
- Compress assets with WebP/AVIF
- Dynamic imports for settings UI

## Known Limitations

1. **Accessibility Permissions**: Required on macOS/Windows (user must grant)
2. **App Compatibility**: Some apps (e.g., Terminal, password managers) may block text injection
3. **Rich Text**: Formatting may be lost during replacement (plaintext only for MVP)
4. **Rate Limits**: API providers impose rate limits (handled gracefully)
5. **Offline Mode**: Local model quality lower than API models
6. **Languages**: English only for MVP (multi-language in future)


