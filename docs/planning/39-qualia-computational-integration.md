# QualiaDB computational integration for the game

Source review: 12 October 2026. This supplements plans 35–38. It does not
replace their scope or mark any graphics/gameplay milestone complete.

## Implementation rule

QualiaDB is the computational foundation. Before introducing a mathematical
algorithm, inspect `solvers/`, `specialized_libs/`, the capability manifests,
and `poet_host/invoke/`. Reuse the canonical engine implementation and expose
missing game adapters in QualiaDB. Game VibeScript supplies rules, parameters
and state transitions; browser JavaScript supplies platform events and display
transport. Do not implement a parallel numerical engine in JavaScript.

Wolfram-style breadth is the intended capability target. Assess each required
operation against its actual implementation, input domain and bindings; do not
infer universal equivalence from a demo or module name. A missing binding is
an integration task, not evidence that the engine lacks the mathematics.

## Sources inspected

The supplied public URLs could not be fetched through the web reader. Their
repository copies were read instead:

- `qualiaDB/docs/compute-engine.html`: browser CPU linear algebra, statistics,
  symbolic algebra, special functions, exact arithmetic, units, transforms,
  graph reasoning and cryptography.
- `qualiaDB/docs/scientific-computing.html`: native WASM calls for Monte Carlo,
  RK4 ODEs and domain science demonstrations; JavaScript orchestrates the demo
  controls rather than owning the solver.
- `qualia-core-db/src/solvers/mod.rs`: canonical solver modules, including
  calculus, interpolation, graph optimization, linear algebra, statistics,
  optimization, units, transforms and geometric algebra.
- `specialized_libs/mod.rs`: geometry, symbolic algebra and advanced symbolic
  integration, limits, series, ODEs, solve and trigonometry, plus domain libraries.
- `poet_host/invoke/math/mod.rs` and `invoke/mod.rs`: Vibe host dispatch for
  calculus, closure-driven ODE integration, CAS, algebra, optimization and units.

## Game integration map

| Game requirement | Existing QualiaDB surface to inspect/use | Game responsibility |
| --- | --- | --- |
| Curved terrain, banks, roads and asset meshes | `specialized_libs/computational_geometry/`, including authoring, loft, triangulation, distance and BVH | Geographic input, artistic constraints and material assignment |
| Large shoreline distance authoring | Existing `bvh::build_bvh_recursive`, `Aabb::distance_sq_to_point`, `point_segment_distance_sq_2d`; `render/water_surface/` adapts these | Supply mesh topology and water appearance parameters |
| Rig thermal and suspension dynamics | `solvers/calculus/` fixed/caller-buffered ODE kernels; `poet_host/invoke/math/closure_solvers.rs`; `physics_simulation/` | Heat sources, capacities, cooling curves, suspension constants and accepted state |
| Steering, trailer transforms and camera frames | `solvers/geometric_algebra/`, native render motor and skeleton APIs | Rig geometry, joint identities and camera requirements |
| Calibration and fitting | `solvers/optimization/`, `linear_algebra/`, `interpolation/` | Measured/reference datasets and acceptable fitting error |
| Supply-chain allocation and route planning | `solvers/graph_opt/`, optimization, `specialized_libs/computational_economics/` | Community demand, stock, travel constraints and player decisions |
| Weather, wildlife and population variation | `solvers/statistics/`, relevant physics/population modules | Australian species profiles, public observations and deterministic seeds |
| Units, formulas and explanations | `solvers/units/`, symbolic algebra and Vibe math bindings | Physical quantities and presentation of derived information |

The current shell selects `wasm-full`. Engine `wasm-full` includes `portal` and
`wasm-scientific`; `portal` itself includes `wasm-scientific`. Several CAS and
physics modules now explicitly allow that WASM feature. The compute demo's
older blanket statement that advanced CAS is native-only must not be used to
exclude the current source capabilities. Check the particular module's cfg and
host dispatch, including explicit unavailable stubs for reduced profiles.

## Integration procedure

1. Identify the mathematical operation, physical units, precision, data size
   and whether it runs during cold authoring or every frame/tick.
2. Find the engine kernel and corresponding Vibe host operation. Inspect its
   implementation rather than treating a name or documentation claim as proof.
3. Use caller-owned buffers and bounded work for runtime paths. The allocating
   general-dimension `calculus::dense` authoring functions are not substitutes
   for zero-heap hot kernels. Apply the existing 42 MiB execution budget.
4. Keep accepted game state, solver/policy identity and replay semantics aligned.
   Changing the thermal/contact integrator changes results and requires versioned
   checkpoint migration; do not silently reinterpret existing saves.
5. Author reference, invalid-input, determinism and resource fixtures. Execute
   validation after implementation, respecting the user's testing sequence.

## Current implementation status

The shoreline spatial adapter now calls the existing Qualia BVH and exact
distance kernels. Native water upload supplies caller buffers, admits the full
scratch cost before construction, and records edge/query/workspace statistics.
It does not introduce a game-side spatial solver. This is source work only.

Driving suspension now obtains its body-height/velocity response through
`render/physics/response`, a bounded adapter over `RungeKutta4Static`. Vibe sets
the target, normalized stiffness/damping, travel and speed limits; native code
substeps at up to 240 Hz within the admitted 50 ms interval. Contact initialization
remains a game policy. The contact fingerprint covers the native adapter and
canonical solver source, so changed math refuses old checkpoints explicitly.
The thermal policy now calls `PhysicsWorkbench.compute` with operation
`thermal_response` through the real Qualia Poet host. The native adapter uses
the same canonical fixed-state ODE solver for the two temperature reservoirs.
Vibe still sets ambient/load/cooling targets and damage thresholds. This replaces
the game-owned integrator; it does not establish calibrated heat capacities or
real-world radiator performance. Calibration and checkpoint migration remain.
No numerical validation, builds, Cargo invocation or rendered acceptance is
claimed here.
