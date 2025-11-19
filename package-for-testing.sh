#!/bin/bash

# Rephraser - Package for Testing Script
# This script builds a DMG with your API key bundled for easy testing

set -e  # Exit on error

echo "📦 Rephraser - Build for Testing"
echo "================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get API key from user
echo "🔑 Enter your OpenAI API key (it will be bundled in the DMG):"
read -s API_KEY

if [ -z "$API_KEY" ]; then
    echo -e "${RED}❌ API key cannot be empty${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}✅ API key received${NC}"
echo ""

# Create resources directory
echo "📁 Creating resources directory..."
mkdir -p src-tauri/resources

# Create default config with API key
echo "⚙️  Creating bundled configuration..."
cat > src-tauri/resources/default-config.json << EOF
{
  "hotkey": "CommandOrControl+Shift+R",
  "default_style": "professional",
  "api_provider": "openai",
  "api_key": "$API_KEY",
  "theme": "system",
  "start_on_login": false,
  "auto_update": true
}
EOF

echo -e "${GREEN}✅ Configuration created${NC}"
echo ""

# Set usage limits reminder
echo -e "${YELLOW}⚠️  IMPORTANT: Set usage limits on this API key!${NC}"
echo "   Go to: https://platform.openai.com/account/limits"
echo "   Set monthly limit to prevent unexpected costs"
echo ""
read -p "Press Enter when you've set limits (or to continue)..."

# Build the DMG
echo ""
echo "🏗️  Building DMG (this may take 2-5 minutes)..."
npm run tauri build

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🎉 Build successful!${NC}"
    echo ""
    echo "📍 DMG Location:"
    echo "   src-tauri/target/release/bundle/dmg/Rephraser_0.1.0_aarch64.dmg"
    echo ""
    echo "📤 Share this DMG with your testers!"
    echo ""
    echo "📝 Instructions for testers:"
    echo "   1. Install Rephraser.app"
    echo "   2. Grant accessibility permissions (System Preferences → Security & Privacy → Accessibility)"
    echo "   3. Select text anywhere and press Cmd+Shift+R"
    echo "   4. Review rephrased text and press Cmd+C to copy and dismiss"
    echo ""
    echo -e "${YELLOW}⚠️  Security Note:${NC}"
    echo "   The API key is bundled in the app. Only share with trusted testers!"
    echo ""
    
    # Open finder to the DMG location
    open src-tauri/target/release/bundle/dmg/
else
    echo -e "${RED}❌ Build failed. Check errors above.${NC}"
    exit 1
fi

