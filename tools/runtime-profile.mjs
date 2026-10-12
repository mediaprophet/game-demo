import {writeFile} from 'node:fs/promises';
const targets = await (await fetch('http://127.0.0.1:9233/json/list')).json();
const target = targets.find(t=>t.type==='page'&&t.url.startsWith('https://mediaprophet.github.io/game-demo/game.html'));
if(!target)throw new Error('Published test target missing');
const socket = new WebSocket(target.webSocketDebuggerUrl);
await new Promise((yes,no)=>{socket.onopen=yes;socket.onerror=no});
let next=0;const pending=new Map();
socket.onmessage=e=>{const value=JSON.parse(e.data);const p=pending.get(value.id);if(p){clearTimeout(p.timer);pending.delete(value.id);value.error?p.no(new Error(JSON.stringify(value.error))):p.yes(value.result)}};
const call=(method,params={})=>new Promise((yes,no)=>{const id=++next;const timer=setTimeout(()=>{pending.delete(id);no(new Error('Timeout '+method))},120000);pending.set(id,{yes,no,timer});socket.send(JSON.stringify({id,method,params}))});
try {
 await call('Profiler.enable');await call('Profiler.setSamplingInterval',{interval:1000});await call('Profiler.start');
 await new Promise(yes=>setTimeout(yes,6000));
 const raw=await call('Profiler.stop');
 const nodes=new Map(raw.profile.nodes.map(n=>[n.id,n]));const sums=new Map();
 for(let i=0;i<(raw.profile.samples||[]).length;i++){const id=raw.profile.samples[i];sums.set(id,(sums.get(id)||0)+(raw.profile.timeDeltas?.[i]||0))}
 const total=[...sums.values()].reduce((a,b)=>a+b,0);
 const top=[...sums].sort((a,b)=>b[1]-a[1]).slice(0,30).map(([id,micros])=>({id,micros,sampledTimeShare:total?micros/total:null,callFrame:nodes.get(id).callFrame}));
 const response=await call('Runtime.evaluate',{expression:`(async()=>{const source=performance.getEntriesByType('resource').find(r=>r.name.includes('/pkg/rolling_commons_shell.js'))?.name;const m=source?await import(source):null;return {revision:m?.pinned_qualiadb_revision?.(),debug:window.__communityGroundDebug?.(),source}})()`,awaitPromise:true,returnByValue:true});
 const result={method:'Exclusive sampled time: sum each timeDeltas[i] by samples[i] leaf node; not instrumented function wall time or GPU time',profileWindowMicros:raw.profile.endTime-raw.profile.startTime,totalSampleMicros:total,top,context:response.result.value};
 const data=JSON.stringify({raw,...result},null,2);if(Buffer.byteLength(data)>8*1024*1024)throw new Error('Profile evidence byte budget exceeded');
 await writeFile('docs/qa/graphics-overhaul/runtime-2026-10-12/published-map-profile.json',data,{flag:'wx'});
 console.log(JSON.stringify(result));
}finally{socket.close()}
