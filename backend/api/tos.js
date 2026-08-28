import { cors, supabaseSelect, proxy, UPSTREAM_DATA } from './_lib.js';

// GET /oneclient/tos.json
// Terms document consumed by the onboarding flow. Served from the Supabase
// `terms` table; falls back to upstream until the table has content.
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(204).end();
  if (req.method !== 'GET' && req.method !== 'HEAD') return res.status(405).end();

  const rows = await supabaseSelect(
    'terms',
    'select=document&order=created_at.desc&limit=1'
  );

  if (Array.isArray(rows) && rows.length > 0) {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Cache-Control', 'public, max-age=300');
    return res.end(req.method === 'HEAD' ? undefined : JSON.stringify(rows[0].document));
  }

  return proxy(req, res, `${UPSTREAM_DATA}/oneclient/tos.json`);
}
