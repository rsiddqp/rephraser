# Quick Public Deployment Guide

## Step 1: Deploy Backend Proxy (5 minutes)

### Using Render.com (Easiest - No CLI needed)

1. **Go to**: https://render.com
2. **Sign up** with GitHub
3. Click **"New +"** → **"Web Service"**
4. Connect your GitHub repo or:
   - Click **"Public Git Repository"**
   - Paste: `https://github.com/YOUR-USERNAME/rephraser` (or use direct deployment below)

5. **Configure:**
   - Name: `rephraser-proxy`
   - Root Directory: `backend-proxy`
   - Build Command: `npm install`
   - Start Command: `node server.js`

6. **Add Environment Variable:**
   - Key: `OPENAI_API_KEY`
   - Value: `sk-your-actual-api-key-here`

7. **Deploy!** (takes ~2 minutes)

8. **Your proxy URL**: `https://rephraser-proxy.onrender.com`

---

## Step 2: Update Desktop App to Use Proxy

I'll modify the app code for you. Just provide me the proxy URL from Step 1.

---

## Step 3: Deploy Landing Page

### Using GitHub Pages (Free)

1. Create new repo: `rephraser-landing`
2. Copy `landing-page/index.html` to the repo
3. Upload your `Rephraser.zip` file
4. Enable GitHub Pages in repo settings
5. Your landing page: `https://YOUR-USERNAME.github.io/rephraser-landing`

### Using Vercel (Easier)

1. Go to: https://vercel.com
2. Sign up with GitHub
3. **New Project** → Import `landing-page/` folder
4. Deploy → Done!
5. Your landing page: `https://rephraser.vercel.app`

---

## Alternative: One-Click Render Deploy

I've created a `render.yaml` file. You can deploy the backend with one click:

1. Push code to GitHub
2. Go to https://dashboard.render.com/select-repo
3. Select your repo
4. Render auto-detects the config
5. Add your `OPENAI_API_KEY`
6. Deploy!

---

## What do you want me to do?

Please tell me:
1. **Did you deploy the backend?** If yes, give me the URL
2. **Where do you want the landing page?** (GitHub Pages or Vercel)

I'll handle the rest!

