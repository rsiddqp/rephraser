#!/bin/bash
set -euo pipefail

# ---------------------------------------------------------------------------
# Rephraser — Build for Direct Distribution (Website / Landing Page)
#
# Signs with Developer ID + notarizes so users can download and run
# without Gatekeeper blocking. This is for distribution OUTSIDE the
# App Store (your landing page, GitHub releases, etc).
#
# Prerequisites:
#   1. "Developer ID Application" certificate in Keychain
#   2. Rust targets installed (script will add them)
#   3. App-specific password from https://appleid.apple.com
# ---------------------------------------------------------------------------

SIGNING_IDENTITY="Developer ID Application: Zuzis LLC (44RU9345Y9)"
TEAM_ID="44RU9345Y9"

echo "=========================================="
echo " Rephraser — Direct Distribution Build"
echo "=========================================="
echo ""
echo "Identity: $SIGNING_IDENTITY"
echo "Team:     $TEAM_ID"
echo ""

# --- Ensure Rust targets ---
echo "Checking Rust targets..."
rustup target add aarch64-apple-darwin 2>/dev/null || true
rustup target add x86_64-apple-darwin  2>/dev/null || true

# --- Build universal binary ---
echo ""
echo "Building universal binary (arm64 + x86_64)..."
echo "This may take several minutes on first build."
echo ""

npm run tauri build -- --target universal-apple-darwin

APP_PATH="src-tauri/target/universal-apple-darwin/release/bundle/macos/Rephraser.app"

if [ ! -d "$APP_PATH" ]; then
  echo "Universal build not found, trying single-arch..."
  npm run tauri build
  APP_PATH="src-tauri/target/release/bundle/macos/Rephraser.app"
fi

if [ ! -d "$APP_PATH" ]; then
  echo "ERROR: Build failed — no .app bundle found."
  exit 1
fi

echo "Build complete: $APP_PATH"

# --- Sign with Developer ID ---
echo ""
echo "Signing for direct distribution..."

codesign --force --deep --options runtime \
  --entitlements src-tauri/entitlements.plist \
  --sign "$SIGNING_IDENTITY" \
  "$APP_PATH"

codesign --verify --deep --strict "$APP_PATH"
echo "Signature valid."

# --- Create ZIP ---
ZIP_PATH="${APP_PATH%.app}.zip"
rm -f "$ZIP_PATH"
ditto -c -k --keepParent "$APP_PATH" "$ZIP_PATH"
echo "Created: $ZIP_PATH"

# --- Notarize ---
echo ""
echo "Submitting for notarization..."
echo "(This sends the app to Apple for malware scanning — takes 2-10 minutes)"
echo ""

if [ -z "${APPLE_ID:-}" ]; then
  read -rp "Apple ID email: " APPLE_ID
fi
if [ -z "${APPLE_APP_PASSWORD:-}" ]; then
  read -rsp "App-specific password (from appleid.apple.com): " APPLE_APP_PASSWORD
  echo ""
fi

xcrun notarytool submit "$ZIP_PATH" \
  --apple-id "$APPLE_ID" \
  --team-id "$TEAM_ID" \
  --password "$APPLE_APP_PASSWORD" \
  --wait

# --- Staple ---
echo ""
echo "Stapling notarization ticket to app..."
xcrun stapler staple "$APP_PATH"

# --- Final ZIP with stapled ticket ---
rm -f "$ZIP_PATH"
ditto -c -k --keepParent "$APP_PATH" "$ZIP_PATH"

# --- Also create DMG ---
DMG_DIR=$(dirname "$APP_PATH")
DMG_PATH="${DMG_DIR}/Rephraser.dmg"
rm -f "$DMG_PATH"
hdiutil create -volname "Rephraser" -srcfolder "$APP_PATH" -ov -format UDZO "$DMG_PATH" 2>/dev/null || true

echo ""
echo "=========================================="
echo " DIRECT DISTRIBUTION BUILD COMPLETE"
echo "=========================================="
echo ""
echo "Signed + notarized app:"
echo "  $APP_PATH"
echo ""
echo "Distribution ZIP (upload to landing page):"
echo "  $ZIP_PATH"
if [ -f "$DMG_PATH" ]; then
echo ""
echo "Distribution DMG:"
echo "  $DMG_PATH"
fi
echo ""
echo "Users can now download and open without Gatekeeper warnings."
echo ""
