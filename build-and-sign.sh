#!/bin/bash
set -e

echo "🔨 Building Rephraser app..."
echo ""

# Build the app
npm run tauri build

echo ""
echo "✍️  Ad-hoc signing the app (no Apple Developer account needed)..."
echo ""

# Ad-hoc sign the app bundle
codesign --force --deep --sign - "src-tauri/target/release/bundle/macos/Rephraser.app"

# Verify the signature
echo ""
echo "✅ Verifying signature..."
codesign -dv "src-tauri/target/release/bundle/macos/Rephraser.app" 2>&1 | grep -E "(Signature|Authority|Identifier)"

echo ""
echo "📦 Creating distribution ZIP..."
cd src-tauri/target/release/bundle/macos
rm -f Rephraser.zip
ditto -c -k --keepParent Rephraser.app Rephraser.zip
cd - > /dev/null

echo ""
echo "✅ BUILD COMPLETE!"
echo ""
echo "📍 Signed app location:"
echo "   src-tauri/target/release/bundle/macos/Rephraser.app"
echo ""
echo "📍 Distribution ZIP:"
echo "   src-tauri/target/release/bundle/macos/Rephraser.zip"
echo ""
echo "🎯 What changed:"
echo "   - App is now ad-hoc signed (better than unsigned)"
echo "   - Reduces Gatekeeper friction"
echo "   - Users still need to right-click → Open (first time only)"
echo "   - No Apple Developer account required!"
echo ""
echo "📤 Ready to share with testers!"

