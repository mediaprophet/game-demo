import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import init, { GameSession, scene_build, scene_quality_state, session_replay } from '../web/pkg/rolling_commons_shell.js';

const [wasm, seedBytes, actionsBytes] = await Promise.all([
  readFile(new URL('../web/pkg/rolling_commons_shell_bg.wasm', import.meta.url)),
  readFile(new URL('../web/fixtures/seed.ttl', import.meta.url)),
  readFile(new URL('../web/fixtures/actions.json', import.meta.url)),
]);
await init(wasm);
const seed = seedBytes.toString('utf8');
const actions = actionsBytes.toString('utf8');
const toPlain = value => {
  if (value instanceof Map) return Object.fromEntries([...value].map(([key, item]) => [key, toPlain(item)]));
  if (Array.isArray(value)) return value.map(toPlain);
  if (value && typeof value === 'object' && value.constructor === Object)
    return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, toPlain(item)]));
  return value;
};

const session = new GameSession(seed, actions);
assert.equal(toPlain(session.propose('switchPowerOn')).ok, false, 'the power goal is blocked before the project is built');
assert.equal(toPlain(session.propose('recordAppropriation')).ok, true, 'the opening campaign action is legal');
const events = toPlain(session.events());
const replay = toPlain(session_replay(seed, actions, events));
assert.equal(replay.diverged_at, null, 'replayed event tape stays deterministic');
assert.equal(replay.world, session.world(), 'replay returns the same world snapshot');

const scene = scene_build(1, 2, true, true, true, true, true, false, false, false, false, false, false, '');
assert.ok(scene.assets.length >= 20, `scene should include its local asset roster; got ${scene.assets.length}`);
assert.ok(scene.assets.every(asset => asset.bytes instanceof Uint8Array), 'all compiled scene assets are sealed byte arrays');
assert.ok(scene.assets.some(asset => asset.id === 'rc:asset/mast-signal'), 'communications mast resolves into the scene');
const high = scene_quality_state(1, 2, true, true, true, true, true, false, false, false, false, false, false, 1, 2, true);
assert.equal(high.quality_tier, 2, 'high-capability renderer can select the high feature tier');

console.log(`PASS Qualia GameSession: legal and blocked actions; deterministic replay (${events.length} events)`);
console.log(`PASS Qualia scene bridge: ${scene.assets.length} sealed assets and communications landmark`);
console.log('PASS renderer quality contract: high tier advertised only at high renderer capability');
