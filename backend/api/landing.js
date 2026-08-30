const fs = require('fs');
const path = require('path');

module.exports = async (req, res) => {
  try {
    const html = fs.readFileSync(path.join(__dirname, '..', 'public', 'index.html'), 'utf-8');
    res.setHeader('Content-Type', 'text/html; charset=utf-8');
    res.setHeader('Cache-Control', 's-maxage=3600, stale-while-revalidate');
    return res.status(200).send(html);
  } catch (err) {
    return res.status(500).send('Landing page not found');
  }
};
