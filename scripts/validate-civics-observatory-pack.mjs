// Validate the game-side package envelope and payload digests for a future
// Observatory export. Source artifacts remain opaque; this does not parse or
// execute Q42/RDF, Qualia Logic, VibeScript, or any other payload.
import { createHash } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { lstat, readFile, realpath } from 'node:fs/promises';
import { isAbsolute, relative, resolve, sep, win32 } from 'node:path';

const input = process.argv[2];
if (!input) {
  throw new Error('Usage: node scripts/validate-civics-observatory-pack.mjs <unpacked-package-directory>');
}

const packagePath = await realpath(resolve(input));
const manifestPath = resolve(packagePath, 'manifest.json');
const manifest = JSON.parse(await readFile(manifestPath, 'utf8'));
const errors = [];
const isObject = value => value !== null && typeof value === 'object' && !Array.isArray(value);
const nonEmpty = value => typeof value === 'string' && value.trim().length > 0;
const shaPattern = /^[a-f\d]{64}$/i;
const uri = value => { try { return Boolean(new URL(value).protocol); } catch { return false; } };
const dateTime = value => typeof value === 'string' && /^\d{4}-\d\d-\d\dT/.test(value) && !Number.isNaN(Date.parse(value));
if (!isObject(manifest)) throw new Error('manifest.json must contain a JSON object');

if (manifest.manifest_type !== 'civics-observatory-game-pack') errors.push('manifest_type must be civics-observatory-game-pack');
if (manifest.manifest_version !== 1) errors.push(`unsupported manifest_version: ${manifest.manifest_version}`);
if (!/^[a-z0-9][a-z0-9._:-]{2,127}$/.test(manifest.package_id || '')) errors.push('package_id is missing or invalid');
if (!dateTime(manifest.created_at)) errors.push('created_at must be an ISO date-time');
if (!uri(manifest.registry_uri)) errors.push('registry_uri must be a URI');
if (!Array.isArray(manifest.development_project_uris) || manifest.development_project_uris.some(value => !uri(value))) {
  errors.push('development_project_uris must be an array of URIs');
}
if (!Array.isArray(manifest.sources) || manifest.sources.length === 0) errors.push('sources must contain at least one source');

const sourceIds = new Set();
const artifactIds = new Set();
const artifactPaths = new Set();
let checkedArtifacts = 0;

async function digestFile(filePath) {
  const hash = createHash('sha256');
  for await (const chunk of createReadStream(filePath)) hash.update(chunk);
  return hash.digest('hex');
}

for (const [sourceIndex, source] of (Array.isArray(manifest.sources) ? manifest.sources : []).entries()) {
  if (!isObject(source)) { errors.push(`sources[${sourceIndex}] must be an object`); continue; }
  if (!/^[a-z0-9][a-z0-9._:/-]{2,255}$/.test(source.source_id || '')) errors.push(`sources[${sourceIndex}].source_id is missing or invalid`);
  else if (sourceIds.has(source.source_id)) errors.push(`duplicate source_id: ${source.source_id}`);
  else sourceIds.add(source.source_id);
  for (const field of ['title', 'update_cadence']) {
    if (!nonEmpty(source[field])) errors.push(`source ${source.source_id || sourceIndex}: ${field} is required`);
  }
  if (!uri(source.source_uri)) errors.push(`source ${source.source_id || sourceIndex}: source_uri must be a URI`);
  if (!dateTime(source.snapshot_at)) errors.push(`source ${source.source_id || sourceIndex}: snapshot_at must be an ISO date-time`);
  if (source.release !== undefined && source.release !== null && typeof source.release !== 'string') {
    errors.push(`source ${source.source_id || sourceIndex}: release must be a string or null`);
  }
  if (!isObject(source.publisher) || !nonEmpty(source.publisher.name) || !uri(source.publisher.uri)) {
    errors.push(`source ${source.source_id || sourceIndex}: publisher.name and publisher.uri are required`);
  }
  if (!isObject(source.licence) || !nonEmpty(source.licence.statement) || !nonEmpty(source.licence.attribution) ||
      !(source.licence.uri === null || uri(source.licence.uri)) ||
      !['permitted', 'conditional', 'prohibited', 'unknown'].includes(source.licence.redistribution)) {
    errors.push(`source ${source.source_id || sourceIndex}: licence statement, attribution and redistribution status are required`);
  }
  if (!['public_download', 'public_api_snapshot', 'public_reference_only', 'restricted', 'synthetic_demo', 'other'].includes(source.access) ||
      !['observed', 'prepared_public_context', 'reviewed_evidence', 'preliminary_model', 'concept_reference', 'synthetic_demo', 'other'].includes(source.maturity)) {
    errors.push(`source ${source.source_id || sourceIndex}: access and maturity must use a supported v1 value`);
  }
  const geography = source.geography;
  const bounds = geography?.bounds;
  const boundsValid = bounds === undefined || bounds === null ||
    (Array.isArray(bounds) && bounds.length === 4 && bounds.every(value => typeof value === 'number' && Number.isFinite(value)));
  if (!isObject(geography) || !nonEmpty(geography.scope) || !('crs' in geography) || !('time_period' in geography) ||
      !(geography.crs === null || typeof geography.crs === 'string') ||
      !(geography.time_period === null || typeof geography.time_period === 'string') || !boundsValid) {
    errors.push(`source ${source.source_id || sourceIndex}: geography.scope, crs and time_period are required`);
  }
  const transformations = source.transformations;
  const transformationsValid = transformations === undefined ||
    (Array.isArray(transformations) && transformations.every(item => {
      if (!isObject(item)) return false;
      return nonEmpty(item.id) && nonEmpty(item.version) && nonEmpty(item.description) &&
        (item.code_sha256 === undefined || (typeof item.code_sha256 === 'string' && shaPattern.test(item.code_sha256)));
    }));
  if ((source.vocabulary_versions !== undefined && (!Array.isArray(source.vocabulary_versions) || source.vocabulary_versions.some(value => !nonEmpty(value)))) ||
      !transformationsValid) {
    errors.push(`source ${source.source_id || sourceIndex}: vocabulary_versions and transformations must follow the v1 schema`);
  }
  if (!Array.isArray(source.uncertainty) || source.uncertainty.some(value => !nonEmpty(value)) ||
      !Array.isArray(source.prohibited_inferences) || source.prohibited_inferences.some(value => !nonEmpty(value)) ||
      !['not_assessed', 'reviewed_no_personal_data', 'review_required', 'restricted'].includes(source.privacy_review)) {
    errors.push(`source ${source.source_id || sourceIndex}: uncertainty and prohibited_inferences must be arrays and privacy_review must use a supported v1 value`);
  }
  if (!Array.isArray(source.artifacts) || source.artifacts.length === 0) {
    errors.push(`source ${source.source_id || sourceIndex}: artifacts must contain at least one payload`);
    continue;
  }

  for (const [artifactIndex, artifact] of source.artifacts.entries()) {
    const label = `${source.source_id || sourceIndex}.artifacts[${artifactIndex}]`;
    if (!isObject(artifact)) { errors.push(`${label} must be an object`); continue; }
    if (!nonEmpty(artifact.artifact_id) || !nonEmpty(artifact.role) || !nonEmpty(artifact.format)) {
      errors.push(`${label} requires artifact_id, role and format`);
    }
    if ((artifact.media_type !== undefined && artifact.media_type !== null && typeof artifact.media_type !== 'string') ||
        (artifact.encoding !== undefined && artifact.encoding !== null && typeof artifact.encoding !== 'string')) {
      errors.push(`${label}: media_type and encoding must be strings or null`);
    }
    if (nonEmpty(artifact.artifact_id)) {
      if (artifactIds.has(artifact.artifact_id)) errors.push(`duplicate artifact_id: ${artifact.artifact_id}`);
      artifactIds.add(artifact.artifact_id);
    }
    if (typeof artifact.path !== 'string' || !artifact.path.trim() || artifact.path.includes('\\') ||
        isAbsolute(artifact.path) || win32.isAbsolute(artifact.path) || artifact.path.split('/').some(part => !part || part === '.' || part === '..')) {
      errors.push(`${label}.path must be a normalized package-relative path using forward slashes`);
      continue;
    }
    if (!shaPattern.test(artifact.sha256 || '')) { errors.push(`${label}.sha256 must be a 64-character SHA-256 digest`); continue; }
    if (artifactPaths.has(artifact.path)) errors.push(`duplicate artifact path: ${artifact.path}`);
    artifactPaths.add(artifact.path);

    const filePath = resolve(packagePath, ...artifact.path.split('/'));
    const rel = relative(packagePath, filePath);
    if (!rel || rel === '..' || rel.startsWith(`..${sep}`) || isAbsolute(rel)) {
      errors.push(`${label}.path escapes the package directory`);
      continue;
    }
    try {
      const resolvedFile = await realpath(filePath);
      const resolvedRelative = relative(packagePath, resolvedFile);
      if (!resolvedRelative || resolvedRelative === '..' || resolvedRelative.startsWith(`..${sep}`) || isAbsolute(resolvedRelative)) {
        errors.push(`${label}.path resolves outside the package directory`);
        continue;
      }
      if (!(await lstat(resolvedFile)).isFile()) { errors.push(`${label}.path is not a regular file`); continue; }
      const actual = await digestFile(resolvedFile);
      if (actual.toLowerCase() !== artifact.sha256.toLowerCase()) {
        errors.push(`${label}.sha256 does not match ${artifact.path}`);
        continue;
      }
      checkedArtifacts++;
    } catch (error) {
      errors.push(`${label}.path cannot be read (${error.code || error.message})`);
    }
  }
}

if (errors.length) {
  process.stderr.write(`Package envelope rejected with ${errors.length} issue(s):\n${errors.map(error => `- ${error}`).join('\n')}\n`);
  process.exitCode = 1;
} else {
  process.stdout.write(`Package envelope and SHA-256 digests verified: ${manifest.sources.length} source(s), ${checkedArtifacts} opaque artifact(s).\n`);
  process.stdout.write('This scaffold does not authenticate publishers, grant redistribution rights, approve runtime use, or interpret RDF/Q42, Qualia Logic, VibeScript, or other payload semantics.\n');
}
