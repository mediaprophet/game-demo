// Export game-owned .10d assets from the exact Qualia WASM build used by the page.
// Run after scripts/build-game.ps1. The manifest is deterministic and lives with the game.
import { createHash } from 'node:crypto';
import { readFile, writeFile, mkdir, readdir, unlink } from 'node:fs/promises';
import { basename, dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import init, { pack_game_hmc, scene_build, verify_game_hmc } from '../web/pkg/rolling_commons_shell.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const wasmPath = join(root, 'web', 'pkg', 'rolling_commons_shell_bg.wasm');
const wasm = await readFile(wasmPath);
await init({ module_or_path: wasm });

const out = join(root, 'assets', 'generated', '10d');
await mkdir(out, { recursive: true });
const beacon = (await readFile(join(root, 'web', 'fixtures', 'community-beacon.vibe'), 'utf8')).trim();
const cases = [
  ['opening', [0, 0, false, false, false, false, false, false, false, false, false, false, false, '']],
  ['early-progress', [1, 1, false, true, true, true, false, false, false, false, false, false, false, '']],
  ['workshop-online', [1, 2, true, true, true, true, false, false, false, false, false, false, false, '']],
  ['signal-online', [1, 2, true, true, true, true, true, false, false, false, false, false, false, '']],
  ['high-tide', [1, 2, true, true, true, true, false, false, false, true, false, false, false, '']],
  ['braced-crossing', [1, 2, true, true, true, true, false, false, true, true, false, false, false, '']],
  ['bridge-open', [1, 2, true, true, true, true, false, true, true, true, false, false, false, '']],
  ['thriving', [2, 2, true, true, true, true, true, true, true, true, true, true, false, '']],
  ['harvested', [2, 2, true, true, true, true, true, true, true, true, true, true, true, '']],
  ['vibe-beacon', [0, 0, false, false, false, false, false, false, false, false, false, false, false, beacon]],
];

const assets = new Map();
const scenes = [];
for (const [name, args] of cases) {
  const scene = scene_build(...args);
  if (!Array.isArray(scene.organs) || scene.organs.length < 135) {
    throw new Error(`${name}: unexpected Qualia scene receipt`);
  }
  const entries = [];
  for (const organ of scene.organs) {
    if (!organ.id?.startsWith('rc:asset/') || !(organ.bytes instanceof Uint8Array)) {
      throw new Error(`${name}: invalid game asset receipt`);
    }
    const bytes = Buffer.from(organ.bytes);
    const sha256 = createHash('sha256').update(bytes).digest('hex');
    const slug = organ.id.slice('rc:asset/'.length).replace(/[^a-z0-9-]/gi, '-');
    const file = `${slug}--${sha256.slice(0, 12)}.10d`;
    const key = `${organ.id}:${sha256}`;
    if (!assets.has(key)) {
      await writeFile(join(out, file), bytes);
      assets.set(key, { id: organ.id, file: `10d/${file}`, bytes: bytes.length, sha256 });
    }
    entries.push({ id: organ.id, file: `10d/${file}`, rgba: [organ.r, organ.g, organ.b, organ.a] });
  }
  scenes.push({ name, assets: entries });
}

const manifest = {
  format: 'maslows-challenge-asset-export-v1',
  generator: 'scripts/export-game-assets.mjs',
  sourceRecipes: 'crates/rolling-commons-shell/src/asset_catalog.rs',
  wasm: { file: 'web/pkg/rolling_commons_shell_bg.wasm', sha256: createHash('sha256').update(wasm).digest('hex') },
  assets: [...assets.values()].sort((a, b) => a.file.localeCompare(b.file)),
  scenes,
};
const manifestText = JSON.stringify(manifest, null, 2) + '\n';
await writeFile(join(root, 'assets', 'generated', 'manifest.json'), manifestText);

const entries = await Promise.all(manifest.assets.map(async (asset) => ({
  key: asset.file,
  bytes: new Uint8Array(await readFile(join(root, 'assets', 'generated', asset.file))),
})));
const pack = pack_game_hmc(entries, manifestText);
if (verify_game_hmc(pack) !== manifest.assets.length + 1) {
  throw new Error('Qualia HMC reader rejected an entry');
}
const packDir = join(root, 'web', 'assets');
await mkdir(packDir, { recursive: true });
await writeFile(join(packDir, 'manifest.json'), manifestText);
const packPath = join(packDir, 'maslows-challenge-scenes.hmc');
await writeFile(packPath, Buffer.from(pack));
const buildId = manifest.wasm.sha256.slice(0, 16);
for (const page of ['game.html', 'spike.html']) {
  const pagePath = join(root, 'web', page);
  const html = await readFile(pagePath, 'utf8');
  const marker = /const WASM_BUILD = '[0-9a-f]{16}';/;
  if (!marker.test(html)) throw new Error(`${page}: missing WASM build marker`);
  await writeFile(pagePath, html.replace(marker, `const WASM_BUILD = '${buildId}';`));
}
// The directory contains generated exports only. Keep it aligned with the
// verified manifest so old variants cannot masquerade as current game assets.
const assetRoot = resolve(out);
const currentFiles = new Set(manifest.assets.map(asset => basename(asset.file)));
for (const entry of await readdir(assetRoot, { withFileTypes: true })) {
  if (!entry.isFile() || !entry.name.endsWith('.10d') || currentFiles.has(entry.name)) continue;
  const stalePath = resolve(assetRoot, entry.name);
  if (dirname(stalePath) !== assetRoot) throw new Error(`Asset outside export directory: ${stalePath}`);
  await unlink(stalePath);
}
console.log(`Exported ${manifest.assets.length} distinct .10d assets across ${scenes.length} scene states and Qualia HMC ${relative(root, packPath)}`);
