# 🚀 Deploy Rephraser in 10 Minutes

Everything is ready! Just follow these 3 simple steps.

---

## Step 1: Deploy Backend (3 minutes)

### Option A: Render.com (Recommended)

1. **Go to**: https://dashboard.render.com/register
2. **Click "GitHub"** to sign up
3. Once logged in, click **"New +"** → **"Web Service"**
4. Select **"Build and deploy from a Git repository"**
5. **Public Repository URL**: 
   - First push your code to GitHub (see commands below)
   - Or use **"Deploy from public repository"** and paste your GitHub repo URL

6. **Settings:**
   - Name: `rephraser-proxy`
   - Root Directory: `backend-proxy`
   - Build Command: `npm install`
   - Start Command: `node server.js`

7. **Environment Variable:**
   - Click "Advanced" → "Add Environment Variable"
   - Key: `OPENAI_API_KEY`
   - Value: `your-openai-api-key-here`

8. **Click "Create Web Service"**

9. **Done!** Your proxy URL: `https://rephraser-proxy.onrender.com`

### Quick Commands to Push to GitHub:

```bash
cd "/Users/rahil/test project"

# Create new GitHub repo at: https://github.com/new
# Then run:
git remote add origin https://github.com/YOUR-USERNAME/rephraser.git
git push -u origin main
```

---

## Step 2: Update App to Use Proxy (I'll do this for you!)

Just tell me your proxy URL from Step 1, and I'll:
- Modify `src-tauri/src/ai.rs` to use your proxy
- Remove API key requirement from the app
- Rebuild the production app

---

## Step 3: Deploy Landing Page (2 minutes)

### Using Vercel (Easiest)

1. **Go to**: https://vercel.com/new
2. **Sign up** with GitHub
3. Click **"Add New"** → **"Project"**
4. **Import** the `landing-page/` folder
5. **Deploy!**
6. Your landing page: `https://rephraser.vercel.app`

### Using GitHub Pages (Free)

1. Create new repo: `rephraser-landing`
2. Copy `landing-page/index.html` to repo
3. Upload your rebuilt `Rephraser.zip`
4. Settings → Pages → Enable
5. Your landing page: `https://YOUR-USERNAME.github.io/rephraser-landing`

---

## Summary

After these 3 steps, you'll have:
- ✅ Backend API running (with your key secure)
- ✅ Desktop app that anyone can download
- ✅ Landing page for distribution

**Ready?** Start with Step 1 (takes 3 mins), then tell me your proxy URL!

