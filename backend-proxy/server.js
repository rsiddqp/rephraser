const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const morgan = require('morgan');
const axios = require('axios');
require('dotenv').config();

const app = express();
const PORT = process.env.PORT || 3000;
const IS_PRODUCTION = process.env.NODE_ENV === 'production';

// ---------------------------------------------------------------------------
// Redis (optional) – falls back to in-memory Map when REDIS_URL is absent
// ---------------------------------------------------------------------------
let redis = null;

async function initRedis() {
  if (!process.env.REDIS_URL) return;
  try {
    const Redis = require('ioredis');
    redis = new Redis(process.env.REDIS_URL, {
      maxRetriesPerRequest: 3,
      retryStrategy(times) {
        if (times > 5) return null; // stop retrying
        return Math.min(times * 200, 2000);
      },
    });
    redis.on('error', (err) => {
      console.error('[redis] connection error – falling back to in-memory', err.message);
      redis = null;
    });
    await redis.ping();
    console.log('[redis] connected');
  } catch (err) {
    console.warn('[redis] unavailable – using in-memory rate limiting', err.message);
    redis = null;
  }
}

// ---------------------------------------------------------------------------
// Security middleware
// ---------------------------------------------------------------------------
app.use(helmet());
app.set('trust proxy', 1);

// CORS – lock down in production if ALLOWED_ORIGINS is set
const allowedOrigins = process.env.ALLOWED_ORIGINS
  ? process.env.ALLOWED_ORIGINS.split(',').map((o) => o.trim())
  : null;

app.use(
  cors({
    origin: allowedOrigins || true,
    methods: ['GET', 'POST'],
    allowedHeaders: ['Content-Type'],
  })
);

app.use(express.json({ limit: '1mb' }));

// ---------------------------------------------------------------------------
// Logging
// ---------------------------------------------------------------------------
if (IS_PRODUCTION) {
  app.use(
    morgan(':date[iso] :method :url :status :response-time ms - :remote-addr', {
      skip: (req) => req.path === '/health',
    })
  );
} else {
  app.use(morgan('dev'));
}

// ---------------------------------------------------------------------------
// Rate limiting – Redis-backed with in-memory fallback
// ---------------------------------------------------------------------------
const RATE_LIMIT_WINDOW = parseInt(process.env.RATE_LIMIT_WINDOW, 10) || 60_000;
const MAX_REQUESTS_PER_WINDOW = parseInt(process.env.MAX_REQUESTS_PER_WINDOW, 10) || 20;
const inMemoryRateMap = new Map();

setInterval(() => {
  const now = Date.now();
  for (const [ip, timestamps] of inMemoryRateMap.entries()) {
    const recent = timestamps.filter((t) => now - t < RATE_LIMIT_WINDOW);
    if (recent.length === 0) inMemoryRateMap.delete(ip);
    else inMemoryRateMap.set(ip, recent);
  }
}, RATE_LIMIT_WINDOW);

async function checkRateLimit(ip) {
  if (redis) {
    try {
      const key = `rl:${ip}`;
      const count = await redis.incr(key);
      if (count === 1) {
        await redis.pexpire(key, RATE_LIMIT_WINDOW);
      }
      return count <= MAX_REQUESTS_PER_WINDOW;
    } catch {
      // Redis failed mid-request; fall through to in-memory
    }
  }

  const now = Date.now();
  const timestamps = (inMemoryRateMap.get(ip) || []).filter(
    (t) => now - t < RATE_LIMIT_WINDOW
  );
  if (timestamps.length >= MAX_REQUESTS_PER_WINDOW) return false;
  timestamps.push(now);
  inMemoryRateMap.set(ip, timestamps);
  return true;
}

// ---------------------------------------------------------------------------
// Request validation middleware
// ---------------------------------------------------------------------------
const VALID_STYLES = new Set(['professional', 'casual', 'sarcasm']);
const MAX_TEXT_LENGTH = 10_000;

function validateRephraseBody(req, res, next) {
  const { text, style } = req.body;

  if (!text || typeof text !== 'string') {
    return res.status(400).json({ error: 'Text is required and must be a string.' });
  }
  const trimmed = text.trim();
  if (trimmed.length === 0) {
    return res.status(400).json({ error: 'Text must not be empty.' });
  }
  if (trimmed.length > MAX_TEXT_LENGTH) {
    return res.status(400).json({
      error: `Text too long (${trimmed.length} chars). Maximum is ${MAX_TEXT_LENGTH}.`,
    });
  }
  if (!style || !VALID_STYLES.has(style)) {
    return res.status(400).json({
      error: `Invalid style "${style}". Must be one of: ${[...VALID_STYLES].join(', ')}`,
    });
  }

  req.body.text = trimmed;
  next();
}

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

// Health check — useful for Heroku, load balancers, uptime monitors
app.get('/health', (_req, res) => {
  res.json({
    status: 'ok',
    service: 'rephraser-proxy',
    version: '2.0.0',
    uptime: Math.floor(process.uptime()),
    redis: redis ? 'connected' : 'unavailable',
    timestamp: new Date().toISOString(),
  });
});

// Convenience root route
app.get('/', (_req, res) => {
  res.json({
    service: 'rephraser-proxy',
    version: '2.0.0',
    endpoints: {
      health: 'GET /health',
      rephrase: 'POST /api/rephrase',
    },
  });
});

// Rephrase endpoint
app.post('/api/rephrase', validateRephraseBody, async (req, res) => {
  const clientIp = req.ip || req.socket.remoteAddress;

  if (!(await checkRateLimit(clientIp))) {
    return res.status(429).json({
      error: 'Rate limit exceeded. Please wait a moment and try again.',
    });
  }

  const { text, style } = req.body;

  const prompts = {
    professional:
      'Rephrase the following text in a professional, formal tone suitable for business communication. ' +
      'Maintain the core message but improve clarity and professionalism. ' +
      'Do not add any preamble or explanation, just return the rephrased text:\n\n' +
      text,
    casual:
      'Rephrase the following text in a casual, friendly tone suitable for informal communication. ' +
      'Make it conversational and approachable. ' +
      'Do not add any preamble or explanation, just return the rephrased text:\n\n' +
      text,
    sarcasm:
      'Rephrase the following text with subtle sarcasm while maintaining the surface-level message. ' +
      'Keep it witty but not offensive. ' +
      'Do not add any preamble or explanation, just return the rephrased text:\n\n' +
      text,
  };

  try {
    const estimatedTokens = Math.ceil(text.length / 4);
    const maxTokens = Math.max(150, Math.min(2000, Math.ceil(estimatedTokens * 1.5)));

    const response = await axios.post(
      'https://api.openai.com/v1/chat/completions',
      {
        model: 'gpt-4o-mini',
        messages: [
          {
            role: 'system',
            content:
              'You are a professional writing assistant that helps rephrase text in different styles. ' +
              'Always return only the rephrased text without any additional explanation or preamble.',
          },
          { role: 'user', content: prompts[style] },
        ],
        temperature: 0.7,
        max_tokens: maxTokens,
      },
      {
        headers: {
          Authorization: `Bearer ${process.env.OPENAI_API_KEY}`,
          'Content-Type': 'application/json',
        },
        timeout: 30_000,
      }
    );

    const rephrased = response.data.choices?.[0]?.message?.content?.trim();
    if (!rephrased) {
      throw new Error('Empty response from OpenAI');
    }

    res.json({ rephrased });
  } catch (error) {
    const status = error.response?.status;
    const detail = error.response?.data?.error?.message || error.message;
    console.error(`[rephrase] error – status=${status} detail=${detail}`);

    if (status === 401) {
      return res.status(500).json({ error: 'Server configuration error.' });
    }
    if (status === 429) {
      return res.status(429).json({ error: 'AI service is busy. Please try again in a moment.' });
    }
    if (status >= 500) {
      return res.status(503).json({ error: 'AI service temporarily unavailable.' });
    }

    res.status(500).json({ error: 'Failed to rephrase text. Please try again.' });
  }
});

// 404 catch-all
app.use((_req, res) => {
  res.status(404).json({ error: 'Not found' });
});

// Global error handler
app.use((err, _req, res, _next) => {
  console.error('[server] unhandled error:', err);
  res.status(500).json({ error: 'Internal server error' });
});

// ---------------------------------------------------------------------------
// Start
// ---------------------------------------------------------------------------
async function start() {
  await initRedis();

  app.listen(PORT, () => {
    console.log(`[server] Rephraser proxy v2.0.0 listening on port ${PORT}`);
    console.log(`[server] environment: ${IS_PRODUCTION ? 'production' : 'development'}`);
    console.log(`[server] OpenAI key: ${process.env.OPENAI_API_KEY ? 'loaded' : 'MISSING'}`);
    console.log(`[server] rate limit: ${MAX_REQUESTS_PER_WINDOW} req / ${RATE_LIMIT_WINDOW}ms`);
    console.log(`[server] redis: ${redis ? 'connected' : 'not configured (in-memory fallback)'}`);
  });
}

start().catch((err) => {
  console.error('[server] failed to start:', err);
  process.exit(1);
});
