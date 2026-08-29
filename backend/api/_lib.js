// Shared helpers for the TCC backend functions (no dependencies, Node 18+).
import { Readable } from 'node:stream';

const SUPABASE_URL = process.env.SUPABASE_URL;
const SUPABASE_SECRET_KEY = process.env.SUPABASE_SECRET_KEY;

// Upstream we fall back to / proxy for content that has not been migrated
// into Supabase yet (version metadata, bundles, art, updater feed).
export const UPSTREAM_META = 'https://meta.theazizi.space';
export const UPSTREAM_DATA = 'https://data-v2.polyfrost.org';
export const UPSTREAM_LATEST =
  'https://github.com/Polyfrost/OneLauncher/releases/latest/download/latest.json';

export function cors(res) {
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'If-None-Match, Range');
}

export async function supabaseSelect(table, searchParams) {
  if (!SUPABASE_URL || !SUPABASE_SECRET_KEY) return null;
  const qs = searchParams ? `?${searchParams}` : '';
  const res = await fetch(`${SUPABASE_URL}/rest/v1/${table}${qs}`, {
    headers: {
      apikey: SUPABASE_SECRET_KEY,
      Authorization: `Bearer ${SUPABASE_SECRET_KEY}`,
      Accept: 'application/json',
    },
  });
  if (!res.ok) return null;
  return res.json();
}

export async function proxy(req, res, upstreamUrl) {
  const headers = {};
  const passthrough = ['if-none-match', 'if-modified-since', 'range', 'accept', 'user-agent'];
  for (const key of passthrough) {
    const value = req.headers[key];
    if (value) headers[key] = value;
  }
  // Ask for the identity encoding so upstream's content-length matches the
  // body we stream; fetch's auto-decompression would otherwise desync them.
  headers['accept-encoding'] = 'identity';

  const method = req.method === 'HEAD' ? 'HEAD' : 'GET';
  const upstream = await fetch(upstreamUrl, { method, headers, redirect: 'follow' });

  res.statusCode = upstream.status;
  for (const key of ['content-type', 'content-length', 'etag', 'last-modified', 'content-range', 'accept-ranges']) {
    const value = upstream.headers.get(key);
    if (value) res.setHeader(key, value);
  }
  res.setHeader('Cache-Control', 'public, max-age=60');

  if (method === 'HEAD' || !upstream.body) {
    res.end();
    return;
  }
  Readable.fromWeb(upstream.body).pipe(res);
}
