import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const require = createRequire(import.meta.url);
if (!process.env.PLAYWRIGHT_CORE) throw new Error('Set PLAYWRIGHT_CORE to an already installed playwright-core package; this check installs nothing.');
const { chromium } = require(process.env.PLAYWRIGHT_CORE);
const origin = new URL(process.env.ASSET_REVIEW_ORIGIN || 'http://127.0.0.1:8090');
if (!['127.0.0.1', 'localhost'].includes(origin.hostname)) throw new Error('This browser check is restricted to a local server.');
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const output = path.join(root, 'docs', 'qa', 'graphics-overhaul', `asset-review-${Date.now()}`);
await mkdir(output, { recursive: true });
const result = { kind: 'single-asset-browser-integration-check', origin: origin.origin, output, steps: [], requests: [], console: [], pageErrors: [], disposition: 'not-art-acceptance' };
const browser = await chromium.launch({ channel: process.env.BROWSER_CHANNEL || 'chrome', headless: false });
result.browser = browser.version();
const context = await browser.newContext({ viewport: { width: 1440, height: 960 }, acceptDownloads: true });
const page = await context.newPage();
page.on('request', request => result.requests.push({ url: request.url(), method: request.method() }));
page.on('console', message => result.console.push({ type: message.type(), text: message.text() }));
page.on('pageerror', error => result.pageErrors.push(error.message));
const step = name => {
  result.steps.push({ name, outcome: 'passed' });
  console.log(`Checked: ${name}`);
};
const status = () => page.locator('#status').innerText();
const select = async file => {
  await page.selectOption('#asset', file);
  await page.waitForFunction(() => !document.getElementById('load').disabled);
};
const load = async () => {
  await page.click('#load');
  await page.waitForFunction(() => {
    const text = document.getElementById('status').textContent;
    if (text.startsWith('VibeScript:')) throw new Error(text);
    return text.startsWith('Engine ingest and tick returned.');
  }, null, { timeout: 90000 });
};

try {
  await page.goto(new URL('/asset-review.html', origin).href, { waitUntil: 'domcontentloaded' });
  await page.waitForFunction(() => {
    const text = document.getElementById('status').textContent;
    if (text.startsWith('VibeScript:')) throw new Error(text);
    return document.getElementById('asset').options.length > 0 && !document.getElementById('asset').disabled;
  }, null, { timeout: 120000 });
  assert.equal(result.requests.some(request => request.url.endsWith('.10d')), false, 'startup must not fetch model geometry');
  assert.equal(result.requests.some(request => request.url.includes('.hmc')), false, 'startup must not fetch HMC packs');
  step('Vibe catalog loads without geometry or HMC fetch');

  const choices = await page.locator('#asset option').evaluateAll(options => options.map(option => ({ file: option.value, label: option.textContent })));
  const first = choices.find(choice => choice.label.includes('rc:asset/flora/banksia'));
  assert.ok(first, 'the current manifest must contain a banksia entry for this authored fixture');
  await select(first.file);
  await load();
  const verification = JSON.parse(await page.locator('#verify-claims').innerText());
  assert.equal(verification.manifest_digest_match, true);
  const report = JSON.parse(await page.locator('#engine-report').innerText());
  assert.ok(Number(report.vertices) > 0 && Number(report.triangles) > 0, 'engine must report real decoded mesh counts');
  assert.ok((await page.locator('#scene-label').innerText()).includes(first.file) || (await page.locator('#manifest-row').innerText()).includes(first.file));
  await page.selectOption('#render-key', 'banksia');
  const rdf = JSON.parse(await page.locator('#rdf-kv').innerText());
  assert.equal(rdf.status, 'authored-source-preview');
  assert.equal(rdf.pack_decoded, false);
  assert.equal(rdf.hash_identity_qualified, false);
  assert.equal(rdf.matches_manifest_iri_and_rank, true);
  assert.ok(rdf.statements.some(statement => statement.p === 'http://xmlns.com/foaf/0.1/depicts'));
  step('one banksia entry decodes and authored Vibe RDF remains explicitly non-pack evidence');
  const layout = await page.evaluate(() => ({ viewport: innerWidth, document: document.documentElement.scrollWidth, canvas: document.getElementById('scene').getBoundingClientRect().width }));
  assert.ok(layout.document <= layout.viewport + 1 && layout.canvas >= 600, 'desktop metadata must not overflow or collapse the review canvas');
  result.layout = layout;
  step('desktop metadata wraps without horizontal overflow and retains a usable canvas');
  await page.screenshot({ path: path.join(output, 'banksia.png'), fullPage: true });

  await page.locator('#cam-yaw').evaluate(input => { input.value = '0.4'; input.dispatchEvent(new Event('input', { bubbles: true })); });
  assert.equal(Number(await page.locator('#cam-yaw-v').innerText()), 0.4);
  step('camera input crosses the Vibe admission path');

  const second = choices.find(choice => choice.file !== first.file);
  assert.ok(second);
  await select(second.file);
  await load();
  assert.ok((await page.locator('#manifest-row').innerText()).includes(second.file));
  const geometry = result.requests.filter(request => new URL(request.url).pathname.endsWith('.10d'));
  assert.deepEqual(new Set(geometry.map(request => new URL(request.url).pathname)), new Set([
    `/asset-files/community/${first.file}`, `/asset-files/community/${second.file}`,
  ]));
  assert.equal(result.requests.some(request => request.url.includes('.hmc')), false);
  step('switching fetches only the two explicitly selected model files and no whole pack');

  await page.fill('#notes', 'Browser transport and Vibe integration check only; not art-quality acceptance.');
  const downloadPromise = page.waitForEvent('download');
  await page.click('#download');
  const download = await downloadPromise;
  const notePath = path.join(output, 'review-note.json');
  await download.saveAs(notePath);
  const note = JSON.parse(await readFile(notePath, 'utf8'));
  assert.equal(note.schema, 'single-asset-manual-uat-v1');
  assert.equal(note.assessment, 'not-reviewed');
  assert.equal(note.asset.file, second.file);
  step('Vibe-authored note preserves selected identity and no automatic art acceptance');

  const firstUrl = new URL(`/asset-files/community/${first.file}`, origin).href;
  const original = await context.request.get(firstUrl);
  assert.equal(original.status(), 200);
  const corrupted = Buffer.from(await original.body());
  corrupted[0] ^= 1;
  await page.route(firstUrl, route => route.fulfill({ status: 200, body: corrupted, contentType: 'application/octet-stream' }));
  await select(first.file);
  await page.click('#load');
  await page.waitForFunction(() => document.getElementById('status').textContent.includes('Asset SHA-256 differs from the manifest'), null, { timeout: 90000 });
  step('corrupted transport bytes are refused by the Vibe digest decision before render');
  await page.unroute(firstUrl);
  await load();
  await page.selectOption('#render-key', 'banksia');
  await page.screenshot({ path: path.join(output, 'final.png'), fullPage: true });
  result.renderer = JSON.parse(await page.locator('#renderer-kv').innerText());
  result.textureEvidence = JSON.parse(await page.locator('#texture-kv').innerText());
  result.package = JSON.parse(await page.locator('#wasm-kv').innerText());
  result.ok = true;
} catch (error) {
  result.ok = false;
  result.error = error.stack || String(error);
  result.status = await status().catch(() => 'page unavailable');
  result.sourceContext = await page.evaluate(async () => {
    const status = document.getElementById('status').textContent;
    if (!status.startsWith('VibeScript: ')) return null;
    const diagnostic = JSON.parse(status.slice('VibeScript: '.length));
    if (!diagnostic.span) return null;
    const taxon = await (await fetch('./fixtures/taxon-bindings.vibe')).text();
    const review = await (await fetch('./fixtures/asset-review.vibe')).text();
    const bytes = new TextEncoder().encode(taxon + '\n' + review);
    const [start, end] = diagnostic.span;
    return { span: diagnostic.span, token: new TextDecoder().decode(bytes.slice(start, end)), context: new TextDecoder().decode(bytes.slice(Math.max(0, start - 100), end + 100)) };
  }).catch(error => ({ error: String(error) }));
  await page.screenshot({ path: path.join(output, 'failure.png'), fullPage: true }).catch(() => {});
} finally {
  await writeFile(path.join(output, 'result.json'), JSON.stringify(result, null, 2) + '\n');
  console.log(JSON.stringify({ ok: result.ok, output, status: result.status, error: result.error, steps: result.steps }, null, 2));
  if (result.ok && process.env.KEEP_BROWSER_OPEN === '1') {
    console.log('Browser left open for manual UAT. Close that browser window to end this process.');
    await new Promise(resolve => browser.on('disconnected', resolve));
  } else {
    await browser.close();
  }
  if (!result.ok) process.exitCode = 1;
}
