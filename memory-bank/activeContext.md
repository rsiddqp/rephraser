# Active Context: Rephraser

## Current Status
**Phase**: ✅ FULLY DEPLOYED - MULTI-MODEL SUPPORT ADDED
**Last Updated**: 2025-11-21
**Quality**: Production-ready with universal LLM support (OpenAI, Claude, Gemini, Perplexity)

**🌐 Public URLs**:
  - Landing Page: https://rsiddqp.github.io/rephraser-landing (✅ LIVE)
  - Download: https://rsiddqp.github.io/rephraser-landing/Rephraser.zip (✅ WORKING)
  - Backend API: https://rephraser-9ur5.onrender.com (✅ tested, working)
  
**📦 Download**: 
  - ✅ Working download via GitHub Pages
  - File: Rephraser.zip (4.4 MB)
  - Direct link: https://rsiddqp.github.io/rephraser-landing/Rephraser.zip
  - **FIX APPLIED**: Changed from GitHub Releases to local GitHub Pages hosting

**GitHub Repos**: 
  - Main code: https://github.com/rsiddqp/rephraser
  - Landing page: https://github.com/rsiddqp/rephraser-landing
  
**Status**: ✅ Ready for public use - share the landing page URL!

## Current Task
The application has been enhanced with **universal multi-model LLM support**. Users can now choose from multiple AI providers and use their own API keys. The workflow remains seamless: SELECT text → hit Cmd+Shift+R (Mac) or Ctrl+Shift+R (Windows) → get revised text. **No manual copying required!**

### Latest Enhancements (2025-11-21)
- ✅ Universal multi-model support (OpenAI, Claude, Gemini, Perplexity)
- ✅ Users provide their own API keys (no proxy server needed)
- ✅ Settings UI updated with model provider selection
- ✅ Corrected hotkey display (Cmd+Shift+R on Mac, Ctrl+Shift+R on Windows)
- ✅ AI module refactored to support all major LLM providers
- ✅ Configuration updated with model_provider field
- ✅ All documentation cleaned up (removed 18 unnecessary markdown files)
- ✅ README updated with multi-model information

## Recent Developments
1. **Memory Bank Created**: Complete documentation structure established
   - `projectbrief.md`: Project requirements and scope
   - `productContext.md`: User experience and product vision
   - `systemPatterns.md`: Architecture and design patterns
   - `techContext.md`: Technology stack and implementation

2. **Tauri Project Initialized**: Full project structure with React + TypeScript
   - Configured Tauri v2 with frameless, transparent popup window
   - Set up Tailwind CSS v3 for styling
   - Added Zustand for state management
   - Integrated Framer Motion for animations

3. **Backend Implementation Complete**:
   - `lib.rs`: Main Tauri app with command handlers and global hotkey registration
   - `ai.rs`: OpenAI GPT-4 integration with style-specific prompts
   - `clipboard_ops.rs`: Text capture and replacement via clipboard manipulation
   - `config.rs`: Configuration file management with Control+Space+R as default hotkey

4. **Frontend Implementation Complete**:
   - `App.tsx`: Main app with automatic hotkey registration and clipboard-based workflow
   - `Popup.tsx`: Adaptive UI with style selector and text preview
   - `Settings.tsx`: Configuration interface for API key and preferences
   - `appStore.ts`: Zustand store for state management

5. **CRITICAL KEYBOARD SHORTCUT IMPLEMENTED** (2025-11-19):
   - Global hotkey registration: Control+Space+R
   - Automatic text capture from clipboard when hotkey pressed
   - Automatic rephrasing triggered on hotkey
   - Complete workflow: copy → shortcut → revised text
   - Added @tauri-apps/plugin-global-shortcut and plugin-clipboard-manager
   - Permissions configured in capabilities/default.json

6. **AUTOMATIC TEXT SELECTION CAPTURE IMPLEMENTED** (2025-11-19 - Enhanced):
   - NEW: No manual copying required! Just select text
   - Created src-tauri/src/accessibility.rs with platform-specific implementations
   - macOS: Uses Core Graphics to simulate Cmd+C and captures selected text
   - Windows: Uses SendInput to simulate Ctrl+C
   - Smart clipboard management: preserves user's original clipboard
   - Seamless workflow: SELECT text → Cmd+Shift+R → get revised text
   - Added core-graphics and windows dependencies
   - Created Info.plist for macOS accessibility permissions
   - Updated frontend to use get_selected_text instead of get_clipboard_text

7. **SMART ADAPTIVE POSITIONING IMPLEMENTED** (2025-11-19):
   - Window automatically appears ABOVE cursor/selected text when hotkey is pressed
   - Multi-monitor support: Detects which screen cursor is on and uses that screen's bounds
   - macOS: Iterates through all active displays to find the one containing cursor
   - Windows: Uses Win32 APIs (GetCursorPos, GetSystemMetrics)
   - Smart positioning logic:
     * Primary: ABOVE cursor (centered horizontally)
     * Falls back to BELOW if top is clipped
     * Adjusts left/right to stay within screen bounds
     * Handles multiple monitors with different sizes/arrangements
   - Window dimensions: 500x600px with 20px padding from edges
   - Window set to always-on-top for visibility
   - Auto-scrolls to rephrased text section when ready
   - Detailed logging for position debugging

8. **PRODUCTION HARDENING COMPLETED** (2025-11-19):
   - Removed all unused code (clipboard_ops.rs deleted, unused functions removed)
   - Added comprehensive input validation (text length limits, empty text checks)
   - Improved error handling with user-friendly messages
   - Enhanced API error messages (401, 429, 500+ errors with context)
   - Added request debouncing to prevent concurrent executions
   - Dynamic token calculation for large text support (up to 10,000 chars)
   - Changed to gpt-4o-mini for better cost/performance
   - Extended timeout to 30s for large text
   - Optimized clipboard delay for faster response
   - Comprehensive diagnostic logging for debugging
   - Production-ready error boundaries
   - Fixed UTF-8 string slicing panic with multi-byte characters
   - Fixed timing: Captures text BEFORE showing window (prevents focus stealing)
   - Fixed window positioning: Always shows ABOVE text with screen-bounds checking
   - Multi-monitor support: Detects active display containing cursor
   - Auto-scroll to rephrased text when ready

9. **SEAMLESS COPY & DISMISS WORKFLOW** (2025-11-19):
   - Click "Copy" button → copies rephrased text AND minimizes window immediately
   - Press Cmd+C when UI is focused → copies rephrased text AND minimizes window
   - Smart detection: Only intercepts Cmd+C when rephrased text exists
   - Preserves normal copy behavior in input fields
   - Resets UI state for next use
   - Ultra-fast workflow: select → hotkey → review → copy → continue working

10. **PUBLIC DEPLOYMENT COMPLETED** (2025-11-19):
   - ✅ Backend proxy DEPLOYED and LIVE at https://rephraser-9ur5.onrender.com
   - ✅ GitHub repository created: https://github.com/rsiddqp/rephraser
   - ✅ Modified app to use proxy server (API key never exposed to users)
   - ✅ Removed API key requirement from app (handled server-side)
   - ✅ Production app rebuilt with proxy integration
   - ✅ Rate limiting: 20 requests/min per IP (server-side)
   - ✅ Free tier deployment on Render.com
   - ✅ Tested and verified: Proxy working correctly
   - Landing page ready at landing-page/index.html
   - Production ZIP ready for download
   
11. **AD-HOC CODE SIGNING IMPLEMENTED** (2025-11-19):
   - ✅ Created build-and-sign.sh script for automated signing
   - ✅ App now ad-hoc signed (no Apple Developer account needed)
   - ✅ Reduces macOS Gatekeeper friction for users
   - ✅ Signed app deployed to landing page
   - ✅ Updated installation instructions on landing page
   - Users can now right-click → Open (no terminal commands for most users)
   - Better UX than unsigned version
   
11. **Build Verification**: 
   - Frontend: 209KB gzipped ✅
   - Backend proxy: LIVE at Render ✅
   - Production app: Built and ready ✅

## Next Steps

### For Friends/Beta Testing (Current)
1. **Share the ZIP file**: `src-tauri/target/release/bundle/macos/Rephraser.zip` (4.4MB)
2. **Include instructions**: `TESTER_INSTRUCTIONS.md`
3. **Current setup**: API key bundled in app (for trusted testers only)
4. **Collect feedback** and iterate

### For Public Deployment (Next Phase)
1. **Deploy backend proxy server**:
   - Created: `backend-proxy/server.js` with secure API key handling
   - Options: Railway (free tier), Heroku ($7/mo), or Vercel (serverless)
   - Rate limiting: 20 requests/minute per IP
   - Cost tracking: ~$0.00009 per rephrase with gpt-4o-mini
   
2. **Modify app to use proxy**:
   - Update `ai.rs` to call proxy instead of OpenAI directly
   - Remove API key from Settings (handled server-side)
   - API key never exposed to users
   
3. **Code signing** (optional for wider distribution):
   - Apple Developer account: $99/year
   - Sign and notarize the app
   - Removes Gatekeeper warnings
   
4. **Landing page**:
   - Simple download page with features
   - Host on GitHub Pages, Vercel, or Netlify (free)
   - Include installation instructions

### Estimated Costs for Public Launch
- Backend hosting: $5-7/month (Railway/Heroku)
- OpenAI API: ~$30/month for 1,000 active users
- Apple Developer: $99/year (optional)
- Total: ~$40-45/month + $99/year one-time

## Active Decisions

### Framework Choice: Tauri (Final)
- **Why**: 3-5MB binaries vs. 50-100MB for Electron
- **Trade-off**: Less mature ecosystem, but acceptable for our use case
- **Alternative**: Electron if Tauri accessibility APIs prove insufficient

### AI Provider: Universal Multi-Model Support
- **Supported Models**:
  - OpenAI GPT-4o-mini (fast, cost-effective)
  - Anthropic Claude 3.5 Sonnet (high quality)
  - Google Gemini Pro (balanced performance)
  - Perplexity Llama 3.1 (alternative option)
- **User Control**: Users choose their preferred provider and use their own API keys
- **Extensible**: Easy to add new providers in the future

### Text Replacement Strategy: Accessibility API First
- **Primary**: Direct text injection via accessibility APIs (fastest)
- **Fallback**: Clipboard + paste simulation (most compatible)
- **Last Resort**: Keystroke simulation (slowest but universal)

### Hotkey Default: Cmd+Shift+R (Mac) / Ctrl+Shift+R (Windows)
- **Display**: Now correctly shows "Cmd + Shift + R" in Settings UI
- **Rationale**: Cross-platform compatible format, unlikely to conflict with existing shortcuts
- **Implementation**: Uses CommandOrControl+Shift+R format for registration
- **Critical**: This hotkey is essential for the entire workflow - select text, hit Cmd+Shift+R, get rephrased result

## Critical Considerations

### 1. Permission Handling
- macOS requires explicit accessibility permission
- Must guide users through permission granting flow
- Graceful degradation if permission denied (clipboard fallback)

### 2. Performance Targets
- **Hotkey → UI**: <150ms (perceived as instant)
- **AI Response**: <500ms (GPT-4 Turbo average)
- **Text Replace**: <100ms (accessibility API)
- **Total Latency**: <750ms (feels seamless)

### 3. Privacy & Security
- No text logging or storage
- API keys encrypted in OS keychain
- Optional offline mode for privacy-conscious users
- Clear data handling disclosure

### 4. Cross-Platform Parity
- macOS and Windows must have feature parity
- Platform-specific code isolated in modules
- Consistent UI/UX across platforms
- Test on both systems throughout development

### 5. Error Handling
- API failures → retry with exponential backoff
- Accessibility denied → fallback to clipboard
- No selection → show helpful error toast
- Offline → use local model or cached results

## Relevant Learnings

### Tauri Best Practices
- Use `invoke` for Rust ↔ TypeScript communication
- Keep frontend logic in TypeScript, OS integration in Rust
- Leverage built-in plugins (`global-shortcut`, `clipboard`, etc.)
- Use `tauri-plugin-store` for persistent settings

### Accessibility API Gotchas
- Not all apps expose accessibility info (Terminal, password fields)
- Rich text formatting often lost during capture
- Coordinate systems differ between monitors/OS versions
- Permission prompts may interrupt flow (handle gracefully)

### AI Prompt Engineering
- Be explicit about tone/style in system prompt
- Provide examples for better consistency
- Set temperature to 0.7 for creative rephrasing
- Limit max tokens to control cost and latency

### Desktop App UX
- Frameless, transparent windows for modern look
- Always-on-top for popup, but auto-hide on focus loss
- Smooth animations (150ms fade) feel polished
- System tray icon for persistent access

## Open Questions
1. **Local Model**: Should we bundle a local model in the installer or make it optional download?
   - **Decision**: Optional download (deferred to Phase 3+)
2. **Subscription Model**: Free tier limits? Pay-per-use vs. subscription?
   - **Defer**: Focus on MVP functionality first
3. **Multi-Language Support**: Should MVP include non-English languages?
   - **Decision**: English only for MVP, add later based on demand
4. **Clipboard vs Accessibility API**: Which should be primary strategy?
   - **Decision**: Clipboard for MVP (simpler), accessibility API in Phase 2
5. **Text Replacement Method**: Which is most reliable?
   - **Testing Needed**: Will determine during end-to-end testing

## Blockers
- **Rust Installation Required**: Cannot compile or test backend without Rust toolchain
  - **Severity**: Critical - blocks all testing
  - **Resolution**: 5-minute install via rustup.rs
  - **Next Step**: Run `npm run tauri dev` after installing Rust

## Resources
- Tauri Docs: https://tauri.app/v1/guides/
- OpenAI API: https://platform.openai.com/docs
- macOS Accessibility: https://developer.apple.com/documentation/accessibility
- Windows UI Automation: https://learn.microsoft.com/en-us/windows/win32/winauto/entry-uiauto-win32

