import { cors, supabaseSelect, proxy, UPSTREAM_DATA } from './_lib.js';

// GET /oneclient/CHANGE_LOG.md
// Serves the newest changelog from the Supabase `changelog` table as plain
// markdown. Falls back to the upstream file until the table has content.
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(204).end();
  if (req.method !== 'GET' && req.method !== 'HEAD') return res.status(405).end();

  const rows = await supabaseSelect(
    'changelog',
    'select=body_md&order=created_at.desc&limit=1'
  );

  if (Array.isArray(rows) && rows.length > 0) {
    const body = rows[0].body_md;
    res.statusCode = 200;
    res.setHeader('Content-Type', 'text/markdown; charset=utf-8');
    res.setHeader('Cache-Control', 'public, max-age=60');
    return res.end(req.method === 'HEAD' ? undefined : body);
  }

  return proxy(req, res, `${UPSTREAM_DATA}/oneclient/CHANGE_LOG.md`);
}
