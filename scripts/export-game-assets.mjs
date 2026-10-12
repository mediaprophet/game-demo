// Export game-owned .10d assets from the exact Qualia WASM build used by the page.
// Run after scripts/build-game.ps1. The manifest is deterministic and lives with the game.
import { createHash } from 'node:crypto';
import { readFile, writeFile, mkdir, readdir, unlink } from 'node:fs/promises';
import { basename, dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import init, { pack_game_hmc, scene_build, set_art_style, set_vehicle_rig, set_fauna_selection, set_flora_selection,
  taxon_binding_json, taxon_dwc_json, validate_game_hmc } from '../web/pkg/rolling_commons_shell.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const wasmPath = join(root, 'web', 'pkg', 'rolling_commons_shell_bg.wasm');
const wasm = await readFile(wasmPath);
await init({ module_or_path: wasm });
// The checked-in first profile is a broad illustrative corridor scenario.
// Place-profile keys select known render recipes; unknown taxa remain data-only.
const ecologyProfile = JSON.parse(await readFile(join(root, 'assets', 'ecology', 'profiles',
  'se-corridor-reference.json'), 'utf8'));
const publicProfiles = join(root, 'web', 'assets', 'ecology', 'profiles');
await mkdir(publicProfiles, { recursive: true });
await writeFile(join(publicProfiles, 'se-corridor-reference.json'),
  JSON.stringify(ecologyProfile, null, 2) + '\n');
set_fauna_selection(JSON.stringify(ecologyProfile.render.faunaSelection));
set_flora_selection(JSON.stringify(ecologyProfile.render.floraSelection || []));

// Taxon bindings come from the VibeScript fixture (web/fixtures/
// taxon-bindings.vibe) through the WASM host — the single authored source.
// GBIF backbone keys are the engine-interop identity; name strings are
// display labels. Fauna asset instance ids embed the renderKey token
// (inland-goat-01, road-kangaroo-01); flora assets prefix-match
// `rc:asset/flora/<renderKey>` via assetFamily.
const taxonBindings = {};
for (const key of [...ecologyProfile.render.faunaSelection, ...(ecologyProfile.render.floraSelection || [])]) {
  const binding = JSON.parse(taxon_binding_json(key));
  if (binding) taxonBindings[key] = binding;
}
function taxonForAsset(assetId) {
  if (assetId.startsWith('rc:asset/flora/')) {
    const entry = Object.entries(taxonBindings).find(([, b]) => assetId.startsWith(`${b.assetFamily}/`));
    return entry ? { renderKey: entry[0], ...entry[1] } : null;
  }
  if (assetId.startsWith('rc:asset/fauna/')) {
    const entry = Object.entries(taxonBindings).find(([k]) => assetId.includes(k));
    return entry ? { renderKey: entry[0], ...entry[1] } : null;
  }
  return null;
}

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

for (const [style, styleSlug] of [[0, 'storybook'], [1, 'earthlight'], [2, 'community']]) {
  set_art_style(style);
  const styleRoot = style === 0 ? join(root, 'assets', 'generated') :
    join(root, 'assets', 'generated', styleSlug);
  const out = join(styleRoot, '10d');
  await mkdir(out, { recursive: true });
  const assets = new Map();
  const textures = new Map();
  const scenes = [];
  // Semantic statements for the pack's `sem/` section (G50): deduped across
  // assets — taxon-node quins repeat per asset, depicts/accrual quins differ.
  const stmtMap = new Map();
  for (const [name, args] of cases) {
    for (const rig of [0, 1, 2, 3]) {
      set_vehicle_rig(rig);
      set_fauna_selection(JSON.stringify(ecologyProfile.render.faunaSelection));
      set_flora_selection(JSON.stringify(ecologyProfile.render.floraSelection || []));
  const scene = scene_build(...args);
  for(const image of scene.textures||[]){
    const bytes=Buffer.from(image.bytes),sha256=createHash('sha256').update(bytes).digest('hex');
    if(image.key!==`textures/sha256/${sha256}`||image.mime!=='image/png'||![0,1,2,3].includes(image.role))
      throw new Error('Invalid native texture dependency receipt');
    const key=`${image.key}:${image.role}`;
    if(!textures.has(key)){
      await mkdir(join(styleRoot,'textures','sha256'),{recursive:true});
      await writeFile(join(styleRoot,image.key),bytes);
      textures.set(key,{file:image.key,sha256,mime:image.mime,role:image.role,bytes:bytes.length});
    }
  }
      const sceneName = `${name}-rig-${rig}-ecology-${ecologyProfile.id}`;
  if (!Array.isArray(scene.assets) || scene.assets.length < 135) {
    throw new Error(`${sceneName}: unexpected Qualia scene receipt`);
  }
  const entries = [];
  for (const asset of scene.assets) {
    if (!asset.id?.startsWith('rc:asset/') || !(asset.bytes instanceof Uint8Array) ||
        typeof asset.materialFamily !== 'string' || !asset.materialFamily) {
      throw new Error(`${sceneName}: invalid game asset receipt`);
    }
    const bytes = Buffer.from(asset.bytes);
    const sha256 = createHash('sha256').update(bytes).digest('hex');
    const slug = asset.id.slice('rc:asset/'.length).replace(/[^a-z0-9-]/gi, '-');
    const file = `${slug}--${sha256.slice(0, 12)}.10d`;
    const key = `${asset.id}:${sha256}`;
    if (!assets.has(key)) {
      await writeFile(join(out, file), bytes);
      const taxon = taxonForAsset(asset.id);
      // dwc/accrual statements are authored in taxon-bindings.vibe and sealed
      // by the host (quin.statement); fields are hex strings — u64s are not
      // f64-lossless. Each entry carries its source terms so the pack lexicon
      // needs no second mapping.
      for (const st of taxon ? JSON.parse(taxon_dwc_json(taxon.renderKey, asset.id)) : []) {
        const stmtId = Object.values(st.quin).join('|');
        if (!stmtMap.has(stmtId)) stmtMap.set(stmtId, st);
      }
      assets.set(key, { id: asset.id, file: `10d/${file}`, bytes: bytes.length, sha256,
        rgba: [asset.r, asset.g, asset.b, asset.a], renderRole: asset.renderRole || 'opaque',
        materialFamily: asset.materialFamily,
        ...(taxon ? { taxon: { gbifKey: taxon.gbifKey, iri: taxon.iri, rank: taxon.rank } } : {}) });
    }
    entries.push(key);
  }
  scenes.push({ name: sceneName, vehicleRig: rig, ecologyProfile: ecologyProfile.id, assetKeys: entries });
    }
  }

  const globalAssets = [...assets.entries()].sort((a, b) => a[1].file.localeCompare(b[1].file));
  const assetIndex = new Map(globalAssets.map(([key], index) => [key, index]));

  // Pack semantic section: 48-byte NQuins (little-endian u64 fields in
  // struct order) + a `<hex-hash> <iri>` lexicon so the hashes resolve
  // in-browser. Integer-typed objects carry no hash — skipped in the lexicon.
  const stmtList = [...stmtMap.values()];
  const quinBytes = Buffer.alloc(stmtList.length * 48);
  stmtList.forEach((st, i) => {
    for (const [j, field] of ['subject','predicate','object','context','metadata','parity'].entries())
      quinBytes.writeBigUInt64LE(BigInt(st.quin[field]), i * 48 + j * 8);
  });
  const lexTerms = new Map();
  for (const st of stmtList) {
    for (const [hash, term] of [[st.quin.subject, st.s], [st.quin.predicate, st.p],
        ...(st.integer ? [] : [[st.quin.object, st.o]])]) {
      if (lexTerms.has(hash) && lexTerms.get(hash) !== term)
        throw new Error(`lexicon hash collision: ${hash}`);
      lexTerms.set(hash, term);
    }
  }
  const lexText = [...lexTerms.entries()]
    .map(([hash, term]) => `${hash.slice(2)} ${term}`).sort().join('\n') + '\n';
  const semanticFiles = stmtList.length ? [
    { key: 'sem/taxonomy.quins', bytes: quinBytes },
    { key: 'sem/taxonomy.lex', bytes: Buffer.from(lexText, 'utf8') },
  ] : [];
  const semantics = semanticFiles.map(f => ({ key: f.key,
    quins: f.key.endsWith('.quins') ? stmtList.length : undefined,
    bytes: f.bytes.length, sha256: createHash('sha256').update(f.bytes).digest('hex') }));
  const manifest = {
  format: 'maslows-challenge-asset-export-v1',
  artStyle: styleSlug,
  epistemicMode: styleSlug === 'community' ? 'civics' : 'simulation',
  generator: 'scripts/export-game-assets.mjs',
  sourceRecipes: ['crates/rolling-commons-shell/src/asset_catalog.rs',
    'crates/rolling-commons-shell/src/vehicles.rs',
    'crates/rolling-commons-shell/src/fauna.rs',
    'crates/rolling-commons-shell/src/flora.rs',
    'crates/rolling-commons-shell/src/material_library.rs',
    'crates/rolling-commons-shell/src/resident_animation.rs',
    'crates/rolling-commons-shell/src/resident_props.rs',
    'crates/rolling-commons-shell/src/animation_bridge.rs',
    'crates/rolling-commons-shell/src/activity_presentation.rs',
    'web/fixtures/resident-activities.vibe',
    'crates/rolling-commons-shell/src/rig_workshop.rs',
    'crates/rolling-commons-shell/src/drive_session.rs',
    'crates/rolling-commons-shell/src/drive_contacts.rs',
    'crates/rolling-commons-shell/src/drive_contact_integration.rs',
    'crates/rolling-commons-shell/src/vehicle_camera.rs',
    'crates/rolling-commons-shell/src/macropod_models.rs',
    'crates/rolling-commons-shell/src/fauna_animation.rs',
    'crates/rolling-commons-shell/src/quadruped_models.rs',
    'crates/rolling-commons-shell/src/quadruped_animation.rs',
    'crates/rolling-commons-shell/src/camera_presentation.rs',
    'crates/rolling-commons-shell/src/workshop_assembly.rs',
    'crates/rolling-commons-shell/src/asset_compile.rs',
    'crates/rolling-commons-shell/src/vehicle_cabins.rs',
    'crates/rolling-commons-shell/src/vehicle_exterior.rs',
    'crates/rolling-commons-shell/src/vehicle_animation.rs',
    'crates/rolling-commons-shell/src/water_presentation.rs',
    'web/fixtures/rig-workshop.vibe',
    'crates/rolling-commons-shell/src/workshop_structure.rs',
    'crates/rolling-commons-shell/src/workshop.rs',
    'crates/rolling-commons-shell/src/camp_home.rs',
    'crates/rolling-commons-shell/src/mature_gums.rs',
    'assets/ecology/profiles/se-corridor-reference.json'],
  wasm: { file: 'web/pkg/rolling_commons_shell_bg.wasm', sha256: createHash('sha256').update(wasm).digest('hex') },
  taxonConvention: ecologyProfile.taxonConvention || null,
  taxonBindings,
  semantics,
  assets: globalAssets.map(([, asset]) => asset),
  textures: [...textures.values()].sort((a,b)=>a.file.localeCompare(b.file)||a.role-b.role),
  scenes: scenes.map(({ assetKeys, ...scene }) => ({
    ...scene,
    assetIndices: assetKeys.map(key => assetIndex.get(key)),
  })),
  };
  const manifestText = JSON.stringify(manifest, null, 2) + '\n';
  await writeFile(join(styleRoot, 'manifest.json'), manifestText);

  const entries = await Promise.all(manifest.assets.map(async (asset) => ({
    key: asset.file,
    bytes: new Uint8Array(await readFile(join(styleRoot, asset.file))),
  })));
  const textureFiles=[...new Set(manifest.textures.map(texture=>texture.file))];
  for(const file of textureFiles)entries.push({key:file,
    bytes:new Uint8Array(await readFile(join(styleRoot,file)))});
  for(const sem of semanticFiles)entries.push({key:sem.key,bytes:new Uint8Array(sem.bytes)});
// Keep the checked-in manifest readable; pack the same data compactly so
// growing scene catalogs stay within the game shell's bounded HMC input.
  const pack = pack_game_hmc(entries, JSON.stringify(manifest));
  if (validate_game_hmc(pack) !== manifest.assets.length + textureFiles.length
      + semantics.length + 1) {
    throw new Error('Qualia HMC reader rejected an entry');
  }
  const packDir = join(root, 'web', 'assets');
  await mkdir(packDir, { recursive: true });
  await writeFile(join(packDir, style === 0 ? 'manifest.json' : `${styleSlug}-manifest.json`), manifestText);
  const packPath = join(packDir, style === 0 ? 'maslows-challenge-scenes.hmc' :
    `maslows-challenge-${styleSlug}.hmc`);
  await writeFile(packPath, Buffer.from(pack));
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
  console.log(`Exported ${styleSlug}: ${manifest.assets.length} distinct .10d assets across ${scenes.length} scene states and Qualia HMC ${relative(root, packPath)}`);
}
const buildId = createHash('sha256').update(wasm).digest('hex').slice(0, 16);
for (const page of ['game.html', 'spike.html']) {
  const pagePath = join(root, 'web', page);
  const html = await readFile(pagePath, 'utf8');
  // The marker may carry a cache-bust suffix (for example the character
  // roster module); refresh the build hash but keep any suffix intact.
  const marker = /const WASM_BUILD = '([0-9a-f]{16})([^']*)';/;
  const match = marker.exec(html);
  if (!match) throw new Error(`${page}: missing WASM build marker`);
  await writeFile(pagePath,
    html.replace(marker, `const WASM_BUILD = '${buildId}${match[2]}';`));
}

// Keep the authoring/review register tied to the actual three shipped style
// manifests instead of leaving a hand-maintained inventory at an old count.
const registerStyles = [
  ['Storybook', 'manifest.json'],
  ['Earthlight', 'earthlight-manifest.json'],
  ['Community Grounds', 'community-manifest.json'],
];
const register = new Map();
const isOneOf = (value, terms) => terms.some(term => value.includes(term));
function assetFamily(id) {
  const key = id.toLowerCase();
  if (key.startsWith('rc:asset/fauna/')) return 'Australian fauna';
  if (key.startsWith('rc:asset/flora/')) return 'Australian flora';
  if (key.startsWith('rc:asset/rig/')) return 'Touring rigs';
  if (isOneOf(key, ['gum-', 'wattle-', 'tree-', 'grass', 'herb-', 'flower-', 'lavender',
    'orchard-', 'crop-', 'reed', 'shrub', 'foliage', 'canopy-'])) return 'Australian flora';
  if (isOneOf(key, ['terrain-', 'meadow', 'ridge', 'hill-', 'footpath', 'road-', 'bridge',
    'canal-', 'creek', 'water-', 'ripple', 'quay'])) return 'Terrain and water';
  if (isOneOf(key, ['workshop', 'hall-', 'barn-', 'market-stall', 'shed-', 'shelter']))
    return 'Buildings';
  if (isOneOf(key, ['solar-', 'battery', 'mast-', 'wind-pump', 'tank-', 'purification']))
    return 'Infrastructure';
  return 'Props and furnishing';
}
function assetSite(id) {
  const key = id.toLowerCase();
  if (key.startsWith('rc:asset/fauna/')) return 'Australian corridor drive fixture';
  if (key.startsWith('rc:asset/flora/')) return 'Australian native planting showcase';
  if (key.startsWith('rc:asset/rig/')) return 'Touring vehicle system';
  if (key.includes('saltwind') || isOneOf(key, ['barn-', 'orchard-', 'market-stall',
    'wind-pump', 'trade-boat', 'quay'])) return 'Saltwind Reach';
  if (isOneOf(key, ['highland', 'ridge-', 'hill-'])) return 'Northern Highlands';
  if (key.includes('canal') || key.includes('bridge')) return 'Canal crossing';
  if (key.includes('workshop')) return 'Kestrel Flats workshop';
  if (key.includes('camp-')) return 'Kestrel Flats mobile camp';
  if (isOneOf(key, ['garden-', 'herb-', 'flower-', 'lavender', 'veggie-', 'hydroponic']))
    return 'Kestrel Flats garden';
  return 'Kestrel Flats commons';
}
function sourceRecipe(id) {
  const key = id.toLowerCase();
  if (key.startsWith('rc:asset/fauna/')) return 'crates/rolling-commons-shell/src/fauna.rs';
  if (key.startsWith('rc:asset/flora/')) return 'crates/rolling-commons-shell/src/flora.rs';
  if (key.startsWith('rc:asset/rig/')) return 'crates/rolling-commons-shell/src/vehicles.rs';
  if (isOneOf(key, ['terrain-', 'meadow', 'ridge', 'hill-']))
    return 'crates/rolling-commons-shell/src/asset_catalog.rs; terrain.rs';
  return 'crates/rolling-commons-shell/src/asset_catalog.rs';
}
function finishTask(family) {
  if (family === 'Australian fauna') return 'Review species form, scale and place-profile selection against Australian source and locality evidence.';
  if (family === 'Australian flora') return 'Review Australian plant silhouette, seasonal state, placement density and ground clearance.';
  if (family === 'Touring rigs') return 'Review recognisable vehicle family, towing proportions, glazing, mirrors and tyre contact.';
  if (family === 'Terrain and water') return 'Review continuous ground, banks, paths, water edges and terrain seam at gameplay distance.';
  if (family === 'Buildings') return 'Review reference composition, structural detail, usable depth and campaign state variants.';
  if (family === 'Infrastructure') return 'Review physical connections, material response and state-dependent operation cues.';
  return 'Review activity placement, material definition, scale, clearance and interaction readability.';
}
function csvCell(value) {
  return `"${String(value).replaceAll('"', '""')}"`;
}

for (const [style, manifestName] of registerStyles) {
  const manifest = JSON.parse(await readFile(join(root, 'web', 'assets', manifestName), 'utf8'));
  const styleFiles = new Map();
  manifest.assets.forEach((asset, index) => {
    const item = register.get(asset.id) ?? {
      id: asset.id,
      family: assetFamily(asset.id),
      site: assetSite(asset.id),
      source: sourceRecipe(asset.id),
      roles: new Set(),
      materials: new Set(),
      states: new Set(),
      files: new Map(),
    };
    item.roles.add(asset.renderRole || 'unspecified');
    item.materials.add(asset.materialFamily || 'unspecified');
    const variants = item.files.get(style) ?? [];
    variants.push(`${asset.file} [sha256 ${asset.sha256}; ${asset.bytes} bytes]`);
    item.files.set(style, variants);
    register.set(asset.id, item);
    styleFiles.set(index, asset.id);
  });
  for (const scene of manifest.scenes) {
    for (const index of new Set(scene.assetIndices || [])) {
      const id = styleFiles.get(index);
      if (id) register.get(id).states.add(scene.name);
    }
  }
}

const registerHeader = [
  'Stable asset ID', 'Parent semantic site', 'Family', 'Source recipe', 'Scene states',
  'Material families', 'Render roles', 'LOD policy', 'Current disposition', 'Replacement/finish task',
  'Capture', 'Acceptance', 'Storybook files', 'Earthlight files', 'Community Grounds files',
];
const registerRows = [...register.values()].sort((a, b) => a.id.localeCompare(b.id)).map(item => [
  item.id,
  item.site,
  item.family,
  item.source,
  [...item.states].sort().join('; '),
  [...item.materials].sort().join('; '),
  [...item.roles].sort().join('; '),
  'Single exported mesh; distance LOD chain pending',
  'Visual review pending; not accepted against the reference',
  finishTask(item.family),
  'Pending named-view capture',
  'Pending human review against docs/planning/35 §8 rubric',
  ...registerStyles.map(([style]) => (item.files.get(style) || []).sort().join('; ')),
]);
await writeFile(join(root, 'docs', 'art', 'asset-replacement-register.csv'),
  `${[registerHeader, ...registerRows].map(row => row.map(csvCell).join(',')).join('\n')}\n`);
console.log(`Updated asset review register: ${register.size} stable IDs across ${registerStyles.length} styles`);
