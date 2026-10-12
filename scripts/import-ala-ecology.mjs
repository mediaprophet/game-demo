// Aggregate a region-filtered Atlas of Living Australia CSV into a reviewable,
// coordinate-free ecology report. This is not a GIS or road-risk classifier.
import { createHash } from 'node:crypto';
import { readFile, writeFile, mkdir } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const [csvArg, metadataArg, profileArg] = process.argv.slice(2);
if (!csvArg || !metadataArg || !profileArg) {
  throw new Error('Usage: node scripts/import-ala-ecology.mjs <occurrences.csv> <snapshot-metadata.json> <region-profile.json>');
}

function parseCsv(source) {
  const rows = [];
  let row = [], field = '', quoted = false;
  for (let i = 0; i < source.length; i++) {
    const ch = source[i];
    if (quoted) {
      if (ch === '"' && source[i + 1] === '"') { field += '"'; i++; }
      else if (ch === '"') quoted = false;
      else field += ch;
    } else if (ch === '"') quoted = true;
    else if (ch === ',') { row.push(field); field = ''; }
    else if (ch === '\n' || ch === '\r') {
      if (ch === '\r' && source[i + 1] === '\n') i++;
      row.push(field); field = '';
      if (row.some(value => value.trim())) rows.push(row);
      row = [];
    } else field += ch;
  }
  if (quoted) throw new Error('CSV has an unterminated quoted field');
  row.push(field);
  if (row.some(value => value.trim())) rows.push(row);
  if (rows.length < 2) throw new Error('CSV must contain a header and at least one data row');
  const headers = rows.shift().map(value => value.trim().toLowerCase());
  return rows.map(values => Object.fromEntries(headers.map((header, index) => [header, values[index] ?? ''])));
}

const csvPath = resolve(csvArg), metadataPath = resolve(metadataArg), profilePath = resolve(profileArg);
const [csvBytes, metadata, profile] = await Promise.all([
  readFile(csvPath), readFile(metadataPath, 'utf8').then(JSON.parse), readFile(profilePath, 'utf8').then(JSON.parse)
]);
if (![1, 2].includes(profile.schemaVersion) || !profile.id || !profile.scope) throw new Error('Unsupported or incomplete region profile');
if (metadata.sourceSystem !== 'Atlas of Living Australia' || !metadata.dataset || !metadata.downloadedAt ||
    !metadata.spatialQuery || metadata.regionProfileId !== profile.id || !metadata.attribution || !metadata.downloadDOI) {
  throw new Error('Metadata must include the ALA dataset, retrieval time, region query, matching profile ID, attribution and download DOI');
}

const rows = parseCsv(csvBytes.toString('utf8'));
const fields = Object.keys(rows[0] ?? {});
const pick = (...names) => fields.find(field => names.includes(field));
const taxonField = pick('taxonconceptid', 'taxonid');
const nameField = pick('scientificname', 'scientific_name');
const commonField = pick('vernacularname', 'commonname');
const occurrenceField = pick('occurrenceid', 'occurrence_id', 'uuid');
const dateField = pick('eventdate', 'event_date', 'eventtime');
const licenceField = pick('license', 'licence', 'rightsholder');
if (!taxonField && !nameField) throw new Error('ALA CSV needs scientificName or taxonConceptID');

const taxa = new Map();
const seen = new Set();
for (const [rowIndex, row] of rows.entries()) {
  const taxonId = (row[taxonField] || row[nameField] || '').trim();
  const scientificName = (row[nameField] || row[taxonField] || '').trim();
  if (!taxonId || !scientificName) continue;
  const occurrence = (row[occurrenceField] || '').trim();
  const dedupe = occurrence || `${taxonId}\n${row[dateField] || ''}\n${rowIndex}`;
  if (seen.has(dedupe)) continue;
  seen.add(dedupe);
  const entry = taxa.get(taxonId) ?? { taxonId, scientificName, commonNames: new Set(), recordCount: 0,
    firstObservation: null, lastObservation: null, licences: new Set() };
  const commonName = (row[commonField] || '').trim();
  const licence = (row[licenceField] || '').trim();
  const eventDate = (row[dateField] || '').trim().slice(0, 10);
  if (commonName) entry.commonNames.add(commonName);
  if (licence) entry.licences.add(licence);
  if (/^\d{4}-\d{2}-\d{2}$/.test(eventDate)) {
    if (!entry.firstObservation || eventDate < entry.firstObservation) entry.firstObservation = eventDate;
    if (!entry.lastObservation || eventDate > entry.lastObservation) entry.lastObservation = eventDate;
  }
  entry.recordCount++;
  taxa.set(taxonId, entry);
}

// schemaVersion 2: hazards carry a structured `taxon` object keyed by GBIF
// backbone key — index all three identity surfaces (legacy taxonId name
// string, structured scientific name, numeric GBIF key) so ALA records in
// any of those forms still join to the authored hazard context.
const profileHazards = new Map();
for (const item of profile.roadHazards || []) {
  if (item.taxonId) profileHazards.set(item.taxonId, item);
  if (item.taxon?.gbifScientificName) profileHazards.set(item.taxon.gbifScientificName, item);
  if (item.taxon?.gbifKey != null) profileHazards.set(String(item.taxon.gbifKey), item);
}
const species = [...taxa.values()].map(item => {
  const licences = [...item.licences].sort();
  const hazard = profileHazards.get(item.taxonId) || profileHazards.get(item.scientificName);
  return {
    taxonId: item.taxonId,
    scientificName: item.scientificName,
    commonNames: [...item.commonNames].sort(),
    recordCount: item.recordCount,
    observationDateRange: [item.firstObservation, item.lastObservation],
    licenceValues: licences,
    licenceStatus: licences.length ? 'recorded; assess every applicable term' : 'missing; review before reuse',
    occurrenceSuggestsPlacePresenceOnly: true,
    roadHazardEvidence: hazard?.evidenceStatus ?? 'none; no road-risk inference from occurrence records',
    modelRenderingApproved: false
  };
}).sort((a, b) => a.scientificName.localeCompare(b.scientificName));

const result = {
  schemaVersion: 1,
  kind: 'ala-region-ecology-review',
  regionProfileId: profile.id,
  regionName: profile.name,
  generatedAt: new Date().toISOString(),
  source: {
    sourceSystem: metadata.sourceSystem,
    dataset: metadata.dataset,
    datasetVersion: metadata.datasetVersion ?? null,
    downloadedAt: metadata.downloadedAt,
    downloadDOI: metadata.downloadDOI,
    attribution: metadata.attribution,
    spatialQuery: metadata.spatialQuery,
    sourceCsvSha256: createHash('sha256').update(csvBytes).digest('hex'),
    sourceRowCount: rows.length,
    deduplicatedOccurrenceCount: seen.size,
    coordinateHandling: 'source coordinates are not copied into this report',
    redistributionReview: 'required before packaging derived records'
  },
  species,
  approvedForRuntime: false,
  reviewNotes: [
    'Occurrence counts are not abundance estimates and reflect source sampling effort.',
    'Taxon presence does not imply current presence at a road, a collision rate, or a safe encounter point.',
    'Keep the source snapshot and metadata with the pack authoring record; review licences and sensitive taxa before distribution.'
  ]
};
const outPath = join(root, 'assets', 'ecology', 'review', `${profile.id}-ala-review.json`);
await mkdir(dirname(outPath), { recursive: true });
await writeFile(outPath, `${JSON.stringify(result, null, 2)}\n`);
process.stdout.write(`Wrote ${species.length} coordinate-free taxa to ${outPath}\n`);
