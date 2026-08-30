import { cors } from './_lib.js';

// Fetches Forge version data from Maven and transforms it
// into the interfrost ModdedManifest format
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(204).end();

  try {
    const upstream = await fetch(
      'https://maven.minecraftforge.net/net/minecraftforge/forge/maven-metadata.xml',
      { headers: { 'User-Agent': 'TCC-Client/2.2.3' } }
    );

    if (!upstream.ok) {
      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Cache-Control', 'public, max-age=60');
      return res.end(JSON.stringify({ gameVersions: [] }));
    }

    const xml = await upstream.text();

    const versions = [];
    const versionRegex = /<version>([^<]+)<\/version>/g;
    let match;
    while ((match = versionRegex.exec(xml)) !== null) {
      versions.push(match[1]);
    }

    const mcVersionMap = {};
    for (const v of versions) {
      const dashIdx = v.indexOf('-');
      if (dashIdx === -1) continue;
      const mcVersion = v.substring(0, dashIdx);
      if (!mcVersionMap[mcVersion]) mcVersionMap[mcVersion] = [];
      mcVersionMap[mcVersion].push(v);
    }

    const gameVersions = [];
    for (const [mcVersion, loaders] of Object.entries(mcVersionMap)) {
      const latestLoader = loaders[0];
      gameVersions.push({
        id: mcVersion,
        stable: true,
        loaders: [{
          id: latestLoader,
          url: `https://maven.minecraftforge.net/net/minecraftforge/forge/${latestLoader}/forge-${latestLoader}-installer.jar`,
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
