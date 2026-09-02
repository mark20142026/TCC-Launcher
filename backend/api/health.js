import { cors } from './_lib.js';

export default async function handler(req, res) {
  cors(res);
  const url = new URL(req.url, `https://${req.headers.host}`);

  // /status/index.json — launcher connectivity check
  if (url.pathname.includes('status')) {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Cache-Control', 'public, max-age=30');
    return res.end(JSON.stringify({
      data: { attributes: { aggregate_state: 'operational' } },
    }));
  }

  // /health
  res.statusCode = 200;
  res.setHeader('Content-Type', 'application/json');
  return res.end(JSON.stringify({ ok: true, service: 'tcc-backend', time: new Date().toISOString() }));
}
