import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const webRoot = path.resolve(__dirname, '..', 'web');
const port = parseInt(process.env.PORT || '8089', 10);
const host = process.env.HOST || '127.0.0.1';

const mimeTypes = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'application/javascript; charset=utf-8',
  '.mjs': 'application/javascript; charset=utf-8',
  '.wasm': 'application/wasm',
  '.json': 'application/json; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.svg': 'image/svg+xml',
  '.ico': 'image/x-icon',
  '.10d': 'application/octet-stream',
  '.hmc': 'application/octet-stream',
  '.vibe': 'text/plain; charset=utf-8',
  '.ttl': 'text/turtle; charset=utf-8',
  '.txt': 'text/plain; charset=utf-8',
};

const server = http.createServer((req, res) => {
  const parsedUrl = new URL(req.url, `http://${req.headers.host || host}`);
  let pathname = decodeURIComponent(parsedUrl.pathname);
  if (pathname === '/' || pathname === '') {
    pathname = '/game.html';
  }

  const filePath = path.normalize(path.join(webRoot, pathname));
  if (!filePath.startsWith(webRoot)) {
    res.statusCode = 403;
    res.end('Access denied');
    return;
  }

  fs.stat(filePath, (err, stats) => {
    if (err || !stats.isFile()) {
      res.statusCode = 404;
      res.setHeader('Content-Type', 'text/plain; charset=utf-8');
      res.end(`Not found: ${pathname}`);
      return;
    }

    const ext = path.extname(filePath).toLowerCase();
    const contentType = mimeTypes[ext] || 'application/octet-stream';

    res.statusCode = 200;
    res.setHeader('Content-Type', contentType);
    res.setHeader('Content-Length', stats.size);
    res.setHeader('Cache-Control', 'no-cache');
    res.setHeader('Access-Control-Allow-Origin', '*');

    const stream = fs.createReadStream(filePath);
    stream.pipe(res);
  });
});

server.listen(port, host, () => {
  console.log(`Game demo local server running at http://${host}:${port}/game.html`);
});
