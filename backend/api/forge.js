import { cors } from './_lib.js';

// Fetches Forge version data from files.minecraftforge.net and transforms it
// into the interfrost ModdedManifest format
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(200).end();

  try {
    // Forge uses a Maven-based metadata system
    // We fetch the promoted versions list
    const upstream = await fetch(
      'https://files.minecraftforge.net/maven/net/minecraftforge/forge/json',
      { headers: { 'User-Agent': 'TCC-Client/2.2.3' } }
    );

    if (!upstream.ok) {
      // Fallback: return empty manifest so launcher doesn't crash
      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Cache-Control', 'public, max-age=60');
      return res.end(JSON.stringify({ game_versions: [] }));
    }

    const data = await upstream.json();

    // Transform Forge format to interfrost ModdedManifest format
    const game_versions = [];
    for (const [mcVersion, loaders] of Object.entries(data)) {
      if (mcVersion === 'homepage' || mcVersion === 'webpage') continue;

      const loaderList = [];
      if (loaders && typeof loaders === 'object') {
        for (const [loaderId, info] of Object.entries(loaders)) {
          if (info && info.id) {
            loaderList.push({
              id: info.id || loaderId,
              url: info.url || `https://files.minecraftforge.net/maven/net/minecraftforge/forge/${info.id}/forge-${info.id}-installer.jar`,
              stable: true
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
    // Return empty manifest on error so launcher doesn't crash
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Cache-Control', 'public, max-age=60');
    res.end(JSON.stringify({ game_versions: [] }));
  }
}
