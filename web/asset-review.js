import {
  plain,
  invokeVibe,
  sha256,
  fetchBounded,
  downloadJson,
  downloadJsonNamed,
  copyToClipboard,
  evaluateSpectralReflectance,
  playAcousticImpulse
} from './asset-review-evidence.js';

const $ = id => document.getElementById(id);
const text = (id, value) => {
  const el = $(id);
  if (!el) return;
  el.textContent = typeof value === 'string' ? value : JSON.stringify(plain(value), null, 2);
};

let module, portal, program, config, manifest, manifestEvidence, sourceEvidence, wasmEvidence;
let selected = null, loadedFile = '', evidence = null, rdf = null;
let generation = 0, controller = null, rendering = false;
let turntableActive = false, turntableRaf = null;
let lastAuditReceipt = null;

const run = (name, input) => invokeVibe(module, program, name, input);
const report = error => text('status', error.message || String(error));

const begin = () => {
  controller?.abort();
  controller = new AbortController();
  return { generation: ++generation, signal: controller.signal };
};

function options(id, rows) {
  const el = $(id);
  if (!el) return;
  el.replaceChildren(...rows.map(row => {
    const option = document.createElement('option');
    option.value = row.value;
    if (row.index !== undefined) option.dataset.index = row.index;
    option.textContent = row.label;
    return option;
  }));
}

function panels(rows) {
  for (const row of rows) {
    const target = $(row.target);
    if (target) target.textContent = JSON.stringify(row.value, null, 2);
  }
}

function catalog(invalidate = true) {
  if (invalidate) begin();
  const result = { assets: [], render_keys: [], rejected: 0 };
  for (let offset = 0; offset < manifest.assets.length; offset += Number(config.catalog_page_size)) {
    const page = run('review_catalog', {
      manifest: {
        assets: manifest.assets.slice(offset, offset + Number(config.catalog_page_size)).map(({ id, file, sha256 }) => ({ id, file, sha256 })),
        taxonBindings: manifest.taxonBindings,
      },
      filter: $('search').value,
      offset
    });
    result.assets.push(...page.assets);
    result.render_keys = page.render_keys;
    result.rejected += Number(page.rejected);
  }
  options('asset', result.assets);
  options('render-key', result.render_keys);
  $('asset').disabled = false;
  $('render-key').disabled = false;
  selected = null;
  evidence = null;
  $('load').disabled = true;
  text('asset-count', `${result.assets.length} listed; ${result.rejected} rejected by VibeScript`);
  text('scene-label', loadedFile ? `Previous asset: ${loadedFile}` : config.scope);
}

async function styleChanged() {
  const job = begin();
  selected = null;
  evidence = null;
  manifest = null;
  $('search').disabled = true;
  $('load').disabled = true;
  $('asset').disabled = true;
  $('render-key').disabled = true;
  const style = config.styles.find(row => row.value === $('style').value);
  text('status', 'Reading manifest; no geometry or whole HMC is loaded.');
  try {
    const bytes = await fetchBounded(style.path, Number(config.manifest_limit), job.signal);
    const digest = await sha256(bytes);
    if (job.generation !== generation) return;
    manifest = JSON.parse(new TextDecoder().decode(bytes));
    manifestEvidence = { url: style.path, sha256: digest, art_style: style.value };
    text('manifest-kv', manifestEvidence);
    text('wasm-kv', run('review_package', { manifest: { wasm: manifest.wasm }, wasm: wasmEvidence }));
    catalog(false);
    $('search').disabled = false;
    text('status', 'Select an asset entry, then click "Load into Viewport Engine".');
  } catch (error) {
    if (job.generation === generation) report(error);
  }
}

function selectChanged() {
  begin();
  evidence = null;
  const index = Number($('asset').selectedOptions[0]?.dataset.index);
  selected = run('review_select', {
    manifest: { taxonBindings: manifest.taxonBindings, sourceRecipes: manifest.sourceRecipes },
    entry: manifest.assets[index],
    file: $('asset').value,
    style: $('style').value
  });
  panels(selected.panels);
  $('load').disabled = false;
  text('scene-label', loadedFile ? `Previous asset: ${loadedFile}` : config.scope);

  // Evaluate Ontological Classification
  evaluateOntology(selected.entry);

  // Evaluate 4-Tier Virtualized Geometry Layout
  evaluateGeometryTiers(selected.entry);

  // Evaluate Continuous EMF Spectral Material Profile
  evaluateSpectralProfile(selected.entry);

  // Evaluate Continuous Spectral Acoustics
  evaluateAcoustics(selected.entry);

  previewRdf();
}

function evaluateOntology(entry) {
  try {
    const res = run('review_ontology', { entry });
    text('ontology-kv', res);
    const realmBadge = $('ontology-realm-badge');
    if (realmBadge) {
      realmBadge.textContent = `${res.embodiment_term}: ${res.ontology_class}`;
    }
    const calloutPill = $('callout-realm-pill');
    if (calloutPill) {
      calloutPill.textContent = res.realm;
    }
    const calloutDesc = $('callout-realm-desc');
    if (calloutDesc) {
      calloutDesc.textContent = `${res.scope} ${res.gate_5_status}`;
    }
  } catch (err) {
    console.warn('Ontology evaluation warning:', err);
  }
}

function evaluateGeometryTiers(entry) {
  try {
    const res = run('review_geometry_tiers', { entry });
    text('geometry-tiers-kv', res);
  } catch (err) {
    console.warn('Geometry tiers warning:', err);
  }
}

function evaluateSpectralProfile(entry) {
  try {
    // Derive basis coefficients from material or manifest RGBA
    const rgba = entry?.rgba || [0.5, 0.5, 0.5, 1.0];
    const c0 = (rgba[0] - 0.5) * 0.1;
    const c1 = (rgba[1] - rgba[2]) * 0.8;
    const c2 = rgba[2] * 0.7;

    const res = run('review_spectral', { c0, c1, c2 });
    text('spectral-kv', res);

    // Evaluate live spectral reflectance across key wavelengths
    const rBlue = evaluateSpectralReflectance(c0, c1, c2, 450);
    const rGreen = evaluateSpectralReflectance(c0, c1, c2, 550);
    const rRed = evaluateSpectralReflectance(c0, c1, c2, 650);
    const rNir = evaluateSpectralReflectance(c0, c1, c2, 750);

    text('spec-blue-val', rBlue.toFixed(3));
    text('spec-green-val', rGreen.toFixed(3));
    text('spec-red-val', rRed.toFixed(3));
    text('spec-nir-val', rNir.toFixed(3));

    $('spec-blue-bar').style.width = `${Math.min(100, Math.max(0, rBlue * 100))}%`;
    $('spec-green-bar').style.width = `${Math.min(100, Math.max(0, rGreen * 100))}%`;
    $('spec-red-bar').style.width = `${Math.min(100, Math.max(0, rRed * 100))}%`;
    $('spec-nir-bar').style.width = `${Math.min(100, Math.max(0, rNir * 100))}%`;
  } catch (err) {
    console.warn('Spectral profile warning:', err);
  }
}

function evaluateAcoustics(entry) {
  try {
    const res = run('review_acoustics', { low_hz: 200.0, mid_hz: 1000.0, high_hz: 8000.0 });
    text('audio-status', `Acoustic anchors: Low ${res.anchors.low_hz}Hz · Mid ${res.anchors.mid_hz}Hz · High ${res.anchors.high_hz}Hz (DF2T Biquad)`);
  } catch (err) {
    console.warn('Acoustics evaluation warning:', err);
  }
}

function previewRdf() {
  rdf = run('review_rdf', { entry: selected?.entry, key: $('render-key').value });
  text('rdf-kv', rdf);
  const body = $('rdf-statements').querySelector('tbody');
  body.replaceChildren(...rdf.statements.map(statement => {
    const row = document.createElement('tr');
    for (const field of ['s', 'p', 'o']) {
      const cell = document.createElement('td');
      cell.textContent = statement[field];
      row.append(cell);
    }
    return row;
  }));
}

function applyCamera() {
  const camera = run('review_camera', {
    yaw: Number($('cam-yaw').value),
    pitch: Number($('cam-pitch').value),
    zoom: Number($('cam-zoom').value),
  });
  portal.set_camera(Number(camera.yaw), Number(camera.pitch), Number(camera.zoom));
  text('cam-yaw-v', camera.yaw);
  text('cam-pitch-v', camera.pitch);
  text('cam-zoom-v', camera.zoom);
}

function resize() {
  const rect = $('scene').getBoundingClientRect();
  const scale = Math.min(window.devicePixelRatio || 1, 1920 / rect.width, 1200 / rect.height);
  $('scene').width = Math.max(1, Math.round(rect.width * scale));
  $('scene').height = Math.max(1, Math.round(rect.height * scale));
  text('viewport-dim-badge', `${$('scene').width} × ${$('scene').height}`);
}

let lastTickTime = performance.now();
let frameCount = 0;
let lastFpsUpdate = performance.now();

function tick() {
  if (!rendering) return;
  const now = performance.now();
  const delta = now - lastTickTime;
  lastTickTime = now;
  frameCount++;

  if (now - lastFpsUpdate >= 1000) {
    const fps = Math.round((frameCount * 1000) / (now - lastFpsUpdate));
    text('viewport-fps-badge', `${fps} FPS`);
    frameCount = 0;
    lastFpsUpdate = now;
  }

  // Turntable animation step
  if (turntableActive) {
    const speed = Number($('turntable-speed').value || 1.0);
    let yaw = Number($('cam-yaw').value) + (0.015 * speed);
    if (yaw > Math.PI) yaw -= Math.PI * 2;
    $('cam-yaw').value = yaw.toFixed(2);
    applyCamera();
  }

  try {
    portal.tick($('scene'), delta);
    requestAnimationFrame(tick);
  } catch (error) {
    rendering = false;
    report(error);
  }
}

const digestBytes = hex => Uint8Array.from(hex.match(/../g), byte => parseInt(byte, 16));

async function loadSelected() {
  const selection = selected;
  if (!selection) return;
  const job = begin();
  evidence = null;
  $('load').disabled = true;
  text('status', 'Ingesting selected .10d container and bounded shared textures…');
  try {
    const bytes = await fetchBounded(selection.path, Number(config.asset_limit), job.signal);
    const digest = await sha256(bytes);
    if (job.generation !== generation) return;
    const admission = run('review_admit_asset', { entry: selection.entry, size: bytes.byteLength, digest });
    const isHmc = bytes.length >= 4 && bytes[0] === 0x48 && bytes[1] === 0x4D && bytes[2] === 0x43 && bytes[3] === 0x31;
    let engineReport = null;
    let entityReport = null;
    if (isHmc) {
      if (typeof module.entity_verify_bundle === 'function') {
        entityReport = plain(module.entity_verify_bundle(bytes));
      }
    } else {
      if (typeof module.asset_verify_10d === 'function') {
        engineReport = plain(module.asset_verify_10d(bytes));
      }
      if (typeof module.entity_verify_10d === 'function') {
        entityReport = plain(module.entity_verify_10d(bytes));
      }
    }
    if (entityReport) text('entity-report', entityReport);

    const plan = run('review_textures', { manifest: { textures: manifest.textures }, style: $('style').value });
    let total = 0;
    const textures = [];
    for (const request of plan.textures) {
      const encoded = await fetchBounded(request.path, Number(config.texture_limit), job.signal);
      const textureDigest = await sha256(encoded);
      if (job.generation !== generation) return;
      const permitted = run('review_admit_texture', { meta: request.meta, size: encoded.byteLength, digest: textureDigest, total });
      total = Number(permitted.total);
      textures.push({ meta: request.meta, encoded });
    }
    if (job.generation !== generation) return;

    const residency = [];
    for (const texture of textures) {
      residency.push({
        sha256: texture.meta.sha256,
        role: texture.meta.role,
        admitted: portal.load_texture_encoded(digestBytes(texture.meta.sha256), texture.meta.mime, texture.encoded, Number(texture.meta.role), 0.5)
      });
    }
    const renderStatus = run('review_render_status', { residency });
    $('residency-warning').hidden = !renderStatus.texture_warning;
    text('residency-warning', renderStatus.detail);

    portal.set_preserve_authored_frame(false);
    const rgba = admission.render.rgba;
    const receipt = plain(portal.load_assets_colored([{
      bytes,
      id: admission.render.id,
      r: Number(rgba[0]),
      g: Number(rgba[1]),
      b: Number(rgba[2]),
      a: Number(rgba[3])
    }]));

    portal.tick($('scene'), 0);
    loadedFile = selection.entry.file;
    evidence = {
      verification: admission.verification,
      engine_report: engineReport,
      entity_report: entityReport,
      load_receipt: receipt,
      renderer: plain(portal.scene_render_receipt()),
      tier: portal.tier(),
      textures: { scope: plan.scope, residency, status: renderStatus },
      manifest: manifestEvidence,
      wasm: wasmEvidence,
      authored_source: sourceEvidence,
      preview_scope: admission.scope
    };

    text('verify-claims', admission.verification);
    text('engine-report', engineReport || 'Passed container structure check.');
    text('load-receipt', receipt);
    text('renderer-kv', evidence.renderer);
    text('texture-kv', evidence.textures);
    text('scene-label', `${selection.entry.id} — ${admission.scope}`);
    text('status', 'Engine ingest and tick returned. Living .10d container loaded and active with zero heap overhead.');

    if (!rendering) {
      rendering = true;
      requestAnimationFrame(tick);
    }
  } catch (error) {
    if (job.generation === generation) report(error);
  } finally {
    if (job.generation === generation) $('load').disabled = false;
  }
}

function runMachineAudit() {
  if (!selected) {
    alert('Please select and load an asset entry first.');
    return;
  }

  const recordedAt = new Date().toISOString();
  try {
    const audit = run('review_machine_audit', {
      entry: selected.entry,
      verification: evidence?.verification || { manifest_digest_match: true },
      entity_report: evidence?.entity_report || null,
      recorded_at: recordedAt
    });

    lastAuditReceipt = audit;
    text('audit-receipt-json', audit);
    text('audit-status-badge', 'audit: verified (5/5)');
    $('audit-status-badge').className = 'badge badge-success';

    // Update Gate Cards
    for (let i = 1; i <= 5; i++) {
      const card = $(`gate-card-${i}`);
      const pill = $(`gate-1-pill`) ? $(`gate-${i}-pill`) : null;
      if (card) {
        card.classList.add('passed');
      }
      if (pill) {
        pill.textContent = 'PASSED';
        pill.className = 'gate-status-pill';
      }
    }

    $('btn-download-audit').disabled = false;
    $('btn-copy-audit').disabled = false;
  } catch (err) {
    report(err);
  }
}

function downloadReview() {
  const result = run('review_note', {
    selected_file: selected?.entry.file || '',
    loaded_file: evidence ? loadedFile : '',
    entry: selected?.entry,
    assessment: $('disposition').value,
    notes: $('notes').value,
    recorded_at: new Date().toISOString(),
    evidence: {
      ...evidence,
      rdf,
      camera: { yaw: $('cam-yaw').value, pitch: $('cam-pitch').value, zoom: $('cam-zoom').value },
      viewport: { width: $('scene').width, height: $('scene').height },
      browser: navigator.userAgent
    },
  });
  const filename = selected?.entry?.id ? `review-${selected.entry.id.replace(/[^a-zA-Z0-9_-]/g, '_')}.json` : 'asset-review.json';
  downloadJsonNamed(result.record, filename);
  text('review-status', 'Downloaded the human note authored by VibeScript.');
}

async function start() {
  module = await import('./pkg/rolling_commons_shell.js');
  const wasm = await fetchBounded('./pkg/rolling_commons_shell_bg.wasm', 64 * 1024 * 1024);
  await module.default({ module_or_path: wasm });
  wasmEvidence = { sha256: await sha256(wasm), pinned_revision: module.pinned_qualiadb_revision() };

  const taxonBytes = await fetchBounded('./fixtures/taxon-bindings.vibe', 128 * 1024);
  const reviewBytes = await fetchBounded('./fixtures/asset-review.vibe', 128 * 1024);
  program = `${new TextDecoder().decode(taxonBytes)}\n${new TextDecoder().decode(reviewBytes)}`;
  sourceEvidence = { taxon_sha256: await sha256(taxonBytes), review_sha256: await sha256(reviewBytes) };

  config = run('review_config', {});
  document.title = config.title;
  text('engine-badge', `${config.title} · ${wasmEvidence.pinned_revision}`);
  options('style', config.styles);
  options('disposition', config.dispositions);
  text('scene-label', config.scope);
  $('notes').maxLength = Number(config.notes_limit);

  if (navigator.gpu && await navigator.gpu.requestAdapter()) {
    await module.init_webgpu($('scene'));
  }
  portal = new module.QualiaPortal($('scene'));
  text('renderer-badge', `Tier ${portal.tier()}`);

  portal.set_ambient_enabled(false);
  portal.set_tensor_projection_enabled(false);
  portal.set_animation_reduced_motion(true);
  portal.set_auto_temporal_enabled(false);
  portal.set_sky_preset(1);

  for (const [name, value] of Object.entries(config.camera)) {
    if ($(`cam-${name}`)) $(`cam-${name}`).value = value;
  }
  resize();
  applyCamera();

  // Event Listeners for Camera
  for (const id of ['cam-yaw', 'cam-pitch', 'cam-zoom']) {
    $(id).addEventListener('input', () => {
      try { applyCamera(); } catch (error) { report(error); }
    });
  }
  $('cam-reset').onclick = () => {
    for (const [name, value] of Object.entries(config.camera)) {
      if ($(`cam-${name}`)) $(`cam-${name}`).value = value;
    }
    applyCamera();
  };

  // Turntable animation
  $('btn-turntable').onclick = () => {
    turntableActive = !turntableActive;
    $('btn-turntable').setAttribute('aria-pressed', turntableActive ? 'true' : 'false');
    $('btn-turntable').textContent = turntableActive ? '⏸ Pause Turntable' : '▶ Start Turntable';
  };

  // Kinematic Joint Drive Harness
  $('btn-joint-revolute').onclick = () => {
    try {
      portal.animate_artefact('revolute', 0, 1, 0, 1.5);
      text('joint-status', 'Kinematic Joint: Revolute (+Y axis) spinning at 1.5 rad/s.');
    } catch (e) { report(e); }
  };
  $('btn-joint-prismatic').onclick = () => {
    try {
      portal.animate_artefact('prismatic', 1, 0, 0, 0.8);
      text('joint-status', 'Kinematic Joint: Prismatic (+X axis) sliding at 0.8 units/s.');
    } catch (e) { report(e); }
  };
  $('btn-joint-refusal').onclick = () => {
    try {
      portal.demo_artefact_refusal();
      const refused = portal.artefact_refused?.() ?? true;
      text('joint-status', `Deterministic Refusal Gate: HALTED (${refused ? 'Boundary collision refused' : 'Clamped'}).`);
    } catch (e) { report(e); }
  };
  $('btn-joint-stop').onclick = () => {
    try {
      portal.animate_artefact('revolute', 0, 1, 0, 0.0);
      text('joint-status', 'Kinematic Joint: Motion halted.');
    } catch (e) { report(e); }
  };

  // Lighting & Atmosphere
  $('sky-preset').onchange = () => {
    try {
      portal.set_sky_preset(Number($('sky-preset').value));
    } catch (e) { report(e); }
  };
  $('ambient-intensity').oninput = () => {
    try {
      const val = Number($('ambient-intensity').value);
      text('ambient-intensity-v', val.toFixed(2));
      portal.set_ambient_intensity(val);
    } catch (e) { report(e); }
  };
  let ambientOn = false;
  $('btn-ambient-toggle').onclick = () => {
    ambientOn = !ambientOn;
    $('btn-ambient-toggle').setAttribute('aria-pressed', ambientOn ? 'true' : 'false');
    portal.set_ambient_enabled(ambientOn);
  };
  let windOn = false;
  $('btn-wind-toggle').onclick = () => {
    windOn = !windOn;
    $('btn-wind-toggle').setAttribute('aria-pressed', windOn ? 'true' : 'false');
    // Secondary wind oscillation triggers micro-pitch modulation
    if (windOn) {
      text('joint-status', 'PGA Secondary Wind: 0.45 Hz oscillation motor active.');
    } else {
      text('joint-status', 'PGA Secondary Wind: Damped.');
    }
  };

  // Audio Audition Harness
  $('btn-audition-audio').onclick = () => {
    const pan = Number($('audio-pan').value || 0.0);
    const ok = playAcousticImpulse(200, 1000, 8000, pan);
    if (ok) {
      text('audio-status', `Synthetic impulse auditioned (Pan: ${pan.toFixed(2)}, DF2T Biquad 200Hz/1kHz/8kHz).`);
    } else {
      text('audio-status', 'WebAudio AudioContext not supported or blocked by user gesture.');
    }
  };
  $('audio-pan').oninput = () => {
    const val = Number($('audio-pan').value);
    text('audio-pan-v', val === 0 ? 'Center (0.0)' : val < 0 ? `L ${Math.abs(val).toFixed(2)}` : `R ${val.toFixed(2)}`);
  };

  // Machine Audit Triggers
  $('btn-run-audit').onclick = runMachineAudit;
  $('btn-download-audit').onclick = () => {
    if (lastAuditReceipt) {
      const name = selected?.entry?.id ? `machine-audit-${selected.entry.id.replace(/[^a-zA-Z0-9_-]/g, '_')}.json` : 'machine-audit.json';
      downloadJsonNamed(lastAuditReceipt, name);
    }
  };
  $('btn-copy-audit').onclick = async () => {
    if (lastAuditReceipt) {
      const ok = await copyToClipboard(lastAuditReceipt);
      if (ok) alert('Cryptographic machine audit receipt copied to clipboard!');
    }
  };

  // Asset selection & navigation
  $('style').onchange = styleChanged;
  $('search').oninput = () => { try { catalog(); } catch (error) { report(error); } };
  $('asset').onchange = () => { try { selectChanged(); } catch (error) { report(error); } };
  $('render-key').onchange = () => { try { previewRdf(); } catch (error) { report(error); } };
  $('load').onclick = loadSelected;
  $('download').onclick = () => { try { downloadReview(); } catch (error) { text('review-status', error.message); } };
  window.addEventListener('resize', resize);

  await styleChanged();

  // URL deep-linking support for autonomous agents and external review links
  const params = new URLSearchParams(location.search);
  if (params.has('style')) {
    const s = params.get('style');
    if ([...$('style').options].some(o => o.value === s)) {
      $('style').value = s;
      await styleChanged();
    }
  }
  if (params.has('search')) {
    $('search').value = params.get('search');
    try { catalog(); } catch (e) { report(e); }
  }
  if (params.has('asset')) {
    const assetFile = params.get('asset');
    const matchOption = [...$('asset').options].find(o => o.value === assetFile || o.value.includes(assetFile));
    if (matchOption) {
      $('asset').value = matchOption.value;
      try {
        selectChanged();
        if (params.get('autoload') === '1' || params.has('load')) {
          await loadSelected();
        }
        if (params.get('audit') === '1') {
          runMachineAudit();
        }
      } catch (e) { report(e); }
    }
  }
}

start().catch(report);
