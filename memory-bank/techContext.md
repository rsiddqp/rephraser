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

**Primary: API-Based**
- **Provider**: OpenAI GPT-4 Turbo / Anthropic Claude Sonnet
- **Client**: `reqwest` HTTP client (Rust) or `axios` (TypeScript)
- **Endpoints**:
  - OpenAI: `https://api.openai.com/v1/chat/completions`
  - Anthropic: `https://api.anthropic.com/v1/messages`
- **Rate Limiting**: Token bucket algorithm, user-tier based
- **Caching**: In-memory LRU cache (5 min TTL)

**Fallback: Local Model**
- **Model**: Gemma 2 2B (quantized to 4-bit)
- **Runtime**: `llama.cpp` via Rust FFI or Node native addon
- **Size**: ~1.5GB download (optional)
- **Performance**: 10-30 tokens/sec on modern CPU
- **Use Case**: Offline mode, privacy-focused users

**Alternative APIs** (for cost optimization):
- QuillBot API (if available)
- Cohere API
- Together.ai (cheaper inference)

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

### OpenAI Integration
```typescript
// src/services/aiService.ts
import { invoke } from '@tauri-apps/api';

export async function rephraseWithOpenAI(
  text: string, 
  style: 'professional' | 'casual' | 'sarcasm'
): Promise<string> {
  // Call Rust backend to make API request (keeps API key secure)
  const result = await invoke<string>('rephrase_text', {
    text,
    style,
    provider: 'openai'
  });
  return result;
}
```

```rust
// src-tauri/src/ai/openai.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

pub async fn rephrase(
    api_key: &str,
    text: &str,
    style: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let prompt = match style {
        "professional" => format!(
            "Rephrase this professionally: {}", text
        ),
        "casual" => format!(
            "Rephrase this casually: {}", text
        ),
        "sarcasm" => format!(
            "Rephrase this with subtle sarcasm: {}", text
        ),
        _ => text.to_string(),
    };
    
    let request = OpenAIRequest {
        model: "gpt-4-turbo-preview".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a writing assistant.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        temperature: 0.7,
        max_tokens: 500,
    };
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?;
    
    let data: serde_json::Value = response.json().await?;
    let rephrased = data["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();
    
    Ok(rephrased)
}
```

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


