# Rephraser - Public Deployment Guide

This guide covers deploying Rephraser for public use while keeping your OpenAI API key secure.

## Architecture Overview

```
Desktop App (Users) → Proxy Server (Your Backend) → OpenAI API
                      [API Key Hidden Here]
```

**Benefits:**
- ✅ API key never exposed to users
- ✅ You control rate limiting and costs
- ✅ Can track usage and analytics
- ✅ Can implement user authentication later
- ✅ Can switch AI providers without updating the app

---

## Phase 1: Friends & Family Testing

### Step 1: Deploy Backend Proxy

#### Option A: Railway (Recommended - Free tier available)

1. **Install Railway CLI:**
```bash
npm install -g @railway/cli
railway login
```

2. **Deploy:**
```bash
cd backend-proxy
npm install
railway init
railway up
```

3. **Set environment variable:**
```bash
railway variables set OPENAI_API_KEY=sk-your-key-here
```

4. **Get your URL:**
```bash
railway domain
# Example: https://rephraser-proxy-production.up.railway.app
```

#### Option B: Heroku

1. **Create Heroku app:**
```bash
cd backend-proxy
heroku create rephraser-proxy
```

2. **Set API key:**
```bash
heroku config:set OPENAI_API_KEY=sk-your-key-here
```

3. **Deploy:**
```bash
git init
git add .
git commit -m "Initial proxy server"
heroku git:remote -a rephraser-proxy
git push heroku main
```

4. **Your URL**: `https://rephraser-proxy.herokuapp.com`

#### Option C: Vercel (Serverless)

1. **Install Vercel CLI:**
```bash
npm install -g vercel
```

2. **Deploy:**
```bash
cd backend-proxy
vercel
```

3. **Set environment variable** in Vercel dashboard

### Step 2: Update Desktop App to Use Proxy

Modify `src-tauri/src/ai.rs`:

```rust
// Change this line:
// .post("https://api.openai.com/v1/chat/completions")

// To your proxy URL:
.post("https://YOUR-PROXY-URL.railway.app/api/rephrase")

// Also modify the request format to match proxy expectations:
let request_body = serde_json::json!({
    "text": text,
    "style": match style {
        Style::Professional => "professional",
        Style::Casual => "casual",
        Style::Sarcasm => "sarcasm",
    }
});

// Remove Authorization header (not needed with proxy)
let response = client
    .post("https://YOUR-PROXY-URL.railway.app/api/rephrase")
    .json(&request_body)
    .timeout(std::time::Duration::from_secs(30))
    .send()
    .await?;

let data: serde_json::Value = response.json().await?;
let rephrased = data["rephrased"]
    .as_str()
    .ok_or("No response from server")?;
```

### Step 3: Remove API Key from App

Since the proxy handles authentication, remove API key requirement:

1. Remove Settings UI for API key
2. Simplify config (no api_key field needed)
3. App just calls your proxy

### Step 4: Build Signed DMG (Optional for friends)

For a small group, the unsigned ZIP works fine. For wider distribution:

1. **Get Apple Developer account** ($99/year)
2. **Create signing certificate**
3. **Sign and notarize:**
```bash
npm run tauri build
# Then use Apple's notarization tools
xcrun notarytool submit path/to/app.dmg --apple-id you@email.com --wait
```

---

## Phase 2: Public Launch with Landing Page

### Step 1: Scale Your Backend

**Add to your proxy:**
- User authentication (optional)
- Usage analytics
- Better rate limiting (Redis-based)
- Cost monitoring
- Error tracking (Sentry)

### Step 2: Create Landing Page

Basic structure:

```html
<!DOCTYPE html>
<html>
<head>
  <title>Rephraser - AI Text Rephrasing for macOS</title>
</head>
<body>
  <h1>Rephraser</h1>
  <p>Instantly rephrase text in any app with AI</p>
  
  <a href="Rephraser_0.1.0_aarch64.dmg" class="download-btn">
    Download for macOS (Apple Silicon)
  </a>
  
  <h2>Features</h2>
  <ul>
    <li>System-wide text rephrasing</li>
    <li>3 styles: Professional, Casual, Sarcasm</li>
    <li>Keyboard shortcut: Cmd+Shift+R</li>
    <li>Works in any app</li>
  </ul>
</body>
</html>
```

Host on:
- GitHub Pages (free)
- Vercel (free)
- Netlify (free)

### Step 3: Distribution Options

#### Option A: Direct Download (Simple)
- Host DMG on your landing page
- Users download and install manually
- No code signing = users see warning (acceptable for beta)

#### Option B: Code Signed (Professional)
- Requires Apple Developer account ($99/year)
- No Gatekeeper warnings
- Better user trust
- Required for Mac App Store

#### Option C: Mac App Store
- Full Apple review process
- Built-in distribution and updates
- 30% revenue share if you charge
- Most trustworthy for users

---

## Cost Considerations

### Backend Hosting
- **Railway Free**: 500 hours/month (enough for testing)
- **Railway Pro**: $5/month for 24/7 uptime
- **Heroku**: $7/month for basic dyno
- **Vercel**: Free for serverless (pay per request)

### API Costs (gpt-4o-mini)
- ~$0.00009 per rephrase
- 1,000 users × 10 rephrases/day = 10,000 requests
- Cost: **~$0.90/day** or **$27/month**

### Scaling Costs
| Users | Requests/Day | Cost/Month |
|-------|-------------|------------|
| 10 | 100 | $0.30 |
| 100 | 1,000 | $3.00 |
| 1,000 | 10,000 | $30.00 |
| 10,000 | 100,000 | $300.00 |

---

## Quick Start: Deploy for Friends Now

### 1. Deploy Proxy (5 minutes)

```bash
# Install Railway CLI
npm install -g @railway/cli
railway login

# Deploy
cd backend-proxy
npm install
railway init
railway up

# Set API key
railway variables set OPENAI_API_KEY=sk-your-key-here

# Get URL
railway domain
# Copy this URL - you'll need it!
```

### 2. Update App (5 minutes)

I'll help you modify the app to use your proxy instead of OpenAI directly.

### 3. Build & Share (2 minutes)

```bash
npm run tauri build
# Share the ZIP file with friends
```

---

## Recommended Approach

**For Friends (Next 2 weeks):**
- Use Railway free tier for proxy
- Share unsigned ZIP file
- Collect feedback

**For Public (After testing):**
- Upgrade Railway to paid ($5/mo)
- Get Apple Developer account ($99/year)
- Code sign the app
- Create simple landing page
- Launch!

---

Would you like me to:
1. **Set up the proxy deployment** (Railway/Heroku)
2. **Modify the app to use the proxy** (so API key stays secret)
3. **Create a simple landing page** template

Let me know which step to start with!

