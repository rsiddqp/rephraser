// Rephraser API Proxy Server
// Keeps your OpenAI API key secure server-side

const express = require('express');
const cors = require('cors');
const axios = require('axios');
require('dotenv').config();

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(express.json({ limit: '1mb' }));

// Rate limiting (simple in-memory, use Redis for production)
const rateLimitMap = new Map();
const RATE_LIMIT_WINDOW = 60000; // 1 minute
const MAX_REQUESTS_PER_WINDOW = 20; // 20 requests per minute per IP

function checkRateLimit(ip) {
  const now = Date.now();
  const userRequests = rateLimitMap.get(ip) || [];
  
  // Remove old requests outside the window
  const recentRequests = userRequests.filter(time => now - time < RATE_LIMIT_WINDOW);
  
  if (recentRequests.length >= MAX_REQUESTS_PER_WINDOW) {
    return false; // Rate limited
  }
  
  recentRequests.push(now);
  rateLimitMap.set(ip, recentRequests);
  return true;
}

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({ status: 'ok', service: 'rephraser-proxy' });
});

// Rephrase endpoint
app.post('/api/rephrase', async (req, res) => {
  const clientIp = req.ip || req.connection.remoteAddress;
  
  // Rate limiting
  if (!checkRateLimit(clientIp)) {
    return res.status(429).json({ 
      error: 'Rate limit exceeded. Please wait a moment and try again.' 
    });
  }
  
  const { text, style } = req.body;
  
  // Validation
  if (!text || typeof text !== 'string') {
    return res.status(400).json({ error: 'Text is required' });
  }
  
  if (text.length > 10000) {
    return res.status(400).json({ error: 'Text too long. Maximum 10,000 characters.' });
  }
  
  if (!['professional', 'casual', 'sarcasm'].includes(style)) {
    return res.status(400).json({ error: 'Invalid style. Must be: professional, casual, or sarcasm' });
  }
  
  // Get prompt based on style
  const prompts = {
    professional: `Rephrase the following text in a professional, formal tone suitable for business communication. Maintain the core message but improve clarity and professionalism. Do not add any preamble or explanation, just return the rephrased text:\n\n${text}`,
    casual: `Rephrase the following text in a casual, friendly tone suitable for informal communication. Make it conversational and approachable. Do not add any preamble or explanation, just return the rephrased text:\n\n${text}`,
    sarcasm: `Rephrase the following text with subtle sarcasm while maintaining the surface-level message. Keep it witty but not offensive. Do not add any preamble or explanation, just return the rephrased text:\n\n${text}`
  };
  
  try {
    // Calculate max_tokens dynamically
    const estimatedTokens = Math.ceil(text.length / 4);
    const maxTokens = Math.max(150, Math.min(2000, Math.ceil(estimatedTokens * 1.5)));
    
    // Call OpenAI API (API key stored server-side)
    const response = await axios.post(
      'https://api.openai.com/v1/chat/completions',
      {
        model: 'gpt-4o-mini',
        messages: [
          {
            role: 'system',
            content: 'You are a professional writing assistant that helps rephrase text in different styles. Always return only the rephrased text without any additional explanation or preamble.'
          },
          {
            role: 'user',
            content: prompts[style]
          }
        ],
        temperature: 0.7,
        max_tokens: maxTokens
      },
      {
        headers: {
          'Authorization': `Bearer ${process.env.OPENAI_API_KEY}`,
          'Content-Type': 'application/json'
        },
        timeout: 30000
      }
    );
    
    const rephrased = response.data.choices[0]?.message?.content?.trim();
    
    if (!rephrased) {
      throw new Error('No response from OpenAI');
    }
    
    res.json({ rephrased });
    
  } catch (error) {
    console.error('OpenAI API Error:', error.response?.data || error.message);
    
    if (error.response) {
      const status = error.response.status;
      if (status === 401) {
        return res.status(500).json({ error: 'Server configuration error' });
      } else if (status === 429) {
        return res.status(429).json({ error: 'Service is busy. Please try again in a moment.' });
      } else if (status >= 500) {
        return res.status(503).json({ error: 'AI service temporarily unavailable' });
      }
    }
    
    res.status(500).json({ error: 'Failed to rephrase text. Please try again.' });
  }
});

// Error handling
app.use((err, req, res, next) => {
  console.error('Server error:', err);
  res.status(500).json({ error: 'Internal server error' });
});

app.listen(PORT, () => {
  console.log(`🚀 Rephraser Proxy Server running on port ${PORT}`);
  console.log(`✅ OpenAI API Key: ${process.env.OPENAI_API_KEY ? 'Loaded' : '❌ Missing'}`);
});

