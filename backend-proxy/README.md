# Rephraser Backend Proxy

Secure proxy server that handles OpenAI API calls, keeping the API key server-side.

## Quick Deploy to Railway

1. Install Railway CLI:
```bash
npm install -g @railway/cli
```

2. Login:
```bash
railway login
```

3. Deploy:
```bash
railway init
railway up
```

4. Set environment variable:
```bash
railway variables set OPENAI_API_KEY=your-key-here
```

5. Add domain:
```bash
railway domain
```

## Environment Variables

- `OPENAI_API_KEY` - Your OpenAI API key (required)
- `PORT` - Server port (optional, defaults to 3000)

## API Endpoints

### POST /api/rephrase
Rephrase text in different styles.

**Request:**
```json
{
  "text": "hello world",
  "style": "professional"
}
```

**Response:**
```json
{
  "rephrased": "Hello, world."
}
```

**Styles:** `professional`, `casual`, `sarcasm`

