import { cors, proxy, UPSTREAM_DATA } from './_lib.js';

// Catch-all: everything not handled by a specific function is proxied to the
// upstream data host (version metadata, bundle archives, art, etc.) so the
// launcher keeps working today; each surface can then be migrated into
// Supabase one endpoint at a time.
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(204).end();

  const path = req.query.path || '';
  const query = new URLSearchParams(req.query);
  query.delete('path');
  const qs = query.toString();
  const upstreamUrl = `${UPSTREAM_DATA}/${path}${qs ? `?${qs}` : ''}`;

  return proxy(req, res, upstreamUrl);
}
