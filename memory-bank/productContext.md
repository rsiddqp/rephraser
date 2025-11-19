# Product Context: Rephraser

## Why This Product Exists

### Problem Statement
1. **Tone Mismatches**: Users struggle to match their writing tone to context (professional emails, casual messages, etc.)
2. **Time Consumption**: Manual rewriting and editing takes significant time
3. **Context Switching**: Existing tools require copying text to external applications or websites
4. **Limited Accessibility**: Most writing assistants are browser-based or application-specific
5. **Grammar & Style Issues**: Non-native speakers and busy professionals need instant assistance

### Market Gap
- Grammarly: Browser/app specific, not truly system-wide, lacks instant tone switching
- QuillBot: Requires browser/app switching, not inline
- ChatGPT: Requires context switching, not integrated into workflow
- Word/Google Docs: Only works within those applications

## What Rephraser Does

### Core Functionality
Rephraser eliminates context switching by working **everywhere** text exists:
- Email clients (Outlook, Gmail, Apple Mail)
- Messaging apps (Slack, Teams, Discord)
- Document editors (Word, Google Docs, Notion)
- Code editors (for documentation/comments)
- Social media platforms
- Any text input field

### User Experience Flow

#### Installation & Setup
1. User downloads installer for macOS/Windows
2. Quick signup/authentication
3. Grant accessibility permissions (required for system-wide access)
4. Configure global hotkey (default: Control+Space+R)
5. App runs in system tray/menu bar

#### Daily Usage
1. User types or **selects text anywhere** (highlight or Cmd+A) - **no manual copying!**
2. Presses global hotkey (Control+Space+R)
3. App **automatically captures the selected text** and shows:
   - Three style buttons (Professional, Casual, Sarcasm)
   - Rephrased text preview (happens automatically)
   - Rephrase icon (regenerate alternatives)
   - Copy button (to use the rephrased text)
4. User selects desired style, optionally regenerates
5. Clicks Copy → rephrased text copied to clipboard
6. User can paste the revised text wherever needed

**Key Innovation**: No manual copying step! Just select → shortcut → done.

#### Example Scenarios

**Scenario 1: Email to Boss**
- User types casual draft: "hey can we push this to next week im swamped"
- **Selects the text** (highlight or Cmd+A) - no copying!
- Presses Control+Space+R
- App automatically captures and rephrases to "Professional"
- Result: "Would it be possible to reschedule this for next week? I'm currently managing several urgent priorities."
- Clicks Copy → pastes into email

**Scenario 2: Slack Message**
- User has formal text: "I appreciate your assistance in resolving this matter expeditiously."
- **Selects the text** - no copying!
- Presses Control+Space+R → changes style to "Casual"
- Result: "Thanks for helping sort this out quickly! 🙌"
- Clicks Copy → pastes into Slack

**Scenario 3: Select All and Rephrase**
- User has a full paragraph in document
- Presses **Cmd+A** (select all)
- Presses Control+Space+R
- App captures entire paragraph automatically
- Reviews rephrased version, clicks Rephrase for alternatives
- Clicks Copy → pastes back

### Design Philosophy

#### Invisible Until Needed
- No persistent UI cluttering the screen
- Runs silently in background
- Zero impact on system performance
- Only appears when invoked

#### Contextually Adaptive
- Popup appears near selected text (not fixed position)
- Dynamically sizes based on text length
- Multi-monitor aware
- Smooth fade/slide animations
- Auto-positions to avoid screen edges

#### Speed & Reliability
- Sub-200ms response time
- Works offline (local models for basic rephrasing)
- Graceful degradation if API unavailable
- Never loses original text
- Undo capability (Ctrl+Z still works)

#### Privacy & Security
- No text logging or storage
- Optional local-only processing
- Encrypted API communications
- No user tracking
- Compliant with GDPR/CCPA

## Target Outcomes

### User Benefits
- **10x faster** writing iteration
- **Zero context switching** friction
- **Consistent tone** across communications
- **Improved clarity** and professionalism
- **Confidence** in written communication

### Business Metrics
- Daily active usage rate: >60%
- Time saved per user: 30+ minutes/day
- User retention: >80% after 30 days
- Net Promoter Score: >50
- Subscription conversion: >25% (freemium model)

## Future Vision
- Browser extension for web-first workflows
- Mobile keyboard integration (iOS/Android)
- Team/enterprise features (shared style guides)
- API for third-party integrations
- Voice-to-text with instant rephrasing
- Multi-language support


