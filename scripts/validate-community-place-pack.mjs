// Validate a game-normalized place pack's graph references and rendered-file
// integrity. Source payloads remain opaque and must be parsed by approved
// QualiaDB/source adapters before a pack is created.
import { createHash } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { lstat, readFile, realpath } from 'node:fs/promises';
import { dirname, isAbsolute, relative, resolve, sep, win32 } from 'node:path';

const input = process.argv[2];
if (!input) throw new Error('Usage: node scripts/validate-community-place-pack.mjs <unpacked-place-pack-directory>');

const inputPath = await realpath(resolve(input));
const inputInfo = await lstat(inputPath);
const root = inputInfo.isDirectory() ? inputPath : await realpath(dirname(inputPath));
const manifestPath = inputInfo.isDirectory() ? resolve(root, 'place-pack.json') : inputPath;
const manifest = JSON.parse(await readFile(manifestPath, 'utf8'));
const errors = [];
const object = value => value !== null && typeof value === 'object' && !Array.isArray(value);
const nonEmpty = value => typeof value === 'string' && value.trim().length > 0;
const shaPattern = /^[a-f\d]{64}$/i;
const uniqueIndex = (items, label, getId = item => item?.id) => {
  const index = new Map();
  for (const [position, item] of (Array.isArray(items) ? items : []).entries()) {
    const id = getId(item);
    if (!nonEmpty(id)) errors.push(`${label}[${position}] has no id`);
    else if (index.has(id)) errors.push(`${label} contains duplicate id ${id}`);
    else index.set(id, item);
  }
  return index;
};
const checkRefs = (refs, index, label) => {
  if (!Array.isArray(refs) || refs.length === 0) { errors.push(`${label} must contain at least one reference`); return; }
  for (const ref of refs) if (!index.has(ref)) errors.push(`${label} references unknown id ${ref}`);
};
const sources = uniqueIndex(manifest?.sourceRecords, 'sourceRecords');
const sourceArtifactIds = new Set([...sources.values()].map(source => source?.artifactId).filter(nonEmpty));
const places = uniqueIndex(manifest?.places, 'places');
const nodes = uniqueIndex(manifest?.routeGraph?.nodes, 'routeGraph.nodes');
const edges = uniqueIndex(manifest?.routeGraph?.edges, 'routeGraph.edges');
const artifacts = uniqueIndex(manifest?.renderArtifacts, 'renderArtifacts');
const tiles = uniqueIndex(manifest?.renderTiles, 'renderTiles');
const dataLayers = uniqueIndex(manifest?.dataLayers ?? [], 'dataLayers');
const checkGeometryRef = (ref, label) => {
  if (!object(ref)) { errors.push(`${label} must be an object`); return; }
  checkRefs([ref.sourceId], sources, `${label}.sourceId`);
  const source = sources.get(ref.sourceId);
  if (!nonEmpty(ref.artifactId) || !nonEmpty(ref.featureId) || !nonEmpty(ref.format)) errors.push(`${label} needs artifactId, featureId and format`);
  if (source && source.artifactId !== ref.artifactId) errors.push(`${label}.artifactId does not match source ${ref.sourceId}`);
};

if (!object(manifest)) throw new Error('place-pack.json must contain a JSON object');
if (manifest.schemaVersion !== 1) errors.push(`unsupported schemaVersion: ${manifest.schemaVersion}`);
if (!/^[a-z0-9][a-z0-9._:-]{2,127}$/.test(manifest.packId || '')) errors.push('packId is missing or invalid');
if (!nonEmpty(manifest.title)) errors.push('title is required');
if (!['fictional_fixture', 'illustrative', 'reviewed', 'production'].includes(manifest.maturity)) errors.push('maturity is invalid');
if (!Array.isArray(manifest.sourceRecords) || manifest.sourceRecords.length === 0) errors.push('sourceRecords must contain at least one record');
if (!Array.isArray(manifest.places) || manifest.places.length === 0) errors.push('places must contain at least one place');
if (!object(manifest.routeGraph) || !Array.isArray(manifest.routeGraph.nodes) || !Array.isArray(manifest.routeGraph.edges)) errors.push('routeGraph.nodes and routeGraph.edges must be arrays');
if (object(manifest.routeGraph) && !(manifest.routeGraph.crs === null || nonEmpty(manifest.routeGraph.crs))) errors.push('routeGraph.crs must be a CRS string or null');
if (!Array.isArray(manifest.renderArtifacts) || !Array.isArray(manifest.renderTiles)) errors.push('renderArtifacts and renderTiles must be arrays');
if (!object(manifest.coverage)) errors.push('coverage must be an object');
else {
  checkRefs(manifest.coverage.sourceRefs, sources, 'coverage.sourceRefs');
  if (manifest.coverage.geometryRef) checkGeometryRef(manifest.coverage.geometryRef, 'coverage.geometryRef');
}

for (const [id, source] of sources) {
  if (!['observatory_artifact', 'public_dataset', 'game_authored'].includes(source.kind)) errors.push(`source ${id} has invalid kind`);
  if (!nonEmpty(source.title) || !nonEmpty(source.licenceStatus) || !nonEmpty(source.gameUseStatus)) errors.push(`source ${id} is missing provenance fields`);
}

for (const [id, place] of places) {
  checkRefs(place.sourceRefs, sources, `place ${id}.sourceRefs`);
  if (place.parentId !== null && place.parentId !== undefined && !places.has(place.parentId)) errors.push(`place ${id} has unknown parent ${place.parentId}`);
  if (place.geometryRef) checkGeometryRef(place.geometryRef, `place ${id}.geometryRef`);
}
for (const [id, place] of places) {
  const visited = new Set([id]);
  let parentId = place.parentId;
  while (parentId && places.has(parentId)) {
    if (visited.has(parentId)) { errors.push(`place hierarchy contains a cycle at ${id}`); break; }
    visited.add(parentId);
    parentId = places.get(parentId).parentId;
  }
}

for (const [id, node] of nodes) {
  checkRefs(node.sourceRefs, sources, `route node ${id}.sourceRefs`);
  if (node.placeId !== null && node.placeId !== undefined && !places.has(node.placeId)) errors.push(`route node ${id} has unknown place ${node.placeId}`);
  if (node.coordinate !== undefined && manifest.routeGraph.crs === null) errors.push(`route node ${id} has coordinates without a routeGraph CRS`);
}
for (const [id, edge] of edges) {
  if (!nodes.has(edge.fromNodeId)) errors.push(`route edge ${id} has unknown fromNodeId ${edge.fromNodeId}`);
  if (!nodes.has(edge.toNodeId)) errors.push(`route edge ${id} has unknown toNodeId ${edge.toNodeId}`);
  if (edge.fromNodeId === edge.toNodeId) errors.push(`route edge ${id} is a self-loop`);
  if (!(Number.isFinite(edge.distanceMetres) && edge.distanceMetres > 0)) errors.push(`route edge ${id} needs a positive distanceMetres`);
  checkRefs(edge.sourceRefs, sources, `route edge ${id}.sourceRefs`);
  if (edge.hazardProfileRefs !== undefined) checkRefs(edge.hazardProfileRefs, sources, `route edge ${id}.hazardProfileRefs`);
  if (edge.geometryRef) checkGeometryRef(edge.geometryRef, `route edge ${id}.geometryRef`);
}

async function digestFile(filePath) {
  const hash = createHash('sha256');
  for await (const chunk of createReadStream(filePath)) hash.update(chunk);
  return hash.digest('hex');
}

for (const [id, artifact] of artifacts) {
  if (!nonEmpty(artifact.role) || !nonEmpty(artifact.format)) errors.push(`render artifact ${id} needs role and format`);
  checkRefs(artifact.sourceRefs, sources, `render artifact ${id}.sourceRefs`);
  if (typeof artifact.path !== 'string' || !artifact.path.trim() || artifact.path.includes('\\') ||
      isAbsolute(artifact.path) || win32.isAbsolute(artifact.path) || artifact.path.split('/').some(part => !part || part === '.' || part === '..')) {
    errors.push(`render artifact ${id}.path must be a normalized package-relative path`);
    continue;
  }
  if (!shaPattern.test(artifact.sha256 || '')) { errors.push(`render artifact ${id}.sha256 must be a SHA-256 digest`); continue; }
  const filePath = resolve(root, ...artifact.path.split('/'));
  const pathInside = relative(root, filePath);
  if (!pathInside || pathInside === '..' || pathInside.startsWith(`..${sep}`) || isAbsolute(pathInside)) {
    errors.push(`render artifact ${id}.path escapes the package`);
    continue;
  }
  try {
    const resolvedFile = await realpath(filePath);
    const resolvedRelative = relative(root, resolvedFile);
    if (!resolvedRelative || resolvedRelative === '..' || resolvedRelative.startsWith(`..${sep}`) || isAbsolute(resolvedRelative)) {
      errors.push(`render artifact ${id}.path resolves outside the package`);
      continue;
    }
    if (!(await lstat(resolvedFile)).isFile()) { errors.push(`render artifact ${id}.path is not a regular file`); continue; }
    if ((await digestFile(resolvedFile)).toLowerCase() !== artifact.sha256.toLowerCase()) errors.push(`render artifact ${id}.sha256 does not match ${artifact.path}`);
  } catch (error) {
    errors.push(`render artifact ${id}.path cannot be read (${error.code || error.message})`);
  }
}

for (const [id, tile] of tiles) {
  if (!artifacts.has(tile.artifactId)) errors.push(`render tile ${id} references unknown artifact ${tile.artifactId}`);
  if (!Array.isArray(tile.bounds) || tile.bounds.length !== 4 || tile.bounds.some(value => !Number.isFinite(value)) ||
      !(tile.bounds[0] < tile.bounds[2] && tile.bounds[1] < tile.bounds[3]) || !nonEmpty(tile.crs)) errors.push(`render tile ${id} needs ordered four-coordinate bounds and a CRS`);
  checkRefs(tile.sourceRefs, sources, `render tile ${id}.sourceRefs`);
}
if (manifest.dataLayers !== undefined && !Array.isArray(manifest.dataLayers)) errors.push('dataLayers must be an array when present');
for (const [id, layer] of dataLayers) {
  if (!nonEmpty(layer.domain) || !Array.isArray(layer.sourceArtifactIds) || !Array.isArray(layer.renderArtifactIds) || !['display_context', 'terrain_input', 'route_input', 'scenario_context', 'fictional_gameplay', 'not_approved'].includes(layer.gameUse)) {
    errors.push(`data layer ${id} has invalid domain, artifact references or gameUse`);
  }
  if (layer.placeId !== null && layer.placeId !== undefined && !places.has(layer.placeId)) errors.push(`data layer ${id} has unknown place ${layer.placeId}`);
  checkRefs(layer.sourceRefs, sources, `data layer ${id}.sourceRefs`);
  if (Array.isArray(layer.sourceArtifactIds)) {
    for (const artifactId of layer.sourceArtifactIds) if (!sourceArtifactIds.has(artifactId)) errors.push(`data layer ${id} references unknown source artifact ${artifactId}`);
  }
  if (Array.isArray(layer.renderArtifactIds)) checkRefs(layer.renderArtifactIds, artifacts, `data layer ${id}.renderArtifactIds`);
}

if (errors.length) {
  process.stderr.write(`Place pack structure rejected with ${errors.length} issue(s):\n${errors.map(error => `- ${error}`).join('\n')}\n`);
  process.exitCode = 1;
} else {
  process.stdout.write(`Place pack references verified: ${places.size} place(s), ${nodes.size} route node(s), ${edges.size} route edge(s), ${artifacts.size} rendered artifact(s).\n`);
  process.stdout.write('This is structural validation only. It does not approve source rights, verify dataset claims, or parse source-owned Q42/RDF, Qualia Logic, VibeScript, or other payloads.\n');
}
