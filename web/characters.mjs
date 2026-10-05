// Game-owned private character graph. Qualia OPFS stores this N3 document;
// it is separate from the shared campaign replay tape.
export const CHARACTER_NAMES = [
  'Ada', 'Mira', 'Orin', 'Tavi', 'Leni', 'Rowan', 'Sol', 'Nia',
  'Kit', 'Ari', 'Eden', 'Jules',
];
export const CHARACTER_BIOS = {
  Ada:'Keeps the tools her mother left behind, and fixes what the town has given up on.',
  Mira:'Carries seeds gathered from every garden she has helped bring back to life.',
  Orin:'A ridge guide who remembers the old footpaths and the people who used them.',
  Tavi:'A curious apprentice, always asking who gets left out of a good plan.',
  Leni:'A patient cook who knows that a shared meal can start a difficult conversation.',
  Rowan:'A creek watcher learning how small changes upstream reshape the whole valley.',
  Sol:'A tinkerer who would rather teach a repair than keep a clever trick secret.',
  Nia:'A market runner with a talent for finding the neighbour who has what is needed.',
  Kit:'A young mapmaker trying to connect places that stopped speaking to one another.',
  Ari:'A careful listener collecting the stories behind the town’s hardest choices.',
  Eden:'A seed keeper determined to make the next harvest belong to everyone.',
  Jules:'A builder who believes a safe crossing should bring communities closer.',
};
export const characterBio = c => CHARACTER_BIOS[c.name] || 'A new neighbour, ready to leave a mark on the valley.';
export const CHARACTER_ARCS = [
  {id:'maker', label:'The Maker', opening:'A silent workshop can still become the town’s heart.'},
  {id:'grower', label:'The Grower', opening:'The first harvest begins with water and a place to plant.'},
  {id:'crossing', label:'The Crossing', opening:'Two settlements need a safe way to meet again.'},
  {id:'steward', label:'The Steward', opening:'The valley has stories worth carrying into the commons.'},
];
export const CHARACTER_TRAITS = [
  {id:'observant', label:'Observant', detail:'Spots nearby sites from farther away.'},
  {id:'trailwise', label:'Trailwise', detail:'Walks a little faster on field paths.'},
  {id:'connector', label:'Connector', detail:'Can approach a site from a little farther away.'},
  {id:'steadfast', label:'Steadfast', detail:'Runs a little faster when the route is clear.'},
];
export const CHARACTER_PLACES = [
  {id:'kestrel', label:'Kestrel Flats', spawn:[-5.6,5.6], yaw:-0.8, pitch:0.18},
  {id:'saltwind', label:'Saltwind Reach', spawn:[11.0,-5.2], yaw:0.15, pitch:0.18},
  {id:'highlands', label:'Northern Highlands', spawn:[0.0,-18.0], yaw:3.14, pitch:0.12},
];
const ids = xs => new Set(xs.map(x=>x.id));
const ARCS=ids(CHARACTER_ARCS), TRAITS=ids(CHARACTER_TRAITS), PLACES=ids(CHARACTER_PLACES);
const round = n => Math.round(n*1000)/1000;

export function seedCharacters(){
  return {selected:1, nextId:4, characters:[
    {id:1,name:'Ada',arc:'maker',trait:'steadfast',home:'kestrel',place:'kestrel',x:-5.6,z:5.6},
    {id:2,name:'Mira',arc:'grower',trait:'observant',home:'saltwind',place:'saltwind',x:11.0,z:-5.2},
    {id:3,name:'Orin',arc:'steward',trait:'trailwise',home:'highlands',place:'highlands',x:0.0,z:-18.0},
  ]};
}
export const activeCharacter = roster => roster.characters.find(c=>c.id===roster.selected);
export function selectCharacter(roster,id){
  if(!roster.characters.some(c=>c.id===id))throw Error('Unknown character');
  roster.selected=id;
  return activeCharacter(roster);
}
export function createCharacter(roster,{name,arc,trait,home}){
  if(roster.characters.length>=12)throw Error('The local cast is full (12 characters).');
  if(!CHARACTER_NAMES.includes(name)||!ARCS.has(arc)||!TRAITS.has(trait)||!PLACES.has(home))
    throw Error('Choose a listed name, story, field trait and starting place.');
  const place=CHARACTER_PLACES.find(p=>p.id===home);
  const c={id:roster.nextId++,name,arc,trait,home,place:home,
    x:place.spawn[0],z:place.spawn[1]};
  roster.characters.push(c);roster.selected=c.id;
  return c;
}
export function moveCharacter(c,x,z,place=c.place){
  if(!Number.isFinite(x)||!Number.isFinite(z)||!PLACES.has(place))return;
  c.x=round(Math.max(-6.6,Math.min(22.6,x)));
  c.z=round(Math.max(-20.6,Math.min(6.6,z)));
  c.place=place;
}
export function nearbyRadius(c){return c.trait==='observant'?7:c.trait==='connector'?5:3.6;}
export function walkSpeed(c,running){
  const base=running?5:2.6;
  return base*(c.trait==='trailwise'&&!running?1.25:
    c.trait==='steadfast'&&running?1.15:1);
}
export function characterChapter(c,world){
  const stage={
    maker: world.online ?
      world.signalOnline?['A workshop that reaches farther','The signal is live. Take a paid repair order and pass those skills on.']:
        ['Power with a purpose','The workshop runs. Reconnect the mast so distant neighbours can call for repairs.']:
      world.approved?['The final connection','The town endorsed the solar project. Bring the workshop online.']:
        ['A workshop waiting','Find the missing parts and earn the council’s endorsement.'],
    grower: world.orchardActive?['A harvest to share','Choose how Saltwind’s harvest should support the commons.']:
      world.gardenActive?['From one garden to two','Help Saltwind reopen water and plant its orchard.']:
      world.waterOnline?['The first beds','Shared water flows. Plant Kestrel’s garden.']:
        ['Water before seeds','Repair the shared cistern so anyone can grow food.'],
    crossing: world.pumpOnline?['Beyond the bridge','The wind pump works. Help the orchard and the first trade route.']:
      world.bridgeOpen?['Water on the far bank','The crossing is open. Bring Saltwind’s wind pump online.']:
        ['The divided canal','Restore or brace the bridge before high water makes the work harder.'],
    steward: world.finale?['A living record','The commons carries the choices you helped make.']:
      world.signalOnline?['The wider circle','Share the town’s progress with neighbours through the mast.']:
      world.gardenActive?['What grows here','Walk the creek valley and help the town keep its new garden alive.']:
        ['Listen to the valley','Explore the ridge, then help restore shelter, water and food.'],
  }[c.arc];
  return {title:stage[0],goal:stage[1],opening:CHARACTER_ARCS.find(a=>a.id===c.arc).opening};
}

// Strict line form: known option IDs and quoted listed names only. This keeps
// the graph private, deterministic and safe to load without arbitrary scripts.
export function encodeCharacters(roster){
  const lines=[
    '@prefix pc: <https://game.example/pc#> .',
    'pc:roster pc:version "1" .',
    `pc:roster pc:selected pc:c${roster.selected} .`,
  ];
  for(const c of roster.characters){
    if(!CHARACTER_NAMES.includes(c.name)||!ARCS.has(c.arc)||!TRAITS.has(c.trait)||
       !PLACES.has(c.home)||!PLACES.has(c.place)||!Number.isFinite(c.x)||!Number.isFinite(c.z))
      throw Error('Invalid character profile');
    const s=`pc:c${c.id}`;
    lines.push(`${s} a pc:PlayableCharacter .`,`${s} pc:name "${c.name}" .`,
      `${s} pc:arc pc:${c.arc} .`,`${s} pc:trait pc:${c.trait} .`,
      `${s} pc:home pc:${c.home} .`,`${s} pc:place pc:${c.place} .`,
      `${s} pc:x "${round(c.x).toFixed(3)}" .`,`${s} pc:z "${round(c.z).toFixed(3)}" .`);
  }
  return lines.join('\n')+'\n';
}
export function decodeCharacters(source){
  const lines=source.split(/\r?\n/).map(x=>x.trim()).filter(Boolean);
  if(lines[0]!=='@prefix pc: <https://game.example/pc#> .'||
     lines[1]!=='pc:roster pc:version "1" .')throw Error('Unsupported character graph');
  const selectedMatch=/^pc:roster pc:selected pc:c(\d+) \.$/.exec(lines[2]);
  if(!selectedMatch)throw Error('Missing selected character');
  const selected=Number(selectedMatch[1]);
  const records=new Map();
  for(const line of lines.slice(3)){
    const m=/^pc:c(\d+) (a|pc:[a-z]+) (pc:[A-Za-z]+|"[A-Za-z0-9 .-]+") \.$/.exec(line);
    if(!m)throw Error('Malformed character graph');
    const id=Number(m[1]), key=m[2], value=m[3];
    if(!Number.isSafeInteger(id)||id<1||id>9999)throw Error('Invalid character ID');
    const record=records.get(id)||{id};
    if(key in record)throw Error('Duplicate character fact');
    record[key]=value.startsWith('"')?value.slice(1,-1):value.slice(3);
    records.set(id,record);
  }
  if(!records.size||records.size>12)throw Error('Invalid cast size');
  const characters=[...records.values()].sort((a,b)=>a.id-b.id).map(r=>{
    if(r.a!=='PlayableCharacter')throw Error('Invalid character type');
    const c={id:r.id,name:r['pc:name'],arc:r['pc:arc'],trait:r['pc:trait'],
      home:r['pc:home'],place:r['pc:place'],x:Number(r['pc:x']),z:Number(r['pc:z'])};
    if(!CHARACTER_NAMES.includes(c.name)||!ARCS.has(c.arc)||!TRAITS.has(c.trait)||
       !PLACES.has(c.home)||!PLACES.has(c.place)||!Number.isFinite(c.x)||!Number.isFinite(c.z)||
       c.x < -6.6||c.x>22.6||c.z < -20.6||c.z>6.6)throw Error('Invalid character facts');
    return c;
  });
  if(!characters.some(c=>c.id===selected))throw Error('Selected character is absent');
  return {characters,selected,nextId:Math.max(...characters.map(c=>c.id))+1};
}
