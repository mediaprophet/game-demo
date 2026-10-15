import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import init, { asset_verify_10d, entity_verify_10d, entity_verify_bundle } from '../../../../web/pkg/rolling_commons_shell.js';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..');
const outDir = path.dirname(fileURLToPath(import.meta.url));
const rd = async rel => new Uint8Array(await readFile(path.join(root, rel)));
const plain = value => JSON.parse(JSON.stringify(value, (key, item) => typeof item === 'bigint' ? item.toString() : item));

const wasm = await rd('web/pkg/rolling_commons_shell_bg.wasm');
await init({ module_or_path: wasm });
const wasmSha = createHash('sha256').update(wasm).digest('hex');

const manifest = JSON.parse(await readFile(path.join(root, 'web/assets/community-manifest.json'), 'utf8'));
const checks = [];
const record = (name, detail, assertions) => {
  const failures = Object.entries(assertions).filter(([, ok]) => ok !== true).map(([key]) => key);
  checks.push({ name, detail, assertions, passed: failures.length === 0 });
  console.log(`${failures.length === 0 ? 'PASS' : 'FAIL'} ${name}${failures.length ? ' — failing: ' + failures.join(', ') : ''}`);
  console.log(`     ${detail}`);
};

const manifestEntry = file => manifest.assets.find(asset => asset.file === `10d/${path.basename(file)}`);

{
  const file = 'assets/generated/community/10d/ground--ea2522a51e5a.10d';
  const bytes = await rd(file);
  const asset = plain(asset_verify_10d(bytes));
  const entity = plain(entity_verify_10d(bytes));
  const digest = createHash('sha256').update(bytes).digest('hex');
  record('community ground hero', `${file} (${bytes.length} bytes) asset surfel_count=${asset.surfel_count}, entity valid=${entity.valid}, crc32c=${entity.crc32c_valid}, reverse_lod=${entity.reverse_lod_valid}, sections=${entity.sections_found?.length}`, {
    'asset_verify_10d surfel_count === 16': asset.surfel_count === 16,
    'entity_verify_10d valid === true': entity.valid === true,
    'crc32c_valid === true': entity.crc32c_valid === true,
    'manifest digest matches': digest === manifestEntry(file)?.sha256
  });
}

{
  const file = 'assets/generated/community/10d/flora-banksia-serrated-leaves--34b236461b80.10d';
  const bytes = await rd(file);
  const asset = plain(asset_verify_10d(bytes));
  const entity = plain(entity_verify_10d(bytes));
  const sections = entity.sections_found || [];
  record('community flora hero (banksia serrated-leaves)', `${file} (${bytes.length} bytes) vertices=${asset.vertices}, triangles=${asset.triangles}, surfels=${asset.surfel_count}; sections: ${sections.join(' | ')}`, {
    'entity_verify_10d valid === true': entity.valid === true,
    'ProceduralSynthesis (PROC) section present': sections.some(s => s.includes('ProceduralSynthesis')),
    'HierarchicalSurfels (SURF) section present': sections.some(s => s.includes('HierarchicalSurfels')),
    'surfel_count === 19 (manifest declared)': asset.surfel_count === 19,
    'manifest digest matches': createHash('sha256').update(bytes).digest('hex') === manifestEntry(file)?.sha256
  });
}

{
  const file = 'assets/generated/community/10d/resident-ada-head--f25345307733.10d';
  const bytes = await rd(file);
  const entity = plain(entity_verify_10d(bytes));
  const sections = entity.sections_found || [];
  const entry = manifestEntry(file);
  record('natural-person representative (resident-ada-head)', `${file} (${bytes.length} bytes) entity class=${entry?.entity?.class}, provenance sidecar=${sections.some(s => s.includes('ProvenanceSidecar'))}, sections=${sections.length}`, {
    'entity_verify_10d valid === true': entity.valid === true,
    'ProvenanceSidecar marker present': sections.some(s => s.includes('ProvenanceSidecar')),
    'manifest entity.class === NaturalPerson': entry?.entity?.class === 'NaturalPerson',
    'manifest digest matches': createHash('sha256').update(bytes).digest('hex') === entry?.sha256
  });
}

{
  const file = 'assets/generated/community/10d/ground--ea2522a51e5a--tier2--ce129d10.10d';
  const bytes = await rd(file);
  const entity = plain(entity_verify_10d(bytes));
  const tier = manifestEntry('10d/ground--ea2522a51e5a.10d')?.tiers?.lod2;
  record('ground tier2 decimated sidecar', `${file} (${bytes.length} bytes) valid=${entity.valid}, vertices=${entity.vertices}, triangles=${entity.triangles}`, {
    'entity_verify_10d valid === true': entity.valid === true,
    'manifest tier digest matches': tier?.file === `10d/${path.basename(file)}` && createHash('sha256').update(bytes).digest('hex') === tier?.sha256
  });
}

for (const pack of ['maslows-challenge-scenes.hmc', 'maslows-challenge-earthlight.hmc', 'maslows-challenge-community.hmc']) {
  const file = `web/assets/${pack}`;
  const bytes = await rd(file);
  const bundle = plain(entity_verify_bundle(bytes));
  record(`HMC bundle ${pack}`, `${file} (${bytes.length} bytes) report: ${JSON.stringify(bundle).slice(0, 300)}`, {
    'entity_verify_bundle valid === true': bundle?.valid === true
  });
}

const result = { wasm_sha256: wasmSha, generated_at: new Date().toISOString(), checks, passed: checks.every(check => check.passed) };
await writeFile(path.join(outDir, 'result.json'), JSON.stringify(result, null, 2) + '\n');
console.log(`\n${result.passed ? 'ALL SPOT CHECKS PASSED' : 'SPOT CHECK FAILURES PRESENT'} — evidence: ${outDir}/result.json`);
if (!result.passed) process.exitCode = 1;
