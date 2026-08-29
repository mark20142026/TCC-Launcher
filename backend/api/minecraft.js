import { cors } from './_lib.js';

// Proxies Mojang's vanilla version manifest
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(200).end();

  try {
    const upstream = await fetch(
      'https://piston-meta.mojang.com/mc/game/version_manifest_v2.json',
      { headers: { 'User-Agent': 'TCC-Client/2.2.3' } }
    );
    if (!upstream.ok) {
      res.statusCode = upstream.status;
      return res.end('Upstream returned ' + upstream.status);
    }
    const data = await upstream.json();
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Cache-Control', 'public, max-age=300');
    res.end(JSON.stringify(data));
  } catch (err) {
    res.statusCode = 502;
    res.end('Failed to fetch vanilla manifest: ' + err.message);
  }
}
