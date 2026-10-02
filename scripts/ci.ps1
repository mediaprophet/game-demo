# Rolling Commons — Phase 0 CI commands (Windows host).
# Usage: scripts\ci.ps1 [-Step all|fmt|check|test|wasm|assets]
param([string]$Step = "all")

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root

function Fmt {
    cargo fmt --all -- --check
}

function Check {
    # Authoritative profile check for wasm32 (no GPU/LLM surface).
    cargo check --workspace --target wasm32-unknown-unknown
}

function Tests {
    cargo test --workspace --target wasm32-unknown-unknown --no-run
}

function Wasm {
    wasm-pack build crates/rolling-commons-shell --target web --out-dir ../../web/pkg --release
}

function Assets {
    # Placeholder for .10d/HMC pack validation (QG-01/QG-02 fixtures land here).
    if (Test-Path packs) {
        Write-Host "packs/: asset validation pending QG-01/QG-02"
    } else {
        Write-Host "no packs/ directory yet — nothing to validate"
    }
}

switch ($Step) {
    "fmt"    { Fmt }
    "check"  { Check }
    "test"   { Tests }
    "wasm"   { Wasm }
    "assets" { Assets }
    "all"    { Fmt; Check; Tests; Wasm; Assets }
    default  { Write-Error "unknown step: $Step" }
}

Pop-Location
