import { cors } from './_lib.js';

// Fetches Fabric version data from meta.fabricmc.net/v2
// Fetches game versions AND every stable loader version from 0.18.5 up, so
// players can pick an older loader instead of only the latest one.
const MIN_LOADER_VERSION = [0, 18, 5];
// Used only when the loader metadata upstream is unreachable, so the endpoint
// still offers something installable.
const FALLBACK_LOADERS = ['0.16.14'];

function loaderTuple(version) {
  const core = String(version).split('+')[0].split('-')[0];
  const parts = core.split('.');
  if (parts.length < 2) return null;
  const nums = [];
  for (const part of parts) {
    const n = Number.parseInt(part, 10);
    if (!Number.isInteger(n) || n < 0) return null;
    nums.push(n);
  }
  while (nums.length < 3) nums.push(0);
  return nums;
}

function atLeast(version, min) {
  const tuple = loaderTuple(version);
  if (!tuple) return false;
  for (let i = 0; i < 3; i++) {
    if (tuple[i] !== min[i]) return tuple[i] > min[i];
  }
  return true;
}

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

    // Every stable loader from 0.18.5 up, newest first (upstream is sorted
    // newest first already; we keep that order).
    let loaderVersions = [];
    try {
      const loaderRes = await fetch(
        'https://meta.fabricmc.net/v2/versions/loader',
        { headers: { 'User-Agent': 'TCC-Client/2.2.3' } }
      );
      if (loaderRes.ok) {
        const loaderData = await loaderRes.json();
        if (Array.isArray(loaderData)) {
          loaderVersions = loaderData
            .filter((l) => l && l.stable && l.version && atLeast(l.version, MIN_LOADER_VERSION))
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
          url: `https://meta.fabricmc.net/v2/versions/loader/${entry.version}/${loaderVersion}/profile/json`,
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
