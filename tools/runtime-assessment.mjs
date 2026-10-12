import {mkdir, writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';

const origin = 'http://127.0.0.1:9233';
const out = resolve('docs/qa/graphics-overhaul/runtime-2026-10-12');
await mkdir(out, {recursive: false});
let written = 0;
async function artifact(name, value) {
  const bytes = Buffer.isBuffer(value) ? value : Buffer.from(JSON.stringify(value, null, 2));
  if (written + bytes.length > 32 * 1024 * 1024) throw new Error('Evidence budget exceeded');
  await writeFile(resolve(out, name), bytes, {flag: 'wx'});
  written += bytes.length;
}
async function connect(url) {
  const socket = new WebSocket(url);
  await new Promise((yes, no) => {socket.onopen = yes; socket.onerror = no;});
  let id = 0;
  const pending = new Map();
  const events = [];
  socket.onmessage = event => {
    const message = JSON.parse(event.data);
    if (message.id) {
      const entry = pending.get(message.id);
      if (!entry) return;
      clearTimeout(entry.timer); pending.delete(message.id);
      if (message.error) entry.no(new Error(JSON.stringify(message.error))); else entry.yes(message.result);
    } else if (events.length < 10000) events.push(message);
  };
  return {events, close: () => socket.close(), call(method, params = {}) {
    return new Promise((yes, no) => {
      const next = ++id;
      const timer = setTimeout(() => {pending.delete(next); no(new Error('CDP timeout: ' + method));}, 120000);
      pending.set(next, {yes, no, timer}); socket.send(JSON.stringify({id: next, method, params}));
    });
  }};
}
const pause = ms => new Promise(yes => setTimeout(yes, ms));
const version = await (await fetch(origin + '/json/version')).json();
const browser = await connect(version.webSocketDebuggerUrl);
await artifact('browser.json', {version, gpu: await browser.call('SystemInfo.getInfo')});
const snapshot = `({url:location.href,readyState:document.readyState,visibility:document.visibilityState,dpr:devicePixelRatio,viewport:[innerWidth,innerHeight],hardwareConcurrency:navigator.hardwareConcurrency,crossOriginIsolated,loader:{className:document.getElementById('world-loader')?.className,stage:document.getElementById('loader-stage')?.textContent,message:document.getElementById('loader-message')?.textContent,busy:document.getElementById('world-loader')?.getAttribute('aria-busy')},quality:document.getElementById('quality-profile')?.value,pin:document.getElementById('pin')?.textContent,tier:document.getElementById('tier')?.textContent,debug:window.__communityGroundDebug?.(),journal:document.getElementById('log')?.innerText,gpuErrors:window.__webGpuErrors||[],pipelineErrors:window.__pipelineErrors||[],shaderErrors:window.__shaderErrors||[],resources:performance.getEntriesByType('resource').map(r=>({name:r.name,startTime:r.startTime,duration:r.duration,transferSize:r.transferSize,encodedBodySize:r.encodedBodySize,decodedBodySize:r.decodedBodySize})),navigation:performance.getEntriesByType('navigation').map(n=>n.toJSON())})`;
const sample = `(async()=>{const values=[];let last,done=false;await new Promise(resolve=>{const timer=setTimeout(()=>{done=true;resolve()},45000);const step=t=>{if(done)return;if(last!==undefined)values.push(t-last);last=t;if(values.length>=120){done=true;clearTimeout(timer);resolve()}else requestAnimationFrame(step)};requestAnimationFrame(step)});return {intervalsMs:values,timedOut:values.length<120,visibility:document.visibilityState,debug:window.__communityGroundDebug?.()}})()`;
const summaries = [];
try {
  for (const [name, url] of [['published', 'https://mediaprophet.github.io/game-demo/game.html?renderer-debug'], ['local', 'http://127.0.0.1:8089/game.html?renderer-debug']]) {
    const target = await (await fetch(origin + '/json/new?about:blank', {method: 'PUT'})).json();
    const client = await connect(target.webSocketDebuggerUrl);
    const evaluate = async expression => {
      const response = await client.call('Runtime.evaluate', {expression, awaitPromise: true, returnByValue: true});
      if (response.exceptionDetails) throw new Error(JSON.stringify(response.exceptionDetails));
      return response.result.value;
    };
    try {
      await client.call('Runtime.enable'); await client.call('Log.enable'); await client.call('Network.enable'); await client.call('Page.enable');
      await client.call('Emulation.setDeviceMetricsOverride', {width:1280,height:800,deviceScaleFactor:1,mobile:false});
      await client.call('Page.addScriptToEvaluateOnNewDocument', {source:`window.__runtimeLoadEvents=[];new MutationObserver(()=>{if(window.__runtimeLoadEvents.length<2000)window.__runtimeLoadEvents.push({time:performance.now(),stage:document.getElementById('loader-stage')?.textContent,message:document.getElementById('loader-message')?.textContent,className:document.getElementById('world-loader')?.className})}).observe(document,{subtree:true,childList:true,attributes:true,characterData:true});`});
      const started = Date.now();
      await client.call('Page.navigate', {url});
      let current;
      for (let attempt=0;attempt<45;attempt++) {
        await pause(2000); current = await evaluate(snapshot);
        if (current.loader.className?.includes('is-dismissed') || current.loader.className?.includes('has-error')) break;
      }
      await artifact(name + '-startup.json', {wallElapsedMs:Date.now()-started,snapshot:current,loadEvents:await evaluate('window.__runtimeLoadEvents')});
      await artifact(name + '-startup.png', Buffer.from((await client.call('Page.captureScreenshot', {format:'png'})).data,'base64'));
      const samples = [];
      if (current?.loader.className?.includes('is-dismissed')) {
        for (const [label, action] of [['default',null],['balanced',`document.getElementById('quality-profile').value='1';document.getElementById('quality-profile').dispatchEvent(new Event('change'));`],['high',`document.getElementById('quality-profile').value='2';document.getElementById('quality-profile').dispatchEvent(new Event('change'));`],['walk-high',`document.querySelector('[data-game-view="walk"]').click();`],['drive-high',`document.querySelector('[data-game-view="drive"]').click();`]]) {
          if(action) await evaluate(action);
          await pause(1500);
          const raw = await evaluate(sample);
          await artifact(name + '-' + label + '-raf.json', raw);
          await artifact(name + '-' + label + '.png', Buffer.from((await client.call('Page.captureScreenshot',{format:'png'})).data,'base64'));
          const sorted = [...raw.intervalsMs].sort((a,b)=>a-b);
          const sum = raw.intervalsMs.reduce((a,b)=>a+b,0);
          const quantile = fraction => sorted.length ? sorted[Math.ceil(fraction*sorted.length)-1] : null;
          samples.push({label,count:sorted.length,timedOut:raw.timedOut,visibility:raw.visibility,meanMs:sorted.length?sum/sorted.length:null,rafRate:sum>0?1000*sorted.length/sum:null,p50Ms:quantile(.5),p95Ms:quantile(.95),p99Ms:quantile(.99)});
        }
        await client.call('Profiler.enable'); await client.call('Profiler.setSamplingInterval',{interval:1000}); await client.call('Profiler.start'); await pause(6000);
        await artifact(name + '-drive-profile.json', await client.call('Profiler.stop'));
        await evaluate(`document.querySelector('[data-game-view="world"]').click();`);
        await evaluate(`document.getElementById('art-style').value='1';document.getElementById('art-style').dispatchEvent(new Event('change'));document.getElementById('art-style').value='0';document.getElementById('art-style').dispatchEvent(new Event('change'));`);
        await pause(15000);
        await artifact(name + '-style-switch.json',{snapshot:await evaluate(snapshot),loadEvents:await evaluate('window.__runtimeLoadEvents')});
        await artifact(name + '-style-switch.png',Buffer.from((await client.call('Page.captureScreenshot',{format:'png'})).data,'base64'));
      }
      await artifact(name + '-final.json',await evaluate(snapshot));
      await artifact(name + '-events.json',client.events);
      summaries.push({name,loader:current?.loader,samples});
      console.log(JSON.stringify(summaries.at(-1)));
    } catch(error) {
      await artifact(name + '-capture-error.json',{error:String(error),events:client.events});
      console.error(name+': '+String(error));
    } finally {
      client.close(); await browser.call('Target.closeTarget',{targetId:target.id});
    }
  }
  await artifact('summary.json',summaries);
  console.log('Evidence directory: '+out+'; bytes: '+written);
} finally { browser.close(); }
