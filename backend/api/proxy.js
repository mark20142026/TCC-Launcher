import { cors, proxy, UPSTREAM_DATA } from './_lib.js';

// Metadata endpoints that need to be fetched from official sources
// instead of the upstream data host
const METADATA_LOADERS = {
  minecraft: {
    upstream: 'https://piston-meta.mojang.com/mc/game/version_manifest_v2.json',
  },
  forge: { upstream: 'https://maven.minecraftforge.net/net/minecraftforge/forge/maven-metadata.xml' },
  fabric: { upstream: 'https://meta.fabricmc.net/v2/versions/game' },
  neo: { upstream: 'https://maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml' },
  quilt: { upstream: 'https://meta.quiltmc.org/v3/versions/game' },
  legacyfabric: { upstream: 'https://meta.fabricmc.net/v2/versions/game' },
};

// Check if this is a metadata manifest request
function getMetadataLoader(path) {
  const match = path.match(/^([a-z]+)\/v\d+\/manifest\.json$/i);
  if (match && METADATA_LOADERS[match[1]]) {
    return match[1];
  }
  return null;
}

// Transform Fabric v2 game versions array to interfrost ModdedManifest format
// Input: [{version: "1.21", stable: true}, ...]
// Output: {game_versions: [{id: "1.21", stable: true, loaders: [{id, url, stable}]}]}
function transformFabricLike(data, loaderBase) {
  if (!Array.isArray(data)) return { gameVersions: [] };

  const game_versions = [];
  for (const entry of data) {
    if (!entry.stable) continue;

    const mcVersion = entry.version;
    // Use latest known stable loader version and construct profile URL
    game_versions.push({
      id: mcVersion,
      stable: true,
      loaders: [{
        id: 'latest',
        url: `${loaderBase}/${mcVersion}/latest/profile/json`,
        stable: true,
      }],
    });
  }
  return { gameVersions: game_versions };
}

// Transform Forge Maven XML to interfrost ModdedManifest format
function transformForgeMeta(xml) {
  const versions = [];
  const versionRegex = /<version>([^<]+)<\/version>/g;
  let match;
  while ((match = versionRegex.exec(xml)) !== null) {
    versions.push(match[1]);
  }

  // Group by MC version (format: "1.21-51.0.33")
  const mcVersionMap = {};
  for (const v of versions) {
    const dashIdx = v.indexOf('-');
    if (dashIdx === -1) continue;
    const mcVersion = v.substring(0, dashIdx);
    if (!mcVersionMap[mcVersion]) mcVersionMap[mcVersion] = [];
    mcVersionMap[mcVersion].push(v);
  }

  const game_versions = [];
  for (const [mcVersion, loaders] of Object.entries(mcVersionMap)) {
    const latestLoader = loaders[0]; // Maven metadata sorts newest first
    game_versions.push({
      id: mcVersion,
      stable: true,
      loaders: [{
        id: latestLoader,
        url: `https://maven.minecraftforge.net/net/minecraftforge/forge/${latestLoader}/forge-${latestLoader}-installer.jar`,
        stable: true,
      }],
    });
  }
  return { gameVersions: game_versions };
}

// Transform NeoForge Maven XML to interfrost ModdedManifest format
function transformNeoForgeMeta(xml) {
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

  const game_versions = [];
  for (const [mcVersion, loaders] of Object.entries(mcVersionMap)) {
    const latestLoader = loaders[0]; // Maven metadata sorts newest first
    game_versions.push({
      id: mcVersion,
      stable: true,
      loaders: [{
        id: latestLoader,
        url: `https://maven.neoforged.net/releases/net/neoforged/neoforge/${latestLoader}/neoforge-${latestLoader}-installer.jar`,
        stable: true,
      }],
    });
  }
  return { gameVersions: game_versions };
}

export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(204).end();

  const path = req.query.path || '';

  // Check if this is a metadata manifest request
  const loader = getMetadataLoader(path);
  if (loader) {
    try {
      const config = METADATA_LOADERS[loader];
      const upstream = await fetch(config.upstream, {
        headers: { 'User-Agent': 'TCC-Client/2.2.3' },
      });

      if (!upstream.ok) {
        res.setHeader('Content-Type', 'application/json');
        res.setHeader('Cache-Control', 'public, max-age=60');
        return res.end(JSON.stringify({ game_versions: [] }));
      }

      let data;
      if (loader === 'neo') {
        const xml = await upstream.text();
        data = transformNeoForgeMeta(xml);
      } else if (loader === 'forge') {
        const xml = await upstream.text();
        data = transformForgeMeta(xml);
      } else if (loader === 'fabric' || loader === 'legacyfabric') {
        const json = await upstream.json();
        data = transformFabricLike(json, 'https://meta.fabricmc.net/v2/versions/loader');
      } else if (loader === 'quilt') {
        const json = await upstream.json();
        data = transformFabricLike(json, 'https://meta.quiltmc.org/v3/versions/loader');
      } else {
        // minecraft - Mojang format matches directly
        data = await upstream.json();
      }

      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Cache-Control', 'public, max-age=300');
      return res.end(JSON.stringify(data));
    } catch (err) {
      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Cache-Control', 'public, max-age=60');
      return res.end(JSON.stringify({ game_versions: [] }));
    }
  }

  // Default: proxy to upstream data host
  const query = new URLSearchParams(req.query);
  query.delete('path');
  const qs = query.toString();
  const upstreamUrl = `${UPSTREAM_DATA}/${path}${qs ? `?${qs}` : ''}`;

  return proxy(req, res, upstreamUrl);
}
