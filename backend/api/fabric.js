import { cors } from './_lib.js';

// Fetches Fabric version data from meta.fabricmc.net/v2
// Fetches game versions AND the latest loader version for each
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(204).end();

  try {
    // Fetch game versions
    const gameRes = await fetch(
      'https://meta.fabricmc.net/v2/versions/game',
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

    // Get latest stable loader version (fetch once for one game version)
    let latestLoaderVersion = '0.16.14'; // fallback
    try {
      const loaderRes = await fetch(
        'https://meta.fabricmc.net/v2/versions/loader',
        { headers: { 'User-Agent': 'TCC-Client/2.2.3' } }
      );
      if (loaderRes.ok) {
        const loaderData = await loaderRes.json();
        if (Array.isArray(loaderData) && loaderData.length > 0) {
          const stableLoader = loaderData.find(l => l.stable) || loaderData[0];
          if (stableLoader && stableLoader.version) {
            latestLoaderVersion = stableLoader.version;
          }
        }
      }
    } catch {}

    const gameVersions = [];
    for (const entry of gameData) {
      if (!entry.stable) continue;
      gameVersions.push({
        id: entry.version,
        stable: true,
        loaders: [{
          id: latestLoaderVersion,
          url: `https://meta.fabricmc.net/v2/versions/loader/${entry.version}/${latestLoaderVersion}/profile/json`,
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
