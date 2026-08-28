import { cors } from './_lib.js';

export default async function handler(req, res) {
  cors(res);
  res.statusCode = 200;
  res.setHeader('Content-Type', 'application/json');
  res.end(JSON.stringify({ ok: true, service: 'tcc-backend', time: new Date().toISOString() }));
}
