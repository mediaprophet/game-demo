import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const webRoot = path.resolve(__dirname, '..', 'web');
const generatedRoot = path.resolve(__dirname, '..', 'assets', 'generated');
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

const realpathSyncSafe = candidate => {
  try {
    return fs.realpathSync(candidate);
  } catch {
    return null;
  }
};

const webRootReal = realpathSyncSafe(webRoot);
const ecopunkModelsRoot = realpathSyncSafe(path.resolve(__dirname, '..', '..', 'ecopunk', 'assets', 'models', 'corymbia-aparrerinja'));
const assetRoots = {
  storybook: realpathSyncSafe(generatedRoot),
  earthlight: realpathSyncSafe(path.join(generatedRoot, 'earthlight')),
  community: realpathSyncSafe(path.join(generatedRoot, 'community')),
};

const assetFileRoute = /^\/asset-files\/(storybook|earthlight|community)\/(10d\/[A-Za-z0-9_-]+\.10d|impo\/[A-Za-z0-9_-]+\.impo|textures\/sha256\/[a-f0-9]{64})$/;
const ecopunkModelRoute = /^\/ecopunk-models\/(model-set\.json|graph\/ecology(?:-graph)?\.(?:json|q42)|10d\/[a-z0-9-]+\.10d|corymbia-aparrerinja\.hmc)$/;

const insideRoot = (root, filePath) => {
  const normalize = value => (process.platform === 'win32' ? value.toLowerCase() : value);
  const relative = path.relative(normalize(root), normalize(filePath));
  return relative === '' || (relative !== '..' && !relative.startsWith('..' + path.sep) && !path.isAbsolute(relative));
};

const deny = (res, status, message) => {
  res.statusCode = status;
  res.setHeader('Content-Type', 'text/plain; charset=utf-8');
  res.end(message);
};

const sendFile = (req, res, filePath, stats, contentType) => {
  res.statusCode = 200;
  res.setHeader('Content-Type', contentType);
  res.setHeader('Content-Length', stats.size);
  res.setHeader('Cache-Control', 'no-cache');
  res.setHeader('Access-Control-Allow-Origin', '*');
  if (req.method === 'HEAD') {
    res.end();
    return;
  }
  fs.createReadStream(filePath).pipe(res);
};

const serveContained = (req, res, rootReal, relPath, contentTypeFor) => {
  if (!rootReal) {
    deny(res, 404, 'Not found');
    return;
  }
  const candidate = path.resolve(rootReal, relPath);
  if (!insideRoot(rootReal, candidate)) {
    deny(res, 403, 'Access denied');
    return;
  }
  fs.realpath(candidate, (realErr, realPath) => {
    if (realErr) {
      deny(res, 404, 'Not found');
      return;
    }
    if (!insideRoot(rootReal, realPath)) {
      deny(res, 403, 'Access denied');
      return;
    }
    fs.stat(realPath, (statErr, stats) => {
      if (statErr || !stats.isFile()) {
        deny(res, 404, 'Not found');
        return;
      }
      sendFile(req, res, realPath, stats, contentTypeFor(realPath));
    });
  });
};

const server = http.createServer((req, res) => {
  if (req.method !== 'GET' && req.method !== 'HEAD') {
    res.statusCode = 405;
    res.setHeader('Allow', 'GET, HEAD');
    res.setHeader('Content-Type', 'text/plain; charset=utf-8');
    res.end('Method not allowed');
    return;
  }

  let pathname;
  try {
    const parsedUrl = new URL(req.url, `http://${req.headers.host || host}`);
    pathname = decodeURIComponent(parsedUrl.pathname);
  } catch {
    deny(res, 400, 'Bad request');
    return;
  }

  const assetMatch = assetFileRoute.exec(pathname);
  if (assetMatch) {
    const [, style, relPath] = assetMatch;
    const contentType = relPath.startsWith('textures/')
      ? () => 'image/png'
      : () => 'application/octet-stream';
    serveContained(req, res, assetRoots[style], relPath, contentType);
    return;
  }

  const ecopunkMatch = ecopunkModelRoute.exec(pathname);
  if (ecopunkMatch) {
    serveContained(req, res, ecopunkModelsRoot, ecopunkMatch[1], filePath =>
      mimeTypes[path.extname(filePath).toLowerCase()] || 'application/octet-stream');
    return;
  }

  if (pathname === '/' || pathname === '') {
    pathname = '/game.html';
  }

  const segments = pathname.split('/').filter(Boolean);
  if (segments.some(segment => segment.startsWith('.'))) {
    deny(res, 403, 'Access denied');
    return;
  }

  serveContained(req, res, webRootReal, segments.join('/'), filePath => {
    const ext = path.extname(filePath).toLowerCase();
    return mimeTypes[ext] || 'application/octet-stream';
  });
});

server.listen(port, host, () => {
  console.log(`Game demo local server running at http://${host}:${port}/game.html`);
});
