# Rephraser Installation Guide

## Quick Start (Most Users)

1. Download `Rephraser.zip` from https://rsiddqp.github.io/rephraser-landing/
2. Double-click to extract → You'll see `Rephraser.app`
3. Right-click `Rephraser.app` → Select "Open"
4. Click "Open" in the confirmation dialog
5. Grant accessibility permissions when prompted
6. Done! Press `Cmd+Shift+R` on any selected text

## If You See "Rephraser is damaged" Error

This is macOS Gatekeeper blocking unsigned apps. The app is completely safe!

### Fix: Remove Quarantine Flag

1. Open **Terminal** (search "Terminal" in Spotlight)
2. Copy and paste this command:

```bash
xattr -d com.apple.quarantine ~/Downloads/Rephraser.app
```

3. Press **Enter**
4. Now double-click `Rephraser.app` - it will open!

### Alternative: Move to Applications First

If the app is in a different location:

```bash
# If in Downloads
xattr -d com.apple.quarantine ~/Downloads/Rephraser.app

# If in Applications
xattr -d com.apple.quarantine /Applications/Rephraser.app

# If on Desktop
xattr -d com.apple.quarantine ~/Desktop/Rephraser.app
```

## Why Does This Happen?

macOS has a security feature called "Gatekeeper" that blocks apps that are:
- Not from the App Store
- Not signed with an Apple Developer certificate ($99/year)

Your Rephraser app is **completely safe** - it just needs to be code-signed to avoid this warning.

## Using Rephraser

Once installed:

1. **Select any text** in any app (email, browser, documents, etc.)
2. **Press `Cmd+Shift+R`**
3. **Popup appears** with rephrased text
4. **Choose a style**: Professional, Casual, or Sarcasm
5. **Press `Cmd+C`** to copy the result (window closes automatically)
6. **Paste anywhere** with `Cmd+V`

## Accessibility Permissions

Rephraser needs accessibility permissions to:
- Capture selected text automatically
- Position the popup window near your cursor

When you first open Rephraser, macOS will prompt you to grant these permissions.

### If Permissions Dialog Doesn't Appear:

1. Open **System Preferences** → **Security & Privacy**
2. Click **Privacy** tab
3. Select **Accessibility** from the left sidebar
4. Click the lock icon (bottom left) and enter your password
5. Find **Rephraser** in the list and check the box
6. If Rephraser isn't listed, click **+** and add it from Applications

## Troubleshooting

### App won't open at all
Run this command:
```bash
xattr -cr ~/Downloads/Rephraser.app
```

### App opens but hotkey doesn't work
- Check accessibility permissions (see above)
- Make sure `Cmd+Shift+R` isn't used by another app

### Popup doesn't appear when pressing hotkey
- Ensure you've selected text first
- Check that Rephraser.app is running (you'll see it in the menu bar)

### No internet connection error
- Backend API might be waking up (first request takes ~30 seconds)
- Try again in 30 seconds

## For Developers: Preventing This Issue

To eliminate the "damaged" warning for all users:

1. **Get Apple Developer Account**: $99/year
2. **Sign the app**:
   ```bash
   codesign --force --deep --sign "Developer ID Application: Your Name" Rephraser.app
   ```
3. **Notarize the app**:
   ```bash
   xcrun notarytool submit Rephraser.zip --apple-id your@email.com --password app-specific-password --team-id TEAMID
   ```
4. **Staple the notarization**:
   ```bash
   xcrun stapler staple Rephraser.app
   ```

Once signed and notarized, users can simply double-click to open - no warnings!

## Getting Help

- Landing Page: https://rsiddqp.github.io/rephraser-landing/
- Issues: Contact support via GitHub

---

**Note**: Rephraser is currently in beta (v0.1.0). The code signing process will be implemented in the next release to eliminate installation friction.

