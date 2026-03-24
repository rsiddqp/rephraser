#!/bin/bash
set -euo pipefail

# ---------------------------------------------------------------------------
# Rephraser — Production Build for Mac App Store
#
# Prerequisites:
#   1. Apple Developer account + certificates installed in Keychain
#   2. Rust targets:  rustup target add aarch64-apple-darwin x86_64-apple-darwin
#   3. Set environment variables (or this script will prompt):
#        APPLE_SIGNING_IDENTITY   e.g. "3rd Party Mac Developer Application: Your Name (TEAMID)"
#        APPLE_TEAM_ID            e.g. "ABCDE12345"
#        APPLE_ID                 e.g. "you@example.com"
#        APPLE_APP_PASSWORD       app-specific password for notarytool
# ---------------------------------------------------------------------------

echo "=========================================="
echo " Rephraser — App Store Production Build"
echo "=========================================="
echo ""

# --- Signing identity ---
if [ -z "${APPLE_SIGNING_IDENTITY:-}" ]; then
  echo "Available signing identities:"
  security find-identity -v -p codesigning | grep "Developer\|3rd Party\|Apple Distribution" || true
  echo ""
  read -rp "Enter signing identity (name or hash): " APPLE_SIGNING_IDENTITY
fi
echo "Signing identity: $APPLE_SIGNING_IDENTITY"

# --- Team ID ---
if [ -z "${APPLE_TEAM_ID:-}" ]; then
  read -rp "Enter Apple Team ID: " APPLE_TEAM_ID
fi

# --- Ensure Rust targets ---
echo ""
echo "Checking Rust targets..."
rustup target add aarch64-apple-darwin 2>/dev/null || true
rustup target add x86_64-apple-darwin  2>/dev/null || true
echo "Rust targets ready."

# --- Build ---
echo ""
echo "Building universal binary (arm64 + x86_64)..."
echo "This may take several minutes on first build."
echo ""

npm run tauri build -- --target universal-apple-darwin

APP_PATH="src-tauri/target/universal-apple-darwin/release/bundle/macos/Rephraser.app"

if [ ! -d "$APP_PATH" ]; then
  echo "ERROR: Build output not found at $APP_PATH"
  echo "Trying single-arch fallback..."
  npm run tauri build
  APP_PATH="src-tauri/target/release/bundle/macos/Rephraser.app"
fi

if [ ! -d "$APP_PATH" ]; then
  echo "ERROR: Build failed — no .app bundle found."
  exit 1
fi

echo ""
echo "Build complete: $APP_PATH"

# --- Sign ---
echo ""
echo "Signing with: $APPLE_SIGNING_IDENTITY"

codesign --force --deep --options runtime \
  --entitlements src-tauri/entitlements.plist \
  --sign "$APPLE_SIGNING_IDENTITY" \
  "$APP_PATH"

echo "Verifying signature..."
codesign --verify --deep --strict "$APP_PATH"
echo "Signature valid."

# --- Create ZIP for notarization ---
ZIP_PATH="${APP_PATH%.app}.zip"
rm -f "$ZIP_PATH"
ditto -c -k --keepParent "$APP_PATH" "$ZIP_PATH"
echo "Created: $ZIP_PATH"

# --- Notarize ---
echo ""
echo "Submitting for notarization..."

if [ -z "${APPLE_ID:-}" ]; then
  read -rp "Enter Apple ID email: " APPLE_ID
fi
if [ -z "${APPLE_APP_PASSWORD:-}" ]; then
  read -rsp "Enter app-specific password: " APPLE_APP_PASSWORD
  echo ""
fi

xcrun notarytool submit "$ZIP_PATH" \
  --apple-id "$APPLE_ID" \
  --team-id "$APPLE_TEAM_ID" \
  --password "$APPLE_APP_PASSWORD" \
  --wait

# --- Staple ---
echo ""
echo "Stapling notarization ticket..."
xcrun stapler staple "$APP_PATH"

# --- Re-zip with stapled ticket ---
rm -f "$ZIP_PATH"
ditto -c -k --keepParent "$APP_PATH" "$ZIP_PATH"

echo ""
echo "=========================================="
echo " BUILD COMPLETE"
echo "=========================================="
echo ""
echo "Signed + notarized app:"
echo "  $APP_PATH"
echo ""
echo "Distribution ZIP:"
echo "  $ZIP_PATH"
echo ""
echo "Next steps:"
echo "  1. Test the app locally (double-click to open)"
echo "  2. Upload to App Store Connect via Transporter or 'xcrun altool'"
echo "  3. Submit for App Review"
echo ""
