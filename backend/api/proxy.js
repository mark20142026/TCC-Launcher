import { cors, proxy, UPSTREAM_DATA } from './_lib.js';

// Metadata endpoints that need to be fetched from official sources
// instead of the upstream data host
const METADATA_LOADERS = {
  minecraft: {
    upstream: 'https://piston-meta.mojang.com/mc/game/version_manifest_v2.json',
    transform: null // No transform needed, Mojang format matches
  },
  forge: { upstream: 'https://files.minecraftforge.net/maven/net/minecraftforge/forge/json' },
  fabric: { upstream: 'https://meta.fabricmc.net/2/game/versions' },
  neo: { upstream: 'https://maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml' },
  quilt: { upstream: 'https://meta.quiltmc.org/2/game/versions' }
};

// Check if this is a metadata manifest request
function getMetadataLoader(path) {
  const match = path.match(/^([a-z]+)\/v\d+\/manifest\.json$/i);
  if (match && METADATA_LOADERS[match[1]]) {
    return match[1];
  }
  return null;
}

// Transform Fabric/Quilt meta format to interfrost ModdedManifest format
function transformFabricMeta(data) {
  const game_versions = [];
  for (const [mcVersion, info] of Object.entries(data)) {
    if (!info || typeof info !== 'object') continue;
    if (info.stable === false) continue;

    const loaderList = [];
    if (info.loader && info.loader.version) {
      loaderList.push({
        id: info.loader.version,
        url: `https://meta.fabricmc.net/2/loader/${mcVersion}/${info.loader.version}/profile/json`,
        stable: info.loader.stable !== false
      });
    }
    if (info.loaders && Array.isArray(info.loaders)) {
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
      game_versions.push({ id: mcVersion, stable: true, loaders: loaderList });
    }
  }
  return { game_versions };
}

// Transform Forge JSON format to interfrost ModdedManifest format
function transformForgeMeta(data) {
  const game_versions = [];
  for (const [mcVersion, loaders] of Object.entries(data)) {
    if (mcVersion === 'homepage' || mcVersion === 'webpage') continue;
    if (!loaders || typeof loaders !== 'object') continue;

    const loaderList = [];
    for (const [loaderId, info] of Object.entries(loaders)) {
      if (info && info.id) {
        loaderList.push({
          id: info.id || loaderId,
          url: info.url || `https://files.minecraftforge.net/maven/net/minecraftforge/forge/${info.id}/forge-${info.id}-installer.jar`,
          stable: true
        });
      }
    }
    if (loaderList.length > 0) {
      game_versions.push({ id: mcVersion, stable: true, loaders: loaderList });
    }
  }
  return { game_versions };
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
    const parts = v.split('-');
    if (parts.length >= 2) {
      const mcVersion = parts[0];
      if (!mcVersionMap[mcVersion]) mcVersionMap[mcVersion] = [];
      mcVersionMap[mcVersion].push(v);
    }
  }

  const game_versions = [];
  for (const [mcVersion, loaders] of Object.entries(mcVersionMap)) {
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
  return { game_versions };
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
        headers: { 'User-Agent': 'TCC-Client/2.2.3' }
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
        const json = await upstream.json();
        data = transformForgeMeta(json);
      } else if (loader === 'fabric' || loader === 'quilt') {
        const json = await upstream.json();
        data = transformFabricMeta(json);
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
