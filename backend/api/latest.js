import { cors, supabaseSelect, proxy, UPSTREAM_LATEST } from './_lib.js';

// GET /oneclient/latest.json
// Autoupdater feed. Once rows exist in the Supabase `releases` table the
// newest version per platform is served from there; until then we proxy the
// upstream feed so signatures (minisign) keep verifying.
export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(204).end();
  if (req.method !== 'GET' && req.method !== 'HEAD') return res.status(405).end();

  const rows = await supabaseSelect(
    'releases',
    'select=*&order=pub_date.desc&limit=50'
  );

  if (Array.isArray(rows) && rows.length > 0) {
    const newest = rows[0].pub_date;
    const platforms = {};
    for (const row of rows.filter((r) => r.pub_date === newest)) {
      platforms[row.platform] = {
        signature: row.signature || undefined,
        url: row.url,
      };
    }
    res.statusCode = 200;
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Cache-Control', 'public, max-age=120');
    return res.end(
      JSON.stringify({
        version: rows[0].version,
        pub_date: newest,
        platforms,
        notes: rows[0].notes || undefined,
      })
    );
  }

  return proxy(req, res, UPSTREAM_LATEST);
}
