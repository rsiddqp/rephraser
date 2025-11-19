# System Patterns: Rephraser Architecture

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Operating System                          │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              Any Application (Email, Browser, etc.)     │ │
│  │  ┌──────────────────────────────────────────────────┐  │ │
│  │  │         User selects text + Ctrl+R+T             │  │ │
│  │  └────────────────────┬─────────────────────────────┘  │ │
│  └───────────────────────┼────────────────────────────────┘ │
└────────────────────────┬─┼─────────────────────────────────┘
                         │ │
                         ▼ ▼
        ┌────────────────────────────────────┐
        │   Rephraser Desktop Application    │
        ├────────────────────────────────────┤
        │  ┌──────────────────────────────┐  │
        │  │   Global Hotkey Listener     │  │
        │  └──────────┬───────────────────┘  │
        │             ▼                       │
        │  ┌──────────────────────────────┐  │
        │  │  Accessibility API Manager   │  │
        │  │  - macOS: AXUIElement        │  │
        │  │  - Windows: UI Automation    │  │
        │  └──────────┬───────────────────┘  │
        │             ▼                       │
        │  ┌──────────────────────────────┐  │
        │  │   Selection Capture Module   │  │
        │  └──────────┬───────────────────┘  │
        │             ▼                       │
        │  ┌──────────────────────────────┐  │
        │  │    Adaptive Popup UI         │  │
        │  │  - Position Calculator       │  │
        │  │  - Style Mode Selector       │  │
        │  │  - Preview Renderer          │  │
        │  └──────────┬───────────────────┘  │
        │             ▼                       │
        │  ┌──────────────────────────────┐  │
        │  │   AI Rephrasing Engine       │  │
        │  │  ┌────────────────────────┐  │  │
        │  │  │ Local Model (Offline)  │  │  │
        │  │  └────────────────────────┘  │  │
        │  │  ┌────────────────────────┐  │  │
        │  │  │ API Client (Online)    │  │  │
        │  │  │ - OpenAI / Anthropic   │  │  │
        │  │  │ - Custom fine-tuned    │  │  │
        │  │  └────────────────────────┘  │  │
        │  └──────────┬───────────────────┘  │
        │             ▼                       │
        │  ┌──────────────────────────────┐  │
        │  │  Text Replacement Module     │  │
        │  │  - Clipboard manipulation    │  │
        │  │  - Direct input injection    │  │
        │  └──────────┬───────────────────┘  │
        │             ▼                       │
        │  ┌──────────────────────────────┐  │
        │  │   Settings & Configuration   │  │
        │  │  - Hotkey manager            │  │
        │  │  - API keys                  │  │
        │  │  - Preferences               │  │
        │  └──────────────────────────────┘  │
        └────────────────────────────────────┘
```

## Core Components

### 1. Global Hotkey Listener
**Purpose**: Capture system-wide keyboard shortcut (Control+Space+R)

**Technology**:
- **macOS**: `CGEventTap` or `NSEvent` addGlobalMonitorForEvents
- **Windows**: `RegisterHotKey` Win32 API or `GlobalHotKey` library
- **Electron**: `electron.globalShortcut.register()`
- **Tauri**: Custom native plugin with Rust

**Key Considerations**:
- Must not conflict with system/app shortcuts
- Configurable key combinations
- Support for "Hyper" key (Cmd+Shift+Ctrl+Opt on macOS)
- Graceful handling when hotkey unavailable

**Implementation Pattern**:
```typescript
// Pseudo-code
class HotkeyManager {
  register(combination: string, callback: Function)
  unregister(combination: string)
  isConflict(combination: string): boolean
  suggestAlternative(combination: string): string
}
```

### 2. Accessibility API Manager
**Purpose**: Detect selected text and current cursor position across all applications

**Technology**:
- **macOS**: Accessibility API (AXUIElement)
  - Requires user permission in System Preferences
  - Can read selected text from any app
  - Get cursor/selection coordinates
- **Windows**: UI Automation API
  - Similar permission requirements
  - Access to active controls and selections

**Key Considerations**:
- Graceful permission request flow
- Fallback to clipboard if accessibility denied
- Handle apps that don't expose accessibility info
- Cache application metadata for performance

**Implementation Pattern**:
```typescript
interface SelectionInfo {
  text: string;
  position: { x: number; y: number };
  appName: string;
  fieldType: string;
}

class AccessibilityManager {
  getSelectedText(): Promise<SelectionInfo>
  replaceSelection(newText: string): Promise<void>
  requestPermissions(): Promise<boolean>
}
```

### 3. Selection Capture Module
**Purpose**: Extract and validate selected text

**Responsibilities**:
- Capture text from accessibility APIs
- Fallback to clipboard monitoring
- Validate text length and format
- Detect single word vs. phrase vs. paragraph
- Track cursor/selection coordinates

**Edge Cases**:
- No text selected → show error toast
- Non-editable text (PDFs) → show copy option
- Multi-selection → handle first selection
- Rich text with formatting → preserve where possible

### 4. Adaptive Popup UI
**Purpose**: Display rephrasing interface near selected text

**Design Requirements**:
- **Positioning**: 
  - Appear near selection (prefer below, right side)
  - Avoid screen edges (flip position if needed)
  - Multi-monitor aware (show on active monitor)
- **Sizing**:
  - Dynamic based on text length
  - Min: 300px wide, Max: 600px wide
  - Auto-height with scroll for long text
- **Animation**:
  - 150ms fade-in with slight scale (0.95 → 1.0)
  - 100ms fade-out on close
- **Components**:
  - Style mode pills (Professional, Casual, Sarcasm)
  - Text preview area
  - Rephrase button (cycle icon)
  - Replace button (checkmark/replace icon)
  - Close button (X or ESC key)

**Technology**:
- Frameless window with transparency
- CSS/Tailwind for styling
- React for component state management
- Framer Motion for animations

### 5. AI Rephrasing Engine
**Purpose**: Generate alternative text in different styles

**Architecture**:
```
Input Text → Style Selection → Prompt Engineering → Model → Output
```

**Model Strategy**:
- **Primary**: OpenAI GPT-4 Turbo or Anthropic Claude Sonnet (API)
  - Highest quality
  - Sub-second latency
  - Cost: ~$0.001-0.01 per rephrase
- **Fallback**: Local quantized model (Gemma 2 2B, LLaMA 3 8B)
  - Works offline
  - ~1-3 second latency on modern hardware
  - Free after download

**Prompt Templates**:
```typescript
const PROMPTS = {
  professional: `Rephrase the following text in a professional, formal tone suitable for business communication. Maintain the core message but improve clarity and professionalism:\n\n{text}`,
  
  casual: `Rephrase the following text in a casual, friendly tone suitable for informal communication. Make it conversational and approachable:\n\n{text}`,
  
  sarcasm: `Rephrase the following text with subtle sarcasm while maintaining the surface-level message. Keep it witty but not offensive:\n\n{text}`,
  
  synonym: `Provide 5 alternative words or short phrases that could replace "{text}" in context. Return as comma-separated list.`
};
```

**Caching Strategy**:
- Cache rephrases for 5 minutes (same text + style)
- LRU cache with 100 entry limit
- Clear on app close

**Error Handling**:
- API timeout (5s) → fallback to local model
- Local model failure → show generic error
- Rate limiting → queue requests
- Invalid input → sanitize and retry

### 6. Text Replacement Module
**Purpose**: Replace selected text in the original application

**Implementation Strategies**:

**Strategy A: Clipboard + Paste Simulation (Most Compatible)**
```typescript
async function replaceText(newText: string) {
  const originalClipboard = await clipboard.read();
  await clipboard.write(newText);
  await simulateKeyPress('Cmd+V'); // or Ctrl+V on Windows
  await delay(100);
  await clipboard.write(originalClipboard); // restore
}
```

**Strategy B: Accessibility API Direct Injection (Preferred)**
```typescript
async function replaceTextDirect(newText: string) {
  const element = await accessibility.getFocusedElement();
  await element.setSelectedTextValue(newText);
}
```

**Strategy C: Keystroke Simulation (Fallback)**
```typescript
async function replaceTextKeystrokes(newText: string) {
  await simulateKeyPress('Backspace'); // delete selection
  await simulateTyping(newText); // type new text
}
```

**Selection**:
- Try Strategy B (fastest, most reliable)
- Fallback to Strategy A if accessibility fails
- Strategy C as last resort (slow but works everywhere)

### 7. Settings & Configuration
**Purpose**: Manage user preferences and app configuration

**Settings Storage**:
- **macOS**: `~/Library/Application Support/Rephraser/config.json`
- **Windows**: `%APPDATA%/Rephraser/config.json`
- Encrypted API keys
- Plain text preferences

**Configuration Options**:
```typescript
interface AppConfig {
  hotkey: string; // default: "Control+Space+R"
  defaultStyle: 'professional' | 'casual' | 'sarcasm';
  apiProvider: 'openai' | 'anthropic' | 'local';
  apiKey?: string;
  theme: 'light' | 'dark' | 'system';
  startOnLogin: boolean;
  autoUpdate: boolean;
  offlineMode: boolean;
  telemetry: boolean;
}
```

## Data Flow

### Happy Path: Rephrase & Replace
1. User copies text in any app (Ctrl+C / Cmd+C) → presses Control+Space+R
2. Hotkey listener triggers → reads clipboard content
3. App window appears with original text from clipboard
4. AI engine receives text + current style → generates rephrase automatically
5. Preview updates with rephrased text
6. User can change style to regenerate or click "Copy"
7. User clicks "Copy" → rephrased text copied to clipboard
8. User pastes (Ctrl+V / Cmd+V) in their original app
9. Workflow complete → user continues working

### Alternative Path: Regenerate
- Steps 1-5 same as above
- User clicks "Rephrase" button (🔄)
- AI engine generates alternative with same style
- Preview updates with new suggestion
- User can cycle through multiple alternatives
- Selects preferred version → Copy to clipboard

### Edge Case: Single Word
1. User copies single word → presses Control+Space+R
2. System detects single word (no spaces/punctuation)
3. AI engine uses synonym prompt instead of rephrase
4. App shows 3-5 synonym options
5. User clicks through alternatives
6. Selects preferred synonym → Copy

## Design Patterns

### Singleton Pattern (App Instance)
- Ensures only one app instance runs
- Lock file prevents duplicate launches
- Second launch focuses existing instance

### Observer Pattern (Hotkey → UI)
- Hotkey listener emits events
- Multiple subscribers react (logging, analytics, UI)

### Strategy Pattern (AI Model Selection)
- Abstracted AI interface
- Runtime selection: OpenAI, Anthropic, Local
- Easy to add new providers

### Factory Pattern (Platform-Specific Code)
- Accessibility factory creates platform-specific implementations
- Hotkey factory creates platform-specific listeners
- Single codebase, platform-specific execution

### Command Pattern (Text Operations)
- Each text operation (capture, replace) is a command
- Enables undo/redo functionality
- Logging and analytics

## Performance Targets

- **Hotkey Response**: <50ms (hotkey press → UI starts rendering)
- **UI Render**: <100ms (UI visible on screen)
- **AI Response**: <500ms (API call for GPT-4 Turbo)
- **Text Replacement**: <100ms (new text injected)
- **Total Latency**: <750ms (hotkey → replacement complete)

- **Memory**: <100MB RAM when idle, <200MB when active
- **CPU**: <5% when idle, <30% during AI processing
- **Disk**: <50MB installation size (excluding local models)

## Security Considerations

1. **API Keys**: Encrypted at rest using OS keychain
2. **Text Privacy**: Never logged or transmitted except to chosen AI provider
3. **Permissions**: Request minimal required permissions
4. **Updates**: Signed binaries, verified downloads
5. **Network**: Optional offline mode, clear data flows


