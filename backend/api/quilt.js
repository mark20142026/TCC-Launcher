import { cors } from './_lib.js';

// Fetches Quilt version data from meta.quiltmc.org/v3
// Fetches game versions AND every stable loader version (newest first), so
// players can pick an older loader instead of only the latest one.
const FALLBACK_LOADERS = ['0.20.0'];

export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(204).end();

  try {
    // Fetch game versions
    const gameRes = await fetch(
      'https://meta.quiltmc.org/v3/versions/game',
      { headers: { 'User-Agent': 'TCC-Client/2.2.3' } }
    );

    if (!gameRes.ok) {
      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Cache-Control', 'public, max-age=60');
      return res.end(JSON.stringify({ gameVersions: [] }));
    }

    const gameData = await gameRes.json();
    if (!Array.isArray(gameData)) {
      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Cache-Control', 'public, max-age=60');
      return res.end(JSON.stringify({ gameVersions: [] }));
    }

    // Every loader version, newest first. The `stable` flag is ignored for
    // the same reason as in fabric.js: upstream only marks the latest one.
    let loaderVersions = [];
    try {
      const loaderRes = await fetch(
        'https://meta.quiltmc.org/v3/versions/loader',
        { headers: { 'User-Agent': 'TCC-Client/2.2.3' } }
      );
      if (loaderRes.ok) {
        const loaderData = await loaderRes.json();
        if (Array.isArray(loaderData)) {
          loaderVersions = loaderData
            .filter((l) => l && l.version)
            .map((l) => l.version);
        }
      }
    } catch {}

    if (loaderVersions.length === 0) {
      loaderVersions = FALLBACK_LOADERS;
    }

    const gameVersions = [];
    for (const entry of gameData) {
      if (!entry.stable) continue;
      gameVersions.push({
        id: entry.version,
        stable: true,
        loaders: loaderVersions.map((loaderVersion) => ({
          id: loaderVersion,
          url: `https://meta.quiltmc.org/v3/versions/loader/${entry.version}/${loaderVersion}/profile/json`,
          stable: true
        }))
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
