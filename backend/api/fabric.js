import { cors } from './_lib.js';

// Fetches Fabric version data from meta.fabricmc.net
// and transforms it into the interfrost ModdedManifest format
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(200).end();

  try {
    const upstream = await fetch(
      'https://meta.fabricmc.net/2/game/versions',
      { headers: { 'User-Agent': 'TCC-Client/2.2.3' } }
    );

    if (!upstream.ok) {
      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Cache-Control', 'public, max-age=60');
      return res.end(JSON.stringify({ game_versions: [] }));
    }

    const data = await upstream.json();

    // Fabric meta returns { "1.21": { ... }, "1.20.4": { ... } }
    // We need to transform to { game_versions: [{ id, stable, loaders: [...] }] }
    const game_versions = [];

    for (const [mcVersion, info] of Object.entries(data)) {
      if (!info || !info.stable) continue; // Only stable game versions

      const loaderList = [];
      // Fabric meta includes "stable" and "latest_stable" etc.
      // The actual loader info is nested
      if (info.loader && info.loader.version) {
        loaderList.push({
          id: info.loader.version,
          url: `https://meta.fabricmc.net/2/loader/${mcVersion}/${info.loader.version}/profile/json`,
          stable: info.loader.stable !== false
        });
      }

      // Also check for multiple loader versions
      if (info.loaders) {
        for (const loader of info.loaders) {
          if (loader.version) {
            loaderList.push({
              id: loader.version,
              url: `https://meta.fabricmc.net/2/loader/${mcVersion}/${loader.version}/profile/json`,
              stable: loader.stable !== false
            });
          }
        }
      }

      if (loaderList.length > 0) {
        game_versions.push({
          id: mcVersion,
          stable: true,
          loaders: loaderList
        });
      }
    }

    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Cache-Control', 'public, max-age=300');
    res.end(JSON.stringify({ game_versions }));
  } catch (err) {
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Cache-Control', 'public, max-age=60');
    res.end(JSON.stringify({ game_versions: [] }));
  }
}
