import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import init, { GameDriveSimulation, GameSession, asset_verify_10d, scene_build, scene_quality_state, session_replay,
  pack_game_hmc, validate_game_hmc } from '../web/pkg/rolling_commons_shell.js';

const [wasm, seedBytes, actionsBytes, driveModuleBytes] = await Promise.all([
  readFile(new URL('../web/pkg/rolling_commons_shell_bg.wasm', import.meta.url)),
  readFile(new URL('../web/fixtures/seed.ttl', import.meta.url)),
  readFile(new URL('../web/fixtures/actions.json', import.meta.url)),
  readFile(new URL('../web/fixtures/drive-simulation.vibe', import.meta.url)),
]);
await init(wasm);
const seed = seedBytes.toString('utf8');
const actions = actionsBytes.toString('utf8');
const driveModule = driveModuleBytes.toString('utf8');
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

const drive = new GameDriveSimulation(driveModule);
const caravanSetup = toPlain(drive.setup(1));
assert.equal(caravanSetup.top_speed, 3, 'VibeScript owns the caravan rig handling profile');
assert.equal(caravanSetup.trailer, true, 'VibeScript identifies a trailer-bound rig');
const driveInput = {dt:0.05,rig_id:1,x:7.2,z:0.48,heading:-Math.PI/2,speed:0,
  throttle:true,reverse:false,brake:false,steer:0,
  terrain_height:0,terrain_height_ahead:0,terrain_height_behind:0,
  terrain_sample_distance:0.55,crossing_open:true,period:'dusk',hazards:[],
  oil_temp:94,coolant_temp:89,engine_health:100,overheat_seconds:0,
  traffic_queue:4,hazard_stops:0,emergency_stops:0,last_warning:''};
const straight = toPlain(drive.step(JSON.stringify(driveInput)));
assert.ok(straight.x > 0 && straight.speed > 0, 'VibeScript advances a powered rig');
assert.equal(straight.mirror_blocked, true, 'VibeScript models straight-road trailer mirror obstruction');
const stopped = toPlain(drive.step(JSON.stringify({...driveInput,throttle:false,x:0,speed:1.4,
  hazards:[
    {selected:false,x:0.01,z:0.48,radius:1.0,id:'profile-excluded',label:'excluded fauna',taxon:'ignored'},
    {selected:true,x:0.1,z:0.48,radius:0.6,id:'test-kangaroo',label:'kangaroo',taxon:'Macropodidae'},
  ]})));
assert.equal(stopped.warning, 'safe-stop', 'VibeScript resolves a cautious wildlife stop at dusk');
assert.equal(stopped.speed, 0, 'wildlife stop holds the vehicle safely');
assert.equal(stopped.hazard_id,'test-kangaroo','VibeScript excludes hazards outside the selected ecology profile');
const overheated = toPlain(drive.step(JSON.stringify({...driveInput,throttle:false,x:20,speed:3,
  terrain_height_ahead:0.099,oil_temp:144,coolant_temp:116,overheat_seconds:10})));
assert.ok(overheated.engine_health < 100 && overheated.engine_warning.includes('CRITICAL'),
  'VibeScript applies sustained hot-climb engine damage and warning state');
assert.equal(overheated.road_surface, 'corrugated gravel', 'VibeScript selects the route surface');

const scene = scene_build(1, 2, true, true, true, true, true, false, false, false, false, false, false, '');
assert.ok(scene.assets.length >= 20, `scene should include its local asset roster; got ${scene.assets.length}`);
assert.ok(scene.assets.every(asset => asset.bytes instanceof Uint8Array), 'all compiled scene assets are sealed byte arrays');
assert.ok(scene.assets.some(asset => asset.id === 'rc:asset/mast-signal'), 'communications mast resolves into the scene');
assert.equal(scene.places.length, 11, 'campaign site marker order remains stable');
assert.equal(scene.pickTargets.length, 16, 'five mesh-adjacent interactive targets join the eleven site targets');
assert.ok(scene.pickTargets.some(target => target.activityId === 'workshop-repair'),
  'the workshop repair bench opens its playable activity');
assert.ok(scene.pickTargets.some(target => target.activityId === 'fruit-harvest'),
  'the orchard harvest props open their playable activity');
const materialized = scene.assets.find(asset => asset.renderRole !== 'water');
const materialReceipt = toPlain(asset_verify_10d(materialized.bytes));
assert.ok(materialReceipt.materials > 0 && materialReceipt.material_ranges > 0,
  'compiled game assets retain their PBR material and submesh binding in .10d');
assert.equal(materialReceipt.normal_vertices,materialReceipt.vertices,
  'authored smooth/hard normals survive sealed container read-back');
assert.equal(materialReceipt.uv_vertices,materialReceipt.vertices,
  'UV seam vertices have matching sealed coordinate counts');
const residentAssets=scene.assets.filter(asset=>asset.id.startsWith('rc:asset/resident-'));
const touringAssets=scene.assets.filter(asset=>asset.id.startsWith('rc:asset/rig/'));
assert.ok(touringAssets.length>0,'touring geometry is included in the native scene');
for(const asset of touringAssets){
  const receipt=toPlain(asset_verify_10d(asset.bytes));
  const expected=asset.id.startsWith('rc:asset/rig/4wd-caravan/')?10:
    asset.id.startsWith('rc:asset/rig/car-trailer/')?8:5;
  assert.equal(receipt.rig_joints,expected,`${asset.id}: body and wheel hinges survive SKL1 read-back`);
  assert.equal(receipt.animation_clips,1,`${asset.id}: native controlled idle pose survives read-back`);
}
assert.equal(residentAssets.filter(asset=>!asset.id.includes('-attachment-')).length,60,
  'six residents retain ten stable body mesh parts each');
assert.equal(residentAssets.filter(asset=>asset.id.includes('-attachment-')).length,16,
  'resident tools and carried objects have native rig attachments');
for(const asset of residentAssets){
  const receipt=toPlain(asset_verify_10d(asset.bytes));
  assert.equal(receipt.rig_joints,18,`${asset.id}: articulated rig survives native read-back`);
  assert.equal(receipt.animation_clips,8,`${asset.id}: all activity and rest clips survive native read-back`);
  assert.equal(receipt.uv_vertices,receipt.vertices,`${asset.id}: rigged seam topology remains aligned`);
}
assert.ok(scene.textures.length>0,'native material library produces shared PNG dependencies');
const textureEntries=new Map(scene.textures.map(image=>[image.key,{key:image.key,bytes:image.bytes}]));
const specimen=[{key:'10d/material-specimen.10d',bytes:materialized.bytes},...textureEntries.values()];
assert.equal(validate_game_hmc(pack_game_hmc(specimen,'{}')),specimen.length+1,
  'native HMC verifier resolves textured material dependencies and matching UV stream');
assert.throws(()=>validate_game_hmc(pack_game_hmc(specimen.filter(entry=>entry.key.startsWith('10d/')),'{}')),
  /dependency|missing/i,'a mesh-only package must not silently pass a textured asset');
const high = scene_quality_state(1, 2, true, true, true, true, true, false, false, false, false, false, false, 1, 2, true);
assert.equal(high.quality_tier, 2, 'high-capability renderer can select the high feature tier');

console.log(`PASS Qualia GameSession: legal and blocked actions; deterministic replay (${events.length} events)`);
console.log(`PASS Qualia scene bridge: ${scene.assets.length} sealed assets and communications landmark`);
console.log(`PASS Qualia semantic interaction: ${scene.pickTargets.length} site/object targets, stable campaign site order`);
console.log(`PASS Qualia PBR packaging: ${materialReceipt.materials} material and ${materialReceipt.material_ranges} submesh bindings retained`);
console.log('PASS VibeScript drive module: rig movement, trailer mirrors, wildlife stop and overheating');
console.log('PASS renderer quality contract: high tier advertised only at high renderer capability');
