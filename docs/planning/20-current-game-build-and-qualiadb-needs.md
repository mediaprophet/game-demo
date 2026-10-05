# Current Game Build and QualiaDB Needs

Status: living implementation note, 2026-10-04. The owner authorized one
generic upstream QualiaDB update for the in-viewport HUD. Other generic
engine gaps remain work orders; game-specific replacements are not acceptable.

## Game work in this pass

**Highlands and camera pass (2026-10-05):** Two deterministic game-authored
heightfield patches add ridges above a creek valley north of the settlements.
Qualia's geospatial DEM builder triangulates them, and the normal `.10d`/HMC
path packages the 12 new terrain and scenery assets. A third Highlands
destination and Map, Survey and Walk presets live in the Qualia in-game HUD.
Walk uses a ground-height sample from the same WASM terrain function and
supports WASD movement, Shift running and drag-to-look; its immersive HUD
leaves the scenery visible at compact widths. The HMC now has 274 variants
across ten scene states and the opening scene has 234 organs. Browser views of
the highland overview, survey and ground camera were inspected, and all 61
scripted campaign checks passed. The current terrain is a finite four-tile
world, with no streaming or gameplay pathing on the new hills. QualiaDB was
not changed.

**Botanical graphics pass (2026-10-05):** The four Kestrel corner trees now
use Qualia-authored leaning pale trunks, forks, flatter blue-green canopy
lobes and pendant foliage. Saltwind adds two gums. Wattle flowers, a herb
border, leafy vegetables with produce in planted states, and a stemmed
lavender bed add distinct low planting. The game-owned HMC contains 262
`.10d` variants across ten states; 222 scene parts appear in the opening
scene. Both territory views were inspected in the full Qualia WASM browser,
including a closer camera view, and all 60 campaign checks passed. This is a
stylized botanical blockout; realistic foliage surfaces, animation and LOD
remain open generic engine and game-art work. QualiaDB was not changed.

**Landscape and prop graphics pass (2026-10-05):** Both territories now use
layered ground edges and rounded meadow forms, with new shrub and planting
families. Saltwind gains canal reeds, ripples and stones, orchard furrows and
crop rows, lavender and windbreaks. Building and trade landmarks gain clearer
awnings, banners, produce, hay, door accents and boat/pump details. These are
game-owned Qualia geometry recipes compiled to `.10d`; the game-owned HMC now
contains 214 distinct variants across ten states, with 181 visible organs in
the opening scene. The full Qualia WASM browser build passed 60 scripted
checks with no runtime errors. The projectors still render bright squares and
the geometry remains blockout quality; the generic renderer work below is
needed for finished surfaces. QualiaDB was not changed in this pass.

**Orchard harvest choice (2026-10-05):** After planting, the player may spend
one crew shift to reserve the first crop for a lower-cost commons gathering
(two coins rather than four), or sell it for three coins. The two choices are
mutually exclusive and optional; the original endings remain finishable.
Qualia SHACL gates the choice and the reserved-food ending, and the event tape
replays either route exactly. The orchard changes from fruit beds to packed
produce crates through game-owned recipes compiled by full Qualia WASM. The
game HMC now carries 162 distinct `.10d` variants across ten scene states.
The local browser passed 60 scripted checks, including the new routes, visual
state receipt, HMC integrity, and replay. QualiaDB was not changed.

**Communications and contract pass (2026-10-04):** The Kestrel mast is now a
playable optional project. Once workshop power is online, a player can spend
one work shift or three coins to connect it. A connected mast unlocks a single
remote repair order that pays four coins toward either finale. Qualia's SHACL
path validates both routes and the payout, with deterministic replay. The
commissioned mast adds a rounded signal head compiled through Qualia geometry
into the game-owned `.10d`/HMC package. Saved content version 4 worlds acquire
an offline mast on load and continue as version 5. The full WASM browser build
passed 55 scripted checks; the game asset export contains 160 distinct `.10d`
variants across nine scene states. QualiaDB was not changed in this pass. The
generic HUD still needs a communications icon; see the HUD work order in
[the interface note](21-ingame-interface-and-asset-quality.md).

**In-game HUD and visible WebGPU pass (2026-10-04):** The upstream QualiaDB
renderer fixes through `0006a07d` resolved the black WebGPU viewport reported
below. A fresh full-engine WASM
build displays both territories at Portal tier 2. The game now puts visible
player controls on a generic Qualia canvas HUD instead of visible HTML panels;
the browser page still contains an invisible legacy data view pending removal.
The game added distinct resident body parts, workshop and hall silhouettes,
bridge states, pump blades, and boat rails through Qualia geometry and `.10d`.
The final browser self-test passed all 47 checks, including the Qualia `SRD1`
surface reading, and live HUD order, rejection, and
VibeScript interactions worked. Semantic tensor nodes remain visually projected
as bright square markers even after the ambient field is disabled; Qualia needs
a generic independent projector-visibility control that preserves picking.
See [the HUD and art pass](21-ingame-interface-and-asset-quality.md).

**Canal tide scenario pass (2026-10-04):** Day four is the last day for the
simple bridge repair. Starting on day five, a player can brace the supports
with one work shift and finish on another, or pay two coins and coordinate a
specialist crew to restore the crossing immediately. Both late routes remain
recoverable and pass Qualia SHACL validation and deterministic replay. The
field dispatch and Saltwind objective state now communicate the time window,
and Qualia-authored `.10d` states distinguish low/high water and bridge
bracing. Save content version is 4; earlier N3 saves require an explicit
Qualia Q42 migration. The rebuilt browser package passed 46 campaign checks.
In a manual day-five route, bracing increased the accepted mesh count from
116 to 117; finishing returned it to 116. OPFS reload preserved the day-six
crossing and replay reproduced the world byte-for-byte. At that point the full renderer
visibility gate QG-12 remained open: mesh receipt changes were not proof of
visible pixels. Concurrent uncommitted renderer work was present in the
QualiaDB checkout after this package build; it was not changed or claimed
as verified by this game pass.

**Gameplay-first campaign pass (2026-10-03):** The two territories now form
one finishable scenario rather than an open-ended list of technology actions.
The player manages twelve starting coins and one work action per day, chooses
paid or salvaged repairs, unlocks a single paid workshop order, and uses the
restored Kestrel infrastructure to reach Saltwind. The last decision is either
a local harvest commons (shelter, garden, orchard, four coins, labour) or a
canal trade route (bridge, pump, orchard, two coins, labour). These produce
distinct graph outcomes and replay tapes. Duplicate solar-part acquisition,
committee endorsement, workshop completion, order payout and finale actions
are SHACL-gated. The page now leads with a field dispatch, campaign progress,
relevant orders, day count, blocked-order help and a result screen. The
VibeScript world projection includes the day, paid order and final outcome.
The rules still pass through Qualia's SHACL path; the DOM guidance is only
presentation. Browser tests passed both endings (43 checks), then a manual
trade-route playthrough, OPFS reload and byte-for-byte replay. The WebGPU
viewport was black at that point, so this was a playable **rules/UI** level whose 3D
visual acceptance awaited QG-12. The 2026-10-04 HUD pass above supersedes that status.

Kestrel Flats now has two connected, rule-gated infrastructure objectives:
restore the workshop's solar power, then repair the shared water tank and
plant the garden. The tank has coin and labour repair routes. Each command
still goes through the existing Qualia-backed `GameSession` validation and
event path. The tank/garden states change existing original `.10d` town
blockouts and have semantic pick targets. The interface is organised as a
strategy view with scene, resources, objectives, orders, an event journal,
and an optional VibeScript workbench. This is a playable slice, not yet a
continuous multi-agent RTS.

Browser verification on 2026-10-03: the scripted solar/shelter/water/garden
playthrough and replay pass. The paid tank route also succeeds from a fresh
start; a second repair is blocked; garden planting updates the objectives and
resources. Saving to OPFS and loading after a page reload preserved the paid
repair and planted garden. The Rust `wasm32-unknown-unknown` check and release
`wasm-pack` build passed. The rebuilt full Qualia package loaded 22 `.10d`
meshes at Portal tier 2, and the full playthrough and replay passed again.
The self-test uses its own session, leaving the playable world at its opening
state.

The graphics pass expands the Kestrel Flats blockout to 57 always-present
`.10d` scene organs, plus state variants. Trees, the water cistern, lamps,
drums and smaller props are generated with Qualia computational geometry
(`authoring` and `parametric_cad`) and retain game-owned source provenance.
Building facades, roads, market stock and garden detail add stronger visual
landmarks. The full Portal browser path accepted all 57 at tier 2 and the
scripted action/replay checks passed. Repairing water and planting the garden
loaded the conditional 58th mesh and updated the Qualia scene receipt. This is
still an authored blockout scene;
materials, lighting, animation and finished HMC packs remain upstream gates.

The VibeScript workbench now has three concrete paths. A pure cell reads a
snapshot of the playable world (resources and project status), a command cell
returns a catalogue action id that is sent through the existing Qualia SHACL
gate, and a scene cell describes one bounded sphere or cylinder. The scene
cell is evaluated by Qualia VibeScript, shaped by Qualia computational
geometry, sealed as `.10d`, and carries its Vibe source in the provenance
section. Rejected scripts leave the last good scene active. This is a game
adapter over existing public Qualia libraries; it does not implement a second
language, rules engine, geometry engine or renderer.

The animated-storybook art pass adds 34 game-owned scene organs to the
previous 57: clustered canopies, four rounded residents, flowers, hills,
clouds and a sun motif. The campaign interface uses a warmer illustrated
palette and the default camera gives the town more space on screen. All scene
forms still use Qualia computational geometry, `.10d` provenance and the full
QualiaPortal. The earlier browser HDR presentation made opaque assets look
pale against a black clear; QG-12 requests a generic opaque-occlusion and
colour fixture.

This game pass authors a second land tile, Saltwind Reach, beside Kestrel
Flats. Both tiles share one Qualia scene, world document and event tape. A
restored bridge, commissioned wind pump and planted orchard form a second
SHACL-gated project chain. Qualia's new public camera target and daylight sky
APIs are used by the game shell; the web UI provides a territory selector.
This is a contiguous two-tile playable map, not yet generic tile streaming or
worker pathfinding.

**Upstream WASM gate observed and corrected by the Qualia agent (2026-10-03):** QualiaDB
commit `0300525b` defined `list_hmc_bundle_entries_wasm` twice in
`crates/qualia-core-db/src/wasm_bridge/dataio.rs` (lines 404 and 436).
The Qualia development agent removed the duplicate in commit `32ef0175` while
this pass was underway. The game shell's `wasm32-unknown-unknown` check and
release WASM build pass against that clean checkout.
Preserve the one tested
HMC ABI in an upstream commit before pinning a reproducible game build.

**Historical QG-12 visual proof, now resolved by the later renderer fix series:** The earlier browser package accepted all
115 always-present `.10d` organs across Kestrel Flats and Saltwind Reach at
Portal tier 2. Its self-test passes the new bridge, pump, orchard and replay
sequence. Yet the WebGPU canvas is entirely black, including after changing
sky preset. A one-organ diagnostic isolated the same result with a successful
WebGPU receipt (1 organ, 12 triangles, 8 vertices). A no-WebGPU diagnostic
displayed Qualia's tier-1 field, so this is isolated to the full WebGPU scene
path. See QG-12 in the [upstream gate work orders](19-qualiadb-upstream-gate-work-orders.md)
for the reproduction and required generic conformance fixture. This game pass
does not substitute another renderer or silently treat upload as visual proof.

## QualiaDB audit: next generic needs

This table records **observed integration gaps or proofs to run**, not an
assertion that another agent has not implemented them. The QualiaDB checkout
was clean at `32ef0175`; recheck the exact tested revision
before closing a row. See the generic upstream brief at
`qualiaDB/docs/work-in-progress/qualia-capability-demonstration-program.md`
and the game's [detailed gate work orders](19-qualiadb-upstream-gate-work-orders.md).

| Need | Current local evidence | General QualiaDB outcome / closing proof |
|---|---|---|
| Durable mutable Q42 session | New `q42/journal.rs` and WASM bridge exist, but `WasmQ42Session` currently uses `Cursor<Vec<u8>>`; game still saves an N3 text world through OPFS. | One public Q42 session with real browser durability, replay, migration, base/pack digest checks, interrupted-write recovery and live Vibe query. Prove browser restart and a non-game consumer before migrating the game. |
| Continuous deterministic simulation | New generic `simulation/fixed_tick.rs` exists; game still uses discrete action proposals. | Verify canonical simultaneous-command ordering, Q42/rule validation, no dropped receipts, native/WASM replay and budget; expose the accepted public bridge and integrate one worker/project task. |
| RTS camera, group selection and navigation | QualiaPortal now exposes camera target/pan. The game has Map, Survey and ground-height Walk presentation with three destinations. Ground walking has no collision or route planner, and semantic pick remains single target. Generic deterministic navigation is present in source but not yet integrated in this game. | Verify world hit, collision, selection sets, commands and route/navigation service in the public WASM surface. Prove at least 100 entities, changing obstacles and semantic IDs across zoom/LOD before adding a private game implementation. |
| Finished assets and content packs | The game-owned catalog compiles 274 distinct computational-geometry `.10d` variants through Qualia, with 234 organs in the opening scene. The full WASM page loads these meshes; core HMC bundle APIs and a game pack exist. Canonical pack selection and asset manifest browser proof remain open. | Reproducible Qualia source → validated `.10d`/Q42 → canonical HMC → offline browser load, with licence/digest/LOD/semantic picks. Then replace blockouts one family at a time. |
| WebGPU scene visibility and finished surfaces | QG-12's black viewport was resolved by the upstream renderer fix series through `0006a07d`; full Portal tier 2 visibly renders the game scene. The current models remain primitive, shading is dim, and tensor projectors show bright square markers. No finished texture, material, shadow, or LOD contract has been demonstrated in the game. | Add a generic projector-visibility control that keeps semantic pick buffers active. Verify reusable material/texture bindings, light, shadow, sky, and LOD through full WASM Portal and `.10d`/HMC before calling art finished. |
| Animation | Static `.10d` meshes render; a stored rig/clip-to-Portal contract has not been demonstrated in the game. | Versioned `.10d` animation format and browser playback/replay for person, vehicle and facility actions. |
| Audio in normal play | `qualia-audio`, core audio DSP and Portal acoustic APIs already exist; the game has no sound cues or music yet. | Expose/integrate existing Qualia Audio for authored cues, ambience, spatial playback, captions, volume and replay-once policy in `wasm-full`. Do not create a second audio engine. |
| VibeScript as game authoring language | Game now evaluates world snapshot queries, SHACL-gated action ids and one authored `.10d` asset from a Vibe cell. The world projection still reads the temporary N3 session, scene records are limited to two primitives, and diagnostic/result types are game-specific. | Public, generic Vibe host over durable Q42 with typed read-only graph queries, typed command proposals and receipts, a bounded scene authoring schema, capability grants, provenance, and clear diagnostics. Demonstrate non-game use and migrate this adapter away with the N3 session. |
| Pin and conformance | Two-territory WASM check and release build use clean Qualia commit `32ef0175`; the game still uses a sibling path dependency and browser visual proof fails. | Test an immutable Qualia revision with native/WASM/browser visual fixtures, record format versions and use it as the game's actual dependency pin. |

Other planned capabilities (P64 local NPCs, public-data terrain, on-demand
geography and multiplayer) stay in the upstream gate register and are not
needed to validate the immediate offline playable slice. They are not
replaced by game-side services if selected later.

## Next game-only steps

1. Browser-test semantic picks and rerun the playthrough/replay after each
   browser package rebuild. Keep both repair routes and save/reload in the
   acceptance set.
2. Improve game-owned source assets and art direction while retaining
   Qualia's `.10d` compiler and renderer. Keep source IDs, provenance and
   licence records separate from the engine.
3. Build a guided opening that teaches selection, resources and two viable
   project routes through existing Qualia-validated actions.
4. When a verified generic Qualia Q42 session and fixed-tick API become
   available, migrate the temporary N3/action session and add workers,
   movement and construction. Remove the temporary authority path in the
   same migration; do not run parallel state machines.
5. Once canonical HMC and animation pass upstream, use finished packs and
   animated people in the browser. Do not claim blockout meshes are AAA art.
