# Generated game assets

`10d/` contains Maslows Challenge mesh assets compiled by the full Qualia WASM
package from game-owned recipes in `crates/rolling-commons-shell/src/asset_catalog.rs`.
`manifest.json` lists each asset's semantic ID, content hash, size, and the
scenario states that use it. These are game content artifacts, stored in this
repository. The QualiaDB repository owns the compiler, container format,
computational geometry, and renderer, not these meshes.

Run `scripts/build-game.ps1` at the repository root to rebuild `web/pkg` and
re-export the assets. The exporter deduplicates identical `.10d` containers
across eight representative scene states. Runtime scene construction remains
authoritative for state combinations beyond those snapshots; new assets and
states should be added to the exporter when authored.

The export is reproducible from the WASM package identified by SHA-256 in the
manifest. The same exporter writes `web/assets/maslows-challenge-scenes.hmc`
using Qualia's QBDL/HMC writer and verifies every entry with its reader.
The pack carries the JSON scene manifest and intact `.10d` files. It is a
provisional pack until QualiaDB resolves the canonical HMC variant and adds
the Q42 identity/rights manifest required for production distribution.
The export script does not change QualiaDB source or its licence.
