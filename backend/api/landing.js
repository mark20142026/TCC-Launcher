const fs = require('fs');
const path = require('path');

module.exports = async (req, res) => {
  try {
    const host = req.headers.host || '';
    const isStore = host.startsWith('store.');
    const filePath = isStore
      ? path.join(__dirname, '..', 'public', 'store', 'index.html')
      : path.join(__dirname, '..', 'public', 'index.html');
    const html = fs.readFileSync(filePath, 'utf-8');
    res.setHeader('Content-Type', 'text/html; charset=utf-8');
    res.setHeader('Cache-Control', 's-maxage=3600, stale-while-revalidate');
    return res.status(200).send(html);
  } catch (err) {
    return res.status(500).send('Page not found: ' + err.message);
  }
};
