# Destructible kinds catalog

Status: **kinds list first**. No mesh batch. No pretty destructible stubs.
Textures, animation, and behaviour wait until that kind is sourced **and** the
canvas paints. Extends [18-asset-production-catalog.md](18-asset-production-catalog.md)
and the [Qualia-only contract](13-qualiadb-only-development-contract.md).

Art direction (appeal): original Pixar-like clear silhouette and warm light —
look only. The pretty frame does not soften a failed state. This is **not** a
shooting game; violence is a consequence inside the fictional frame, never a
smash toy and never a real human in the wreck.

## Claim rules

| Rule | Meaning |
| --- | --- |
| Humans | **Not a destructible kind.** No intact/stressed/failed mesh for a who. A life that ends is scene state, not an asset. |
| Kind before mesh | Catalog kinds and how each fails; do not invent stub meshes |
| Three looks | Every kind that can break: `intact` → `stressed` → `failed` |
| Cause named | A destruction cell names **kind** + **cause** |
| Living kinds | Flora / fauna / funga harmed as living presence, never a who |
| Amenity | A shop has nothing to break until it has a facade |
| Human | No portrait of a real person in any wreck; life-ending stays scene state |

## Cause vocabulary (closed set for v0)

| Cause id | Typical use |
| --- | --- |
| `cause.impact` | Collision, hitch failure, debris strike |
| `cause.heat` | Overheat, fire spread to instrument |
| `cause.load` | Bad mass on hitch / structural overload |
| `cause.flood` | Water form / inundation on ground or structure |
| `cause.fire` | Wildfire / structure fire (disaster) |
| `cause.storm` | Wind, hail, tree-fall |
| `cause.wear` | Gradual maintenance failure |
| `cause.drought` | Water stress on living flora/funga (and soil moisture fields) |
| `cause.violence` | Intentional harm in-scene — consequence, not a toy mode |

Violence and disaster both stay **scene receipts**. Neither binds or instructs
against a real human.

## Kind table

Columns: `kind_id`, family, living?, causes that apply, intact/stressed/failed
meaning, texture/anim/behaviour notes, mesh status.

### Instruments (handle — not a who)

| kind_id | Causes | intact | stressed | failed | tex / anim / behaviour | mesh |
| --- | --- | --- | --- | --- | --- | --- |
| `instrument.vehicle` | impact, heat, load, wear, violence, storm | roadworthy | overheat / damage | write-off | heat gauge, load silhouette; drive cell | blockout only |
| `instrument.van` | impact, heat, load, wear, violence, storm | hitchable | sway / overload | write-off (may take vehicle) | hitch + mass sit; tow cell | blockout only |
| `instrument.cargo_cart` | impact, load, wear | usable | damaged bed | scrap | carry behaviour | missing |
| `instrument.licence` | — (capacity) | held | at-risk | revoked → instruments **parked** | not a mesh; capacity on handle | n/a |

Fuel cost stays a **trip scenario claim**, not spending authority.

### Ground and water (natural / place fields)

| kind_id | Causes | intact | stressed | failed | notes | mesh |
| --- | --- | --- | --- | --- | --- | --- |
| `ground.terrain` | flood, fire, storm, wear | passable | eroded / scorched | impassable / washed out | CRS + uncertainty named | blockout |
| `ground.road` | flood, storm, wear, impact | open | damaged | closed | passability receipt | blockout |
| `ground.bridge` | flood, storm, impact, load, wear | open | braced / limited | collapsed gap | canal tide coupling | blockout |
| `water.natural` … `water.black` | flood, storm, wear (treatment ≠ merge) | form stable | contaminated / rising | form change only via treatment cell | six forms stay distinct | empty |

### Buildings and amenities

| kind_id | Causes | intact | stressed | failed | notes | mesh |
| --- | --- | --- | --- | --- | --- | --- |
| `building.workshop` | fire, storm, impact, wear, violence | operating | fault / unpowered | ruined | power + project stage | blockout |
| `building.hall` | fire, storm, wear, violence | open | damaged | closed | occupancy | blockout |
| `building.market` / shop facade | fire, storm, impact, violence | trading | damaged stock | no facade ⇒ **nothing to break** | amenity, not a who | blockout / missing facade |
| `building.tank` | impact, storm, wear, violence | sealed | leaking | ruptured | water litres | blockout |
| `building.solar` / battery | storm, heat, wear, violence, impact | generating | fault | scrap | capacity | blockout |

### Living non-persons (separate kinds)

| kind_id | Causes | intact | stressed | failed | notes | mesh |
| --- | --- | --- | --- | --- | --- | --- |
| `living.flora.*` | fire, flood, storm, drought, wear, violence | healthy | stressed | dead / cleared | presence on place | empty / blockout trees |
| `living.fauna.*` | fire, flood, storm, impact, violence | healthy | injured / fleeing | dead (scene) | **personality = trait**, not who | empty |
| `living.funga.*` | fire, flood, storm, drought, violence | present | stressed | gone | separate kind | empty |

### Props and resources

| kind_id | Causes | intact | stressed | failed | notes | mesh |
| --- | --- | --- | --- | --- | --- | --- |
| `prop.stockpile` | fire, flood, storm, violence, wear | stored | damaged | lost | amount + condition | blockout |
| `prop.tool` | wear, impact, violence | usable | worn | broken | repair path | missing |

### Travel corridor

| kind_id | Causes | intact | stressed | failed | notes | mesh |
| --- | --- | --- | --- | --- | --- | --- |
| `travel.corridor` | storm, flood, fire | open | hazard | closed | dated OSM snapshot when sourced | empty |
| `travel.amenity_point` | — | present | — | removed from snapshot version | shop/service **point**, not who | empty |

## Production order (after paint)

1. One instrument pair (vehicle + van) with intact/stressed/failed + hitch load.
2. One building (workshop) with fire/storm/wear failure looks.
3. One ground sector with flood/fire wear.
4. One flora family and one fauna with trait (no face).
5. Only then scale textures, LODs, and animation clips through Qualia `.10d`.

## Soft-rise gate

No soft-rise of destruction looks, smash VFX, or “destructible toy” chrome while
`?firstpaint` is still a receipt / empty shell. Tip `1987916` pin `v0.0.40.11`
does not count as paint until a tick shows non-black pixels without `unreachable`.
