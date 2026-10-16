import { createRequire } from 'node:module';
import fs from 'node:fs';
import path from 'node:path';
const require = createRequire(import.meta.url);
const playwrightPath = process.env.PLAYWRIGHT_CORE || 'C:\\Users\\tim_h\\.cache\\codex-runtimes\\codex-primary-runtime\\dependencies\\node\\node_modules\\playwright-core';
if (!fs.existsSync(playwrightPath)) throw Error('Set PLAYWRIGHT_CORE to an installed playwright-core package.');
const { chromium } = require(playwrightPath);
const browser = await chromium.launch({channel:process.env.BROWSER_CHANNEL||'chrome',headless:true,args:['--enable-unsafe-webgpu','--enable-features=Vulkan,WebGPU']});
const page = await browser.newPage({viewport:{width:1440,height:900}});
await page.route('https://talbingo.invalid/**',async route=>{
  const pathname=new URL(route.request().url()).pathname;
  const file=path.resolve('web',`.${pathname}`);
  if(!file.startsWith(path.resolve('web')+path.sep)||!fs.existsSync(file))return route.fulfill({status:404,body:'not found'});
  const ext=path.extname(file);const contentType=ext==='.html'?'text/html':ext==='.js'?'application/javascript':ext==='.wasm'?'application/wasm':ext==='.vibe'?'text/plain':'application/octet-stream';
  return route.fulfill({status:200,contentType,body:fs.readFileSync(file)});
});
page.on('console', msg=>{if(msg.type()==='error')console.log('CONSOLE',msg.text())});
page.on('pageerror', error=>console.log('PAGE ERROR',error.message));
await page.goto('https://talbingo.invalid/talbingo.html');
await page.waitForFunction(()=>document.querySelector('#status').textContent.startsWith('Ready.'),null,{timeout:120000}).catch(async error=>{console.log('LOAD STATUS',await page.locator('#status').textContent());throw error});
console.log('RENDER TIER',await page.evaluate(()=>window.__talbingoTier));
console.log('SCENE RECEIPT',await page.evaluate(()=>window.__sceneReceipt));
const roadCheck=await page.evaluate(async()=>{
  const m=await import('./pkg/rolling_commons_shell.js');
  const source=await(await fetch('./fixtures/talbingo-drive.vibe')).text();
  const sim=new m.GameDriveSimulation(source);
  const unwrap=v=>v instanceof Map?Object.fromEntries([...v].map(([k,x])=>[k,unwrap(x)])):v;
  const mismatches=[];
  for(let site=0;site<6;site++)for(const x of [-5.45,0,8,16,21.65]){
    const start=unwrap(sim.setup(0));
    const state=unwrap(sim.step(JSON.stringify({...start,x,z:m.talbingo_road_z(site,x),dt:0,site_id:site,rig_id:0,throttle:false,reverse:false,brake:false,steer:0,terrain_height:0,terrain_height_ahead:0,terrain_height_behind:0,terrain_sample_distance:.55,crossing_open:true,hazards:[],period:'day'})));
    const expected=m.talbingo_road_z(site,x);
    if(state.error||Math.abs(state.road_center_z-expected)>.002)mismatches.push({site,x,expected,actual:state.road_center_z,error:state.error});
  }
  return mismatches;
});
if(roadCheck.length)throw Error('Visual road and driving policy disagree: '+JSON.stringify(roadCheck.slice(0,3)));
console.log('ROAD ALIGNMENT 30 samples match');
const routeCheck=await page.evaluate(async()=>{
  const m=await import('./pkg/rolling_commons_shell.js');
  const source=await(await fetch('./fixtures/talbingo-drive.vibe')).text();
  const sim=new m.GameDriveSimulation(source);
  const unwrap=v=>v instanceof Map?Object.fromEntries([...v].map(([k,x])=>[k,unwrap(x)])):v;
  const failures=[];
  for(let site=0;site<6;site++)for(const outbound of [true,false]){
    let state=unwrap(sim.setup(0));
    if(!outbound)state={...state,x:21.65,z:m.talbingo_road_z(site,21.65),heading:Math.PI/2,trailer_heading:Math.PI/2};
    let reached=false;
    for(let tick=0;tick<750;tick++){
      const next=unwrap(sim.step(JSON.stringify({...state,dt:.05,site_id:site,rig_id:0,throttle:true,reverse:false,brake:false,steer:0,terrain_height:0,terrain_height_ahead:0,terrain_height_behind:0,terrain_sample_distance:.55,crossing_open:true,hazards:[],period:'day'})));
      if(next.error){failures.push({site,outbound,error:next.error});break}
      state={...state,...next};
      if(outbound?state.x>=16.9:state.x<=-5.4){reached=true;break}
    }
    if(!reached)failures.push({site,outbound,x:state.x,z:state.z,warning:state.warning});
  }
  return failures;
});
if(routeCheck.length)throw Error('Route traversal failed: '+JSON.stringify(routeCheck));
console.log('ROUTES 6 outbound and 6 return traversals pass');
const campCheck=await page.evaluate(async()=>{
  const m=await import('./pkg/rolling_commons_shell.js');
  const source=await(await fetch('./fixtures/talbingo-drive.vibe')).text();
  const sim=new m.GameDriveSimulation(source);
  const targets=[[20,-3.5],[14.7,-4],[19,-3.5],[20.3,-2.7],[21.5,-5.1],[19,-3.4]];
  const unwrap=v=>v instanceof Map?Object.fromEntries([...v].map(([k,x])=>[k,unwrap(x)])):v;
  const failures=[];
  for(let site=0;site<6;site++){
    let walker={x:18,z:3.5};const [tx,tz]=targets[site];
    for(let tick=0;tick<260;tick++){
      const east=Math.abs(tx-walker.x)<.1?0:Math.sign(tx-walker.x);
      const south=Math.abs(tz-walker.z)<.1?0:Math.sign(tz-walker.z);
      walker=unwrap(sim.camp_step(JSON.stringify({...walker,site_id:site,dt:.05,east,south})));
      if(walker.error){failures.push({site,error:walker.error});break}
      if(Math.hypot(walker.x-tx,walker.z-tz)<2.2)break;
    }
    if(Math.hypot(walker.x-tx,walker.z-tz)>=2.2)failures.push({site,walker});
  }
  return failures;
});
if(campCheck.length)throw Error('Camp landmarks unreachable: '+JSON.stringify(campCheck));
console.log('CAMP WALK 6 landmarks reachable');
for(let i=0;i<6;i++){
  await page.selectOption('#site',String(i));
  await page.waitForFunction(()=>document.querySelector('#status').textContent.startsWith('Ready.'),null,{timeout:120000}).catch(async error=>{console.log('SITE',i,'STATUS',await page.locator('#status').textContent());throw error});
  const name=await page.locator('#site option:checked').textContent();
  console.log('SCENE',i,name,await page.locator('#mission').textContent(),await page.evaluate(()=>window.__sceneReceipt?.material_pipeline));
  await page.screenshot({path:`docs/qa/talbingo-site-${i}.png`,fullPage:true});
}
await page.selectOption('#site','0');
await page.locator('#start').click();
await page.keyboard.down('w');
await page.waitForTimeout(1800);
await page.screenshot({path:'docs/qa/talbingo-drive.png',fullPage:true});
await page.waitForFunction(()=>document.querySelector('#status').textContent.startsWith('Arrived.'),{timeout:30000});
await page.keyboard.up('w');
if(await page.locator('#choices button[data-correct="true"]').isEnabled())throw Error('Camp choice available before inspecting landmark');
await page.keyboard.down('w');
await page.waitForFunction(()=>!document.querySelector('#inspect').disabled,{timeout:12000});
await page.keyboard.up('w');
await page.screenshot({path:'docs/qa/talbingo-walk.png',fullPage:true});
await page.locator('#inspect').click();
await page.locator('#choices button[data-correct="false"]').click();
if(!await page.locator('#status').textContent().then(s=>s.startsWith('Try another')))throw Error('Incorrect choice accepted');
await page.locator('#choices button[data-correct="true"]').click();
await page.locator('#return').click();
await page.keyboard.down('w');
await page.waitForFunction(()=>document.querySelector('#status').textContent.startsWith('Round trip complete'),{timeout:30000});
await page.keyboard.up('w');
console.log('ROUND TRIP',await page.locator('#status').textContent());
await page.screenshot({path:'docs/qa/talbingo-prototype.png',fullPage:true});
await page.setViewportSize({width:390,height:844});
await page.reload();
await page.waitForFunction(()=>document.querySelector('#status').textContent.startsWith('Ready.'),{timeout:120000});
if(!await page.locator('.play-touch').isVisible())throw Error('Mobile play controls missing');
await page.screenshot({path:'docs/qa/talbingo-mobile.png',fullPage:true});
await browser.close();
