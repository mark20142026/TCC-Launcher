const fs = require('fs');
const path = require('path');

module.exports = async (req, res) => {
  try {
    const host = (req.headers.host || req.headers['x-forwarded-host'] || '').toLowerCase();

    // If store domain, serve store page
    if (host.startsWith('store.')) {
      const html = fs.readFileSync(path.join(__dirname, '..', 'pages', 'store.html'), 'utf-8');
      res.setHeader('Content-Type', 'text/html; charset=utf-8');
      res.setHeader('Cache-Control', 's-maxage=3600, stale-while-revalidate');
      return res.status(200).send(html);
    }

    const html = fs.readFileSync(path.join(__dirname, '..', 'pages', 'landing.html'), 'utf-8');
    res.setHeader('Content-Type', 'text/html; charset=utf-8');
    res.setHeader('Cache-Control', 's-maxage=3600, stale-while-revalidate');
    return res.status(200).send(html);
  } catch (err) {
    return res.status(500).send('Page not found: ' + err.message);
  }
};
