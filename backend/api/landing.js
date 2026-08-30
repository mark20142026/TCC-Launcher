const fs = require('fs');
const path = require('path');

const INSTALLER_URL = 'https://github.com/pnbx/TCC-Launcher/releases/download/v2.3.2/oneclient_app_2.3.2_x64-setup.exe';

module.exports = async (req, res) => {
  try {
    const host = (req.headers.host || req.headers['x-forwarded-host'] || '').toLowerCase();
    const url = req.url || '/';

    // download.theazizi.space/tcc/latest → redirect to installer
    if (host.startsWith('download.') && url.startsWith('/tcc/latest')) {
      res.writeHead(302, { Location: INSTALLER_URL });
      return res.end();
    }

    // download.theazizi.space → download page
    if (host.startsWith('download.')) {
      const html = fs.readFileSync(path.join(__dirname, '..', 'pages', 'download.html'), 'utf-8');
      res.setHeader('Content-Type', 'text/html; charset=utf-8');
      res.setHeader('Cache-Control', 's-maxage=3600, stale-while-revalidate');
      return res.status(200).send(html);
    }

    // store.theazizi.space → store page
    if (host.startsWith('store.')) {
      const html = fs.readFileSync(path.join(__dirname, '..', 'pages', 'store.html'), 'utf-8');
      res.setHeader('Content-Type', 'text/html; charset=utf-8');
      res.setHeader('Cache-Control', 's-maxage=3600, stale-while-revalidate');
      return res.status(200).send(html);
    }

    // Default → landing page
    const html = fs.readFileSync(path.join(__dirname, '..', 'pages', 'landing.html'), 'utf-8');
    res.setHeader('Content-Type', 'text/html; charset=utf-8');
    res.setHeader('Cache-Control', 's-maxage=3600, stale-while-revalidate');
    return res.status(200).send(html);
  } catch (err) {
    return res.status(500).send('Page not found: ' + err.message);
  }
};
