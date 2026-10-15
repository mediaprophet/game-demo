import { createRequire } from 'node:module';
import { writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const require = createRequire(import.meta.url);
const { chromium } = require(process.env.PLAYWRIGHT_CORE);
const outDir = path.dirname(fileURLToPath(import.meta.url));
const origin = process.env.GAME_ORIGIN || 'http://127.0.0.1:8091';
const result = { url: `${origin}/`, console: [], pageErrors: [], requests_failed: [], ready: false };
const browser = await chromium.launch({ channel: 'chrome', headless: false });
try {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 960 } })).newPage();
  page.on('console', m => { if (result.console.length < 200) result.console.push({ type: m.type(), text: m.text() }); });
  page.on('pageerror', e => result.pageErrors.push(String(e)));
  page.on('requestfailed', r => result.requests_failed.push({ url: r.url(), failure: r.failure()?.errorText }));
  await page.goto(`${origin}/`, { waitUntil: 'domcontentloaded', timeout: 60000 });
  try {
    await page.waitForFunction(() => {
      const loader = document.getElementById('world-loader');
      if (loader?.classList.contains('has-error')) {
        throw new Error(`loader failed: ${document.getElementById('loader-stage')?.textContent} — ${document.getElementById('loader-message')?.textContent}`);
      }
      return loader?.classList.contains('is-dismissed');
    }, null, { timeout: 240000 });
    result.ready = true;
  } catch (error) {
    result.error = error.stack || String(error);
  }
  result.loaderStage = await page.locator('#loader-stage').innerText().catch(() => null);
  result.loaderMessage = await page.locator('#loader-message').innerText().catch(() => null);
  await page.waitForTimeout(4000);
  result.canvas = await page.evaluate(() => {
    const canvas = document.querySelector('canvas');
    if (!canvas) return null;
    const rect = canvas.getBoundingClientRect();
    let nonBlank = null;
    try {
      const ctx = document.createElement('canvas').getContext('2d');
      ctx.canvas.width = 64; ctx.canvas.height = 64;
      ctx.drawImage(canvas, 0, 0, 64, 64);
      const data = ctx.getImageData(0, 0, 64, 64).data;
      nonBlank = [...data].some(v => v !== 0);
    } catch (e) { nonBlank = `unreadable (${e.message})`; }
    return { id: canvas.id, width: canvas.width, height: canvas.height, css: `${rect.width}x${rect.height}`, nonBlank };
  });
  await page.screenshot({ path: path.join(outDir, 'game-ready.png'), fullPage: false });
  result.url_final = page.url();
} finally {
  await writeFile(path.join(outDir, 'result.json'), JSON.stringify(result, null, 2) + '\n');
  console.log(JSON.stringify({ ready: result.ready, loaderStage: result.loaderStage, loaderMessage: result.loaderMessage,
    error: result.error, pageErrors: result.pageErrors, requests_failed: result.requests_failed.slice(0, 10),
    canvas: result.canvas, url_final: result.url_final }, null, 2));
  await browser.close();
  if (!result.ready) process.exitCode = 1;
}
