# Current Game Build and QualiaDB Needs

Status: living implementation note, 2026-10-04. The QualiaDB checkout is
**read-only for this game-development pass** at the project owner's request.
The game may consume existing public Qualia APIs and add game-owned content,
bindings and presentation. Any missing general capability is recorded here
for the QualiaDB development agent; it is not rebuilt privately in the game.

## Game work in this pass

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
crossing and replay reproduced the world byte-for-byte. The full renderer
visibility gate QG-12 remains open: mesh receipt changes are not proof of
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
viewport remains black, so this is a playable **rules/UI** level whose 3D
visual acceptance is still blocked by QG-12.

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
The QualiaDB checkout remains read-only for this task. Preserve the one tested
HMC ABI in an upstream commit before pinning a reproducible game build.

**Current blocking visual proof:** The rebuilt browser package accepts all
115 always-present `.10d` organs across Kestrel Flats and Saltwind Reach at
Portal tier 2. Its self-test passes the new bridge, pump, orchard and replay
sequence. Yet the WebGPU canvas is entirely black, including after changing
sky preset. A one-organ diagnostic isolated the same result with a successful
WebGPU receipt (1 organ, 12 triangles, 8 vertices). A no-WebGPU diagnostic
displayed Qualia's tier-1 field, so this is isolated to the full WebGPU scene
path. See QG-12 in the [upstream gate work orders](19-qualiadb-upstream-gate-work-orders.md)
for the reproduction and required generic conformance fixture. This game pass
does not substitute another renderer or silently treat upload as visual proof.

## Read-only QualiaDB audit: next generic needs

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
| RTS camera, group selection and navigation | QualiaPortal now exposes camera target/pan and the game uses territory focus; semantic pick remains single target. Generic deterministic navigation is present in source but not yet integrated in this game. | Verify world hit, selection sets, commands and route/navigation service in the public WASM surface. Prove at least 100 entities, changing obstacles and semantic IDs across zoom/LOD before adding a private game implementation. |
| Finished assets and content packs | 115 game-owned computational-geometry blockouts compile to `.10d` through Qualia; current page loads meshes at runtime. Core HMC bundle APIs exist; canonical pack selection and asset manifest browser proof remain open. | Reproducible Qualia source → validated `.10d`/Q42 → canonical HMC → offline browser load, with licence/digest/LOD/semantic picks. Then replace blockouts one family at a time. |
| WebGPU scene visibility and finished surfaces | The full Portal accepts 115 meshes but renders an entirely black canvas, even with one accepted cube-like organ. Sky presets and lighting APIs exist at `32ef0175`; their visible result is unproven. The older package showed pale blockouts. No game-facing texture, material, shadow or LOD contract has been demonstrated. | First fix the generic accepted-opaque-cube browser visual regression with pixel and occlusion proof in Qualia. Then verify or expose material/texture bindings, light, shadow, sky and LOD through full WASM Portal and `.10d`/HMC; prove a reusable non-game scene before calling art finished. |
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
