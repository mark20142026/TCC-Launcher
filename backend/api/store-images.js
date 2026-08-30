const fs = require('fs');
const path = require('path');

const MIME = {
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.gif': 'image/gif',
  '.svg': 'image/svg+xml',
  '.ico': 'image/x-icon',
  '.webp': 'image/webp',
};

module.exports = async (req, res) => {
  try {
    const url = new URL(req.url, `https://${req.headers.host}`);
    const imgPath = url.searchParams.get('path') || '';
    const filePath = path.join(__dirname, '..', 'public', 'store', 'images', imgPath);

    if (!fs.existsSync(filePath) || !filePath.includes('images')) {
      return res.status(404).send('Image not found');
    }

    const ext = path.extname(filePath).toLowerCase();
    const ct = MIME[ext] || 'application/octet-stream';
    const data = fs.readFileSync(filePath);

    res.setHeader('Content-Type', ct);
    res.setHeader('Cache-Control', 'public, max-age=86400, immutable');
    return res.status(200).send(data);
  } catch (err) {
    return res.status(500).send('Error: ' + err.message);
  }
};
