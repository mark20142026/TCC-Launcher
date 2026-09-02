import { cors, supabaseSelect, proxy, UPSTREAM_DATA } from './_lib.js';

export default async function handler(req, res) {
  cors(res);
  if (req.method === 'OPTIONS') return res.status(204).end();
  if (req.method !== 'GET' && req.method !== 'HEAD') return res.status(405).end();

  const url = new URL(req.url, `https://${req.headers.host}`);
  const isChangelog = url.pathname.includes('CHANGE_LOG');

  if (isChangelog) {
    const rows = await supabaseSelect('changelog', 'select=body_md&order=created_at.desc&limit=1');
    if (Array.isArray(rows) && rows.length > 0) {
      res.statusCode = 200;
      res.setHeader('Content-Type', 'text/markdown; charset=utf-8');
      res.setHeader('Cache-Control', 'public, max-age=60');
      return res.end(req.method === 'HEAD' ? undefined : rows[0].body_md);
    }
    return proxy(req, res, `${UPSTREAM_DATA}/oneclient/CHANGE_LOG.md`);
  }

  // TOS
  const rows = await supabaseSelect('terms', 'select=document&order=created_at.desc&limit=1');
  if (Array.isArray(rows) && rows.length > 0) {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Cache-Control', 'public, max-age=300');
    return res.end(req.method === 'HEAD' ? undefined : JSON.stringify(rows[0].document));
  }
  return proxy(req, res, `${UPSTREAM_DATA}/oneclient/tos.json`);
}
