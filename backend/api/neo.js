import { cors } from './_lib.js';

// Fetches NeoForge version data from maven.neoforged.net
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(200).end();

  try {
    // NeoForge metadata endpoint
    const upstream = await fetch(
      'https://maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml',
      { headers: { 'User-Agent': 'TCC-Client/2.2.3' } }
    );

    if (!upstream.ok) {
      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Cache-Control', 'public, max-age=60');
      return res.end(JSON.stringify({ game_versions: [] }));
    }

    const xml = await upstream.text();

    // Parse versions from Maven metadata XML
    const versions = [];
    const versionRegex = /<version>([^<]+)<\/version>/g;
    let match;
    while ((match = versionRegex.exec(xml)) !== null) {
      versions.push(match[1]);
    }

    // Group by MC version (format: {mcVersion}-{neoVersion})
    const mcVersionMap = {};
    for (const v of versions) {
      // NeoForge versions like "1.21-21.0.1" or "21.0.1"
      const parts = v.split('-');
      if (parts.length >= 2) {
        const mcVersion = parts[0];
        if (!mcVersionMap[mcVersion]) mcVersionMap[mcVersion] = [];
        mcVersionMap[mcVersion].push(v);
      } else {
        // Try to extract MC version from the neo version
        // NeoForge 21.x = MC 1.21, 20.x = MC 1.20.x
        const major = parseInt(parts[0].split('.')[0]);
        let mcVer;
        if (major >= 21) mcVer = `1.${major - 0}`;
        else if (major >= 20) mcVer = `1.20`;
        else continue;

        if (!mcVersionMap[mcVer]) mcVersionMap[mcVer] = [];
        mcVersionMap[mcVer].push(v);
      }
    }

    const game_versions = [];
    for (const [mcVersion, loaders] of Object.entries(mcVersionMap)) {
      // Take the latest loader version for each MC version
      const latestLoader = loaders[loaders.length - 1];
      game_versions.push({
        id: mcVersion,
        stable: true,
        loaders: [{
          id: latestLoader,
          url: `https://maven.neoforged.net/releases/net/neoforged/neoforge/${latestLoader}/neoforge-${latestLoader}-installer.jar`,
          stable: true
        }]
      });
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
