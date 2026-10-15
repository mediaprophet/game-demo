import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import { copyFile, mkdir, readFile, readdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const require = createRequire(import.meta.url);
if (!process.env.PLAYWRIGHT_CORE) throw new Error('Set PLAYWRIGHT_CORE to an installed playwright-core package.');
const { chromium } = require(process.env.PLAYWRIGHT_CORE);
const origin = new URL(process.env.ASSET_REVIEW_ORIGIN || 'http://127.0.0.1:8090');
if (!['127.0.0.1', 'localhost'].includes(origin.hostname)) throw new Error('Only a local test server is allowed.');
const gameDemo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const modelFolder = path.resolve(gameDemo, '..', 'ecopunk', 'assets', 'models', 'corymbia-aparrerinja');
const modelSet = JSON.parse(await readFile(path.join(modelFolder, 'model-set.json'), 'utf8'));
const output = path.join(gameDemo, 'docs', 'qa', 'graphics-overhaul', `ecopunk-models-${Date.now()}`);
await mkdir(output, { recursive: true });
const result = { origin: origin.origin, modelFolder, output, models: [], pageErrors: [] };
const browser = await chromium.launch({ headless: true, ...(process.env.BROWSER_CHANNEL ? { channel: process.env.BROWSER_CHANNEL } : {}) });
let page;
try {
  const context = await browser.newContext({ viewport: { width: 1440, height: 960 }, acceptDownloads: true });
  page = await context.newPage();
  page.on('pageerror', error => result.pageErrors.push(error.message));
  await page.goto(new URL('/asset-review.html', origin).href, { waitUntil: 'domcontentloaded' });
  await page.waitForFunction(() => {
    const status = document.getElementById('status').textContent;
    if (status.startsWith('VibeScript:')) throw new Error(status);
    return !document.getElementById('asset').disabled;
  }, null, { timeout: 120000 });
  await page.locator('#local-model-folder').setInputFiles(modelFolder);
  await page.waitForFunction(() => {
    const status = document.getElementById('local-model-status').textContent;
    if (status.includes('missing') || status.includes('invalid') || status.includes('Unsupported')) throw new Error(status);
    return !document.getElementById('local-model-load').disabled;
  });
  assert.equal(await page.locator('#local-model option').count(), modelSet.models.length);

  for (const model of modelSet.models) {
    await page.selectOption('#local-model', model.id);
    await page.click('#local-model-load');
    await page.waitForFunction(() => {
      const status = document.getElementById('local-model-status').textContent;
      return status.includes('verified and rendered.') || !document.getElementById('local-model-load').disabled;
    }, null, { timeout: 30000 });
    const status = await page.locator('#local-model-status').innerText();
    assert.ok(status.includes('verified and rendered.'), `${model.id}: ${status}`);
    const receipt = JSON.parse(await page.locator('#local-model-receipt').innerText());
    assert.equal(receipt.model, model.id);
    assert.equal(receipt.checks.length, model.components.length);
    assert.ok(receipt.checks.every(check => check.entityVerification?.valid === true));
    assert.equal(await page.locator('#btn-run-audit').isDisabled(), true);
    await page.locator('#scene').screenshot({ path: path.join(output, `${model.id}.png`) });
    result.models.push({ id: model.id, components: receipt.checks.length, load: receipt.load, renderer: receipt.renderer });
    console.log(`Verified and rendered: ${model.id} (${receipt.checks.length} components)`);
  }
  assert.deepEqual(result.pageErrors, []);
  const downloadPromise = page.waitForEvent('download');
  await page.click('#local-model-download');
  const download = await downloadPromise;
  await download.saveAs(path.join(output, 'model-receipt.json'));

  const localReceiptCleared = async () => {
    assert.equal(await page.locator('#local-model-receipt').innerText(), 'No local model loaded.');
    assert.equal(await page.locator('#local-model-download').isDisabled(), true);
  };
  await page.selectOption('#local-model', modelSet.models[1].id);
  await localReceiptCleared();
  assert.match(await page.locator('#local-model-status').innerText(), /Selection changed/);
  assert.equal(await page.locator('#local-model-load').isDisabled(), false);
  console.log('Selection change cancels and invalidates the stale local receipt');

  const firstAssetOption = await page.locator('#asset option').first().getAttribute('value');
  await page.click('#local-model-load');
  await page.waitForFunction(() => document.getElementById('local-model-status').textContent.includes('verified and rendered.'), null, { timeout: 120000 });
  await page.selectOption('#asset', firstAssetOption);
  await localReceiptCleared();
  assert.equal(await page.locator('#btn-run-audit').isDisabled(), true);
  assert.equal(await page.locator('#local-model-load').isDisabled(), false);
  console.log('Catalog selection cancels local work and invalidates the stale receipt');

  const scratch = path.join(output, 'scratch-models');
  const tamperedFolder = async (name, mutate) => {
    const dir = path.join(scratch, name);
    await mkdir(path.join(dir, '10d'), { recursive: true });
    for (const file of await readdir(path.join(modelFolder, '10d'))) {
      await copyFile(path.join(modelFolder, '10d', file), path.join(dir, '10d', file));
    }
    await mkdir(path.join(dir, 'graph'), { recursive: true });
    for (const file of await readdir(path.join(modelFolder, 'graph'))) {
      await copyFile(path.join(modelFolder, 'graph', file), path.join(dir, 'graph', file));
    }
    const set = JSON.parse(JSON.stringify(modelSet));
    mutate(set);
    await writeFile(path.join(dir, 'model-set.json'), JSON.stringify(set));
    return dir;
  };
  const openFolder = dir => page.locator('#local-model-folder').setInputFiles(dir);

  await openFolder(await tamperedFolder('bad-color', set => { set.models[0].components[0].rgba[0] = 1.5; }));
  await page.waitForFunction(() => document.getElementById('local-model-status').textContent.includes('Missing or invalid component'), null, { timeout: 30000 });
  assert.equal(await page.locator('#local-model-load').isDisabled(), true);
  assert.equal(await page.locator('#local-model option').count(), 0);
  await localReceiptCleared();
  console.log('Rejected: out-of-range rgba before portal load');

  await openFolder(await tamperedFolder('over-aggregate', set => {
    for (const component of set.models[0].components) component.bytes = 7 * 1024 * 1024;
    const extra = { ...set.models[0].components[1], id: `${set.models[0].components[1].id}-extra`, bytes: 7 * 1024 * 1024 };
    set.models[0].components.push(extra);
  }));
  await page.waitForFunction(() => document.getElementById('local-model-status').textContent.includes('aggregate bound'), null, { timeout: 30000 });
  assert.equal(await page.locator('#local-model-load').isDisabled(), true);
  console.log('Rejected: declared aggregate over 32 MiB before portal load');

  await openFolder(await tamperedFolder('bad-digest', set => {
    const digest = set.models[0].components[0].sha256;
    set.models[0].components[0].sha256 = (digest[0] === '0' ? '1' : '0') + digest.slice(1);
  }));
  await page.waitForFunction(() => !document.getElementById('local-model-load').disabled, null, { timeout: 30000 });
  await page.selectOption('#local-model', modelSet.models[0].id);
  await page.click('#local-model-load');
  await page.waitForFunction(() => document.getElementById('local-model-status').textContent.includes('differs'), null, { timeout: 120000 });
  await localReceiptCleared();
  console.log('Rejected: tampered component digest before portal load');

  const duplicateIds = await tamperedFolder('duplicate-ids', set => { set.models.push({ ...set.models[0] }); });
  await openFolder(duplicateIds);
  await page.waitForFunction(() => document.getElementById('local-model-status').textContent.includes('unique nonempty'), null, { timeout: 30000 });
  assert.equal(await page.locator('#local-model-load').isDisabled(), true);
  console.log('Rejected: duplicate model ids at folder open');

  await openFolder(modelFolder);
  await page.waitForFunction(() => !document.getElementById('local-model-load').disabled, null, { timeout: 30000 });
  assert.equal(await page.locator('#local-model option').count(), modelSet.models.length);
  await page.selectOption('#local-model', modelSet.models[0].id);
  await page.click('#local-model-load');
  await page.waitForFunction(() => document.getElementById('local-model-status').textContent.includes('verified and rendered.'), null, { timeout: 120000 });
  assert.deepEqual(result.pageErrors, []);
  result.passed = true;
} catch (error) {
  result.passed = false;
  result.error = error.stack || String(error);
  if (page) {
    result.localModelStatus = await page.locator('#local-model-status').innerText().catch(() => null);
    result.pageStatus = await page.locator('#status').innerText().catch(() => null);
  }
  throw error;
} finally {
  await writeFile(path.join(output, 'result.json'), JSON.stringify(result, null, 2));
  await browser.close();
  console.log(`Evidence: ${output}`);
}
