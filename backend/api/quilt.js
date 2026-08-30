import { cors } from './_lib.js';

// Fetches Quilt version data from meta.quiltmc.org/v3
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(204).end();

  try {
    const upstream = await fetch(
      'https://meta.quiltmc.org/v3/versions/game',
      { headers: { 'User-Agent': 'TCC-Client/2.2.3' } }
    );

    if (!upstream.ok) {
      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Cache-Control', 'public, max-age=60');
      return res.end(JSON.stringify({ gameVersions: [] }));
    }

    const data = await upstream.json();

    if (!Array.isArray(data)) {
      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Cache-Control', 'public, max-age=60');
      return res.end(JSON.stringify({ gameVersions: [] }));
    }

    const gameVersions = [];
    for (const entry of data) {
      if (!entry.stable) continue;
      gameVersions.push({
        id: entry.version,
        stable: true,
        loaders: [{
          id: 'latest',
          url: `https://meta.quiltmc.org/v3/versions/loader/${entry.version}/latest/profile/json`,
          stable: true
        }]
      });
    }

    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Cache-Control', 'public, max-age=300');
    res.end(JSON.stringify({ gameVersions }));
  } catch (err) {
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Cache-Control', 'public, max-age=60');
    res.end(JSON.stringify({ gameVersions: [] }));
  }
}
