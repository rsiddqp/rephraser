# Build and Package Rephraser for Testing

## Option 1: Bundle API Key (For Trusted Testers)

This approach bundles your API key securely in the DMG for easy testing.

### Steps:

1. **Create a default config file with your API key:**

```bash
cd "/Users/rahil/test project/src-tauri"
cat > resources/default-config.json << 'EOF'
{
  "hotkey": "CommandOrControl+Shift+R",
  "default_style": "professional",
  "api_provider": "openai",
  "api_key": "sk-YOUR-ACTUAL-API-KEY-HERE",
  "theme": "system",
  "start_on_login": false,
  "auto_update": true
}
EOF
```

2. **Build the DMG:**

```bash
cd "/Users/rahil/test project"
npm run tauri build
```

3. **DMG will be created at:**
```
src-tauri/target/release/bundle/dmg/Rephraser_0.1.0_aarch64.dmg
```

4. **Share the DMG** with testers

### How It Works:
- On first launch, app detects no config exists
- Automatically copies the bundled `default-config.json` to user's config directory
- Testers can use the app immediately without setup
- Users can still change the API key in Settings if needed

### Security Considerations:
- ⚠️ API key will be in the DMG (readable if someone extracts it)
- ✅ Good for: Trusted team members, internal testing
- ❌ Bad for: Public distribution, untrusted users
- 💡 Set usage limits on this API key in OpenAI dashboard

---

## Option 2: Separate Setup Script (More Secure)

For wider distribution, provide the DMG + a setup script.

### Steps:

1. **Build the DMG (without API key):**

```bash
npm run tauri build
```

2. **Create a setup script for testers:**

```bash
# setup-rephraser.sh
#!/bin/bash

echo "🚀 Setting up Rephraser for testing..."

# Create config directory
CONFIG_DIR="$HOME/Library/Application Support/Rephraser"
mkdir -p "$CONFIG_DIR"

# Write config with API key
cat > "$CONFIG_DIR/config.json" << 'EOF'
{
  "hotkey": "CommandOrControl+Shift+R",
  "default_style": "professional",
  "api_provider": "openai",
  "api_key": "sk-YOUR-ACTUAL-API-KEY-HERE",
  "theme": "system",
  "start_on_login": false,
  "auto_update": true
}
EOF

echo "✅ Configuration created!"
echo "📍 Location: $CONFIG_DIR/config.json"
echo ""
echo "🎯 Next steps:"
echo "1. Open Rephraser.app"
echo "2. Grant accessibility permissions when prompted"
echo "3. Press Cmd+Shift+R to test!"
```

3. **Share with testers:**
- DMG file
- `setup-rephraser.sh` script
- Instructions: Run the script before opening the app

---

## Option 3: Environment Variable (Developer Testing)

For developer testers who are comfortable with terminal.

### Steps:

1. **Modify config.rs to check environment variable:**

Add this to `src-tauri/src/config.rs`:

```rust
impl Default for AppConfig {
    fn default() -> Self {
        let api_key = std::env::var("REPHRASER_API_KEY").ok();
        
        Self {
            hotkey: "CommandOrControl+Shift+R".to_string(),
            default_style: "professional".to_string(),
            api_provider: "openai".to_string(),
            api_key,
            theme: "system".to_string(),
            start_on_login: false,
            auto_update: true,
        }
    }
}
```

2. **Build DMG:**
```bash
npm run tauri build
```

3. **Testers run:**
```bash
export REPHRASER_API_KEY="sk-your-key-here"
open /Applications/Rephraser.app
```

---

## Option 4: OpenAI API Key Proxy (Most Secure)

Create a simple proxy server that uses your API key server-side.

### Backend Proxy (Node.js):

```javascript
// proxy-server.js
const express = require('express');
const axios = require('axios');
const app = express();

app.use(express.json());

app.post('/api/rephrase', async (req, res) => {
  const { text, style } = req.body;
  
  try {
    const response = await axios.post('https://api.openai.com/v1/chat/completions', {
      model: 'gpt-4o-mini',
      messages: [/* your prompts */],
      temperature: 0.7,
    }, {
      headers: {
        'Authorization': `Bearer ${process.env.OPENAI_API_KEY}`
      }
    });
    
    res.json({ rephrased: response.data.choices[0].message.content });
  } catch (error) {
    res.status(500).json({ error: 'Failed to rephrase' });
  }
});

app.listen(3000);
```

### Modify Rephraser to use proxy:
- Change `ai.rs` to call your proxy instead of OpenAI directly
- Testers use your proxy URL (API key never exposed)
- You control usage and costs

---

## Recommended Approach for Your Use Case:

**Option 1** (Bundle API Key) is the easiest for quick testing with trusted people.

### Implementation:

1. **Create the resources directory:**
```bash
mkdir -p "/Users/rahil/test project/src-tauri/resources"
```

2. **Add your API key to default-config.json:**
```bash
cat > "/Users/rahil/test project/src-tauri/resources/default-config.json" << 'EOF'
{
  "hotkey": "CommandOrControl+Shift+R",
  "default_style": "professional",
  "api_provider": "openai",
  "api_key": "YOUR_ACTUAL_OPENAI_API_KEY_HERE",
  "theme": "system",
  "start_on_login": false,
  "auto_update": true
}
EOF
```

3. **Update tauri.conf.json to include resources:**
Add to the bundle section:
```json
"resources": ["resources/*"]
```

4. **Build:**
```bash
npm run tauri build
```

5. **Set API key limits in OpenAI dashboard** to prevent abuse

The app will auto-configure on first launch! Testers can use it immediately.

Which approach would you like to use?

