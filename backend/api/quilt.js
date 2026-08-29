import { cors } from './_lib.js';

// Fetches Quilt version data from meta.quiltmc.org
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(200).end();

  try {
    const upstream = await fetch(
      'https://meta.quiltmc.org/2/game/versions',
      { headers: { 'User-Agent': 'TCC-Client/2.2.3' } }
    );

    if (!upstream.ok) {
      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Cache-Control', 'public, max-age=60');
      return res.end(JSON.stringify({ game_versions: [] }));
    }

    const data = await upstream.json();

    // Quilt meta has same format as Fabric: { "1.21": { stable: true, ... } }
    const game_versions = [];

    for (const [mcVersion, info] of Object.entries(data)) {
      if (!info || !info.stable) continue;

      const loaderList = [];
      if (info.loader && info.loader.version) {
        loaderList.push({
          id: info.loader.version,
          url: `https://meta.quiltmc.org/2/loader/${mcVersion}/${info.loader.version}/profile/json`,
          stable: info.loader.stable !== false
        });
      }

      if (info.loaders) {
        for (const loader of info.loaders) {
          if (loader.version) {
            loaderList.push({
              id: loader.version,
              url: `https://meta.quiltmc.org/2/loader/${mcVersion}/${loader.version}/profile/json`,
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
