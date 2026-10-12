import { createHash } from 'node:crypto';
import { readFile, stat } from 'node:fs/promises';
import { resolve } from 'node:path';
import init, { validate_game_hmc } from '../web/pkg/rolling_commons_shell.js';

const root = resolve(import.meta.dirname, '..');
const read = path => readFile(resolve(root, path));
const wasmPath = 'web/pkg/rolling_commons_shell_bg.wasm';
const wasm = await read(wasmPath);
const wasmSha = createHash('sha256').update(wasm).digest('hex');
const wasmModule = await WebAssembly.compile(wasm);
await init({module_or_path:wasm});
if (!WebAssembly.Module.exports(wasmModule).some(item => item.name === 'memory'))
  throw new Error('Release WASM does not export memory');

const styles = [
  ['web/assets/manifest.json', 'web/assets/maslows-challenge-scenes.hmc'],
  ['web/assets/earthlight-manifest.json', 'web/assets/maslows-challenge-earthlight.hmc'],
  ['web/assets/community-manifest.json', 'web/assets/maslows-challenge-community.hmc'],
];
for (const [manifestPath, packPath] of styles) {
  const manifest = JSON.parse(await read(manifestPath));
  const actualDigest = manifest.wasm?.sha256;
  if (actualDigest !== wasmSha)
    throw new Error(`${manifestPath} WASM digest ${actualDigest} != ${wasmSha}`);
  if (!Array.isArray(manifest.scenes) || manifest.scenes.length !== 40)
    throw new Error(`${manifestPath} should contain all 40 authored scene states`);
  if (!Array.isArray(manifest.assets) || manifest.assets.length<135)
    throw new Error(`${manifestPath} is missing the authored scene catalog`);
  for(const scene of manifest.scenes){
    if(!Array.isArray(scene.assetIndices)||!scene.assetIndices.length||
      scene.assetIndices.some(index=>!Number.isInteger(index)||!manifest.assets[index])||
      new Set(scene.assetIndices).size!==scene.assetIndices.length)
      throw new Error(`${manifestPath}: invalid scene references for ${scene.name}`);
  }
  if(!Array.isArray(manifest.textures)||!manifest.textures.length||
    manifest.textures.some(texture=>texture.file!==`textures/sha256/${texture.sha256}`||
      texture.mime!=='image/png'||![0,1,2,3].includes(texture.role)))
    throw new Error(`${manifestPath}: missing or invalid surface dependencies`);
  const packInfo = await stat(resolve(root, packPath));
  if (packInfo.size < 100_000) throw new Error(`${packPath} is missing or unexpectedly small`);
  const pack=await read(packPath),textureCount=new Set(manifest.textures.map(texture=>texture.file)).size;
  if(validate_game_hmc(pack)!==manifest.assets.length+textureCount+1)
    throw new Error(`${packPath}: native material/UV/texture dependency verification failed`);
}

await stat(resolve(root, 'web/assets/ecology/profiles/se-corridor-reference.json'));
const html = await read('web/game.html').then(bytes => bytes.toString('utf8'));
for (const marker of [
  `const WASM_BUILD = '${wasmSha.slice(0, 16)}-community-overhaul';`,
  'id="atlas-overlay"',
  'data-game-view="trip"',
  'id="drive-dashboard"',
  'id="quality-profile"',
  'id="art-style"',
  'id="route-feedback"',
]) {
  if (!html.includes(marker)) throw new Error(`Missing game entry point: ${marker}`);
}

console.log(`PASS release WASM: ${wasm.byteLength.toLocaleString()} bytes, SHA-256 ${wasmSha}`);
console.log('PASS Storybook, Earthlight, Community Grounds: 40 scene states, native asset and texture dependency verification, matching WASM digest');
console.log('PASS Australia trip map, art-style selector, renderer-quality selector, Direct Drive dashboard');
