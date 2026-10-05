# Playable characters and first-person play

## Player-facing goal

Make the valley feel like a place inhabited by people with different reasons to
be there. The player can create and switch between protagonists from the
Qualia-rendered in-game HUD. Each protagonist has a personal introduction, a
story path, a field trait, a home region, a saved field position, and their own
current chapter. Choosing someone else must not erase the first character's
location or personal progress.

The first-person view is a way to inhabit and explore the existing strategy
game, not a separate demo or alternate rules engine. Walking, looking, and
exploring nearby sites should lead back into the same Qualia game session and
the same rule-checked town decisions.

## Current playable slice

- Three starting characters are available: Ada, Mira, and Orin. The in-game
  creator can add up to twelve profiles from a name roster.
- The player chooses one of four story paths (Maker, Grower, Crossing, or
  Steward), one field trait, and one of three starting regions.
- Each name has a short personal background. The selected story path supplies
  the current chapter and goal; chapter text responds to the shared campaign's
  milestones.
- Characters keep independent region and field coordinates. Switching to a
  character resumes them there; choosing a region from the in-game HUD moves
  that character to its region's start point.
- Traits change nearby-site interaction radius or walking speed. `WASD` moves,
  drag looks around, `Shift` runs, `E` opens a nearby site, and `1`/`2` switch
  back to Map/Survey. Nearby actions still go through the existing Qualia
  session's action gates.
- The roster and creator are rendered through Qualia HUD documents on top of
  the 3D view. They are not HTML menus.
- Character profiles are saved separately from the shared campaign replay,
  through the existing Qualia local world-save/load API. Switching profiles
  cannot append events to or alter the campaign replay.

## Narrative and state model

The campaign is shared: its economy, infrastructure, and event history stay
consistent when a player switches protagonists. Personal identity and field
position are per-character. Story chapters are currently projections of the
selected arc and shared campaign milestones, not independent branching
campaigns. The character background is derived from the selected name; a
character can be customized with the current fixed choice lists, but cannot yet
be given arbitrary free text or a bespoke portrait.

This division keeps the existing event-sourced campaign deterministic. It is
an initial slice, not a claim that independent character quests or a full RPG
progression system are complete.

## Character creation and switching acceptance

1. Character creation, selection, and return-to-game controls stay inside the
   game HUD and work with pointer and keyboard input.
2. A created character receives a distinct ID and retains its selected story,
   trait, home, current region, and position after reload.
3. Switching away and back resumes the saved field position and does not reset
   shared town state or another character's profile.
4. Each story path has a clear opening, a current actionable goal, and chapter
   changes that follow authored campaign milestones. Text must fit the HUD at
   supported viewport sizes.
5. Traits have visible, explainable effects and do not bypass game rules.
6. Each start region provides a safe spawn, a legible first-person view, and at
   least one nearby place to explore. Camera look remains inside world bounds;
   walking should not pass through terrain, buildings, or people.
7. The player can find the active character, region, story goal, and available
   nearby interaction without leaving first person.
8. Saving and loading either the campaign or character profiles must report
   failures clearly and preserve the other data set.

## Current gaps to close

The next simulation layer is specified in the
[character behaviour and logic mechanics plan](24-character-behaviour-and-logic-mechanics.md).
It extends this roster/story slice with observable practices, commitments,
relationship history, community governance, project delivery, and explainable
cross-scope consequences through QualiaDB's logic capabilities; those mechanics
are not implemented by the current character profiles yet.

### Game content and interaction

- The three seed characters start in different named regions, but all regions
  are parts of the same current settlement scene. Distinct starting locations
  need their own authored landmarks, residents, ambience, and first encounters.
- Chapter goals are currently short prompts over the shared town quest graph.
  Each protagonist needs a complete authored arc: scene-setting, character
  relationships, choices, consequences, and a satisfying resolution.
- Character identity is text-only. The player cannot yet choose appearance,
  voice, pronouns, equipment, accessibility options, or a custom name.
- First-person movement has no robust collision, slopes, steps, path-following,
  interaction targeting feedback, or obstruction recovery. These must use
  Qualia spatial and geometry capabilities as they become available.
- There is no visible player body, hands, or character-specific animation in
  first person. NPC scale, facing, and spacing need authored review from every
  spawn, and character models need readable silhouettes and expressions.
- Save feedback, duplicate-name policy, delete/rename flows, version migration,
  and multiple local player slots still need product decisions.

### QualiaDB capability work, if current APIs cannot satisfy the needs

Keep these generic and upstream in QualiaDB; do not add game-only engine
forks, JavaScript rendering, or substitute engines:

- **Character HUD controls:** in-world portrait/icon support, multi-line
  responsive text, and accessible choice/input controls, including free-text
  entry, validation, focus handling, and keyboard navigation.
- **Spatial movement:** queryable collision shapes, walkable-surface and slope
  queries, swept movement, and geometry-backed pathfinding/navigation with
  predictable WASM APIs. The game should use the computational geometry and
  spatial libraries already in QualiaDB before proposing new primitives.
- **Character presentation:** efficient animated 3D character rigs, facial and
  gesture animation, first-person hands/body presentation, and animation state
  control available to VibeScript and the WASM renderer.
- **Camera controls:** configurable field of view, look sensitivity, inversion,
  camera collision/obstruction handling, per-character saved preferences, and
  stable touch/gamepad mappings exposed through the Qualia camera API.
- **Profile persistence:** a generic versioned mutable-record/profile API with
  transactional writes, migration hooks, and independent save slots. The
  current implementation stores a strict fixed-schema N3 roster using the
  existing Qualia OPFS world-save API; it is not a Q42 or HMC character format.
- **VibeScript:** author scene introductions, character dialogue, chapter
  transitions, and safe world queries in VibeScript with minimal JavaScript
  glue. A script must not bypass the authoritative Qualia game-session rules.

Before implementing each item, verify the current full Qualia WASM package and
libraries. Record a generic QualiaDB work order if the capability is absent;
continue independent game work while it is developed. Package static authored
character content and presentation assets in the project's HMC when that
pipeline is ready; store mutable profile state through the generic Qualia
profile/persistence capability.

## Verification walkthrough

1. Open the game and choose **Walk**.
2. Open **Characters**, inspect the personal hook/story/trait/place for each
   profile, and select Mira or Orin. Confirm the view starts in their region.
3. Move, open a nearby site with `E`, then switch away and return. Confirm each
   protagonist resumes at their own location while the campaign state remains
   shared.
4. Create another character, choose a different story, trait, and region, then
   reload the page and confirm the profile remains available.
5. Use a small viewport and keyboard-only navigation to inspect clipping,
   focus, labels, and return paths.
