#!/bin/bash
set -euo pipefail

# ---------------------------------------------------------------------------
# Rephraser — Build for Mac App Store
#
# Signs with "Apple Distribution" and creates a .pkg for App Store upload.
#
# Prerequisites:
#   1. "Apple Distribution" certificate in Keychain (you have this)
#   2. "3rd Party Mac Developer Installer" certificate in Keychain
#      If missing: Apple Developer portal > Certificates > + > Mac Installer Distribution
#   3. Provisioning profile for com.rephraser.desktop
#      Apple Developer portal > Profiles > + > Mac App Store
#   4. App record created in App Store Connect
#   5. Rust targets installed (script will add them)
# ---------------------------------------------------------------------------

SIGNING_IDENTITY="Apple Distribution: Zuzis LLC (44RU9345Y9)"
INSTALLER_IDENTITY="3rd Party Mac Developer Installer: Zuzis LLC (44RU9345Y9)"
TEAM_ID="44RU9345Y9"
BUNDLE_ID="com.rephraser.desktop"

echo "=========================================="
echo " Rephraser — App Store Build"
echo "=========================================="
echo ""
echo "App Identity:       $SIGNING_IDENTITY"
echo "Installer Identity: $INSTALLER_IDENTITY"
echo "Bundle ID:          $BUNDLE_ID"
echo "Team:               $TEAM_ID"
echo ""

# --- Check for installer certificate ---
if ! security find-identity -v | grep -q "3rd Party Mac Developer Installer"; then
  echo "WARNING: '3rd Party Mac Developer Installer' certificate not found!"
  echo ""
  echo "You need this to create the .pkg for App Store upload."
  echo "To create it:"
  echo "  1. Go to https://developer.apple.com/account/resources/certificates/add"
  echo "  2. Select 'Mac Installer Distribution'"
  echo "  3. Follow the CSR steps and download/install the certificate"
  echo ""
  read -rp "Press Enter to continue anyway (will skip .pkg creation), or Ctrl+C to abort..."
  SKIP_PKG=true
else
  SKIP_PKG=false
fi

# --- Check for provisioning profile ---
PROFILE_PATH=""
for f in ~/Library/MobileDevice/Provisioning\ Profiles/*.provisionprofile; do
  if [ -f "$f" ] && security cms -D -i "$f" 2>/dev/null | grep -q "$BUNDLE_ID"; then
    PROFILE_PATH="$f"
    break
  fi
done

if [ -z "$PROFILE_PATH" ]; then
  echo "WARNING: No provisioning profile found for $BUNDLE_ID"
  echo ""
  echo "To create one:"
  echo "  1. Register App ID: https://developer.apple.com/account/resources/identifiers/add/bundleId"
  echo "     - Platform: macOS"
  echo "     - Bundle ID (explicit): $BUNDLE_ID"
  echo "  2. Create Profile: https://developer.apple.com/account/resources/profiles/add"
  echo "     - Type: Mac App Store"
  echo "     - App ID: $BUNDLE_ID"
  echo "     - Certificate: Apple Distribution"
  echo "  3. Download and double-click to install"
  echo ""
  read -rp "Press Enter to continue without profile, or Ctrl+C to set it up first..."
else
  echo "Provisioning profile found: $PROFILE_PATH"
fi

# --- Ensure Rust targets ---
echo ""
echo "Checking Rust targets..."
rustup target add aarch64-apple-darwin 2>/dev/null || true
rustup target add x86_64-apple-darwin  2>/dev/null || true

# --- Build universal binary ---
echo ""
echo "Building universal binary (arm64 + x86_64)..."
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

# --- Embed provisioning profile ---
if [ -n "$PROFILE_PATH" ]; then
  echo "Embedding provisioning profile..."
  cp "$PROFILE_PATH" "$APP_PATH/Contents/embedded.provisionprofile"
fi

# --- Sign with Apple Distribution ---
echo ""
echo "Signing for App Store..."

codesign --force --deep --options runtime \
  --entitlements src-tauri/entitlements.plist \
  --sign "$SIGNING_IDENTITY" \
  "$APP_PATH"

codesign --verify --deep --strict "$APP_PATH"
echo "Signature valid."

# --- Create .pkg for App Store upload ---
if [ "$SKIP_PKG" = false ]; then
  echo ""
  echo "Creating installer .pkg..."
  PKG_PATH="${APP_PATH%.app}.pkg"
  rm -f "$PKG_PATH"

  productbuild --component "$APP_PATH" /Applications \
    --sign "$INSTALLER_IDENTITY" \
    "$PKG_PATH"

  echo "Created: $PKG_PATH"

  echo ""
  echo "=========================================="
  echo " APP STORE BUILD COMPLETE"
  echo "=========================================="
  echo ""
  echo "Installer package (upload to App Store Connect):"
  echo "  $PKG_PATH"
  echo ""
  echo "Next steps:"
  echo "  1. Open Transporter.app (download from Mac App Store)"
  echo "  2. Drag the .pkg into Transporter"
  echo "  3. Click Deliver"
  echo "  4. Go to App Store Connect and submit for review"
  echo ""
else
  echo ""
  echo "=========================================="
  echo " APP STORE BUILD (PARTIAL)"
  echo "=========================================="
  echo ""
  echo "Signed app (cannot upload without installer certificate):"
  echo "  $APP_PATH"
  echo ""
  echo "To complete App Store submission, install the"
  echo "'Mac Installer Distribution' certificate and re-run."
  echo ""
fi
