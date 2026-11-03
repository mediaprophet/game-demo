$ErrorActionPreference = 'Stop'
$repo = Resolve-Path (Join-Path $PSScriptRoot '..')
$crate = Join-Path $repo 'crates\rolling-commons-shell'
$qualia = Join-Path $repo '..\qualiaDB'
$wasmBindgenVersion = '0.2.129'
$wasmBindgenRoot = Join-Path $repo 'target\wasm-bindgen-cli'
$wasmBindgenBin = Join-Path $wasmBindgenRoot 'bin'
$wasmBindgenExe = Join-Path $wasmBindgenBin 'wasm-bindgen.exe'
$cachedWasmPackBindgen = 'C:\Users\tim_h\AppData\Local\.wasm-pack\wasm-bindgen-ba0b07921e305a69\wasm-bindgen.exe'
if (Test-Path -LiteralPath $cachedWasmPackBindgen) {
    $wasmBindgenExe = $cachedWasmPackBindgen
    $wasmBindgenBin = Split-Path -Parent $cachedWasmPackBindgen
}
$expected = (Get-Content -LiteralPath (Join-Path $repo 'qualia.ref') -Raw).Trim()
$actual = (git -c safe.directory=C:/github/qualiaDB -C $qualia rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0 -or $actual -ne $expected) {
    throw "QualiaDB checkout must be at $expected (found $actual)"
}
if (-not (Test-Path -LiteralPath $wasmBindgenExe)) {
    cargo install wasm-bindgen-cli --version $wasmBindgenVersion --locked --offline --root $wasmBindgenRoot
    if ($LASTEXITCODE -ne 0) { throw "Unable to install cached wasm-bindgen $wasmBindgenVersion" }
}
$env:PATH = "$wasmBindgenBin;$env:PATH"
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    $nodeCandidates = @(
        'C:\Users\tim_h\AppData\Local\nvm\v24.22.0',
        'C:\nvm4w\nodejs',
        'C:\Program Files\nodejs'
    )
    foreach ($candidate in $nodeCandidates) {
        if (Test-Path (Join-Path $candidate 'node.exe')) {
            $env:PATH = "$candidate;$env:PATH"
            break
        }
    }
}
Push-Location $repo
try {
    cargo build --offline -p rolling-commons-shell --target wasm32-unknown-unknown --release
    if ($LASTEXITCODE -ne 0) { throw 'cargo build rolling-commons-shell failed' }
    & $wasmBindgenExe --target web --typescript --out-dir web/pkg --out-name rolling_commons_shell target/wasm32-unknown-unknown/release/rolling_commons_shell.wasm
    if ($LASTEXITCODE -ne 0) { throw 'wasm-bindgen failed' }
    $pkgIgnore = Join-Path $repo 'web/pkg/.gitignore'
    if (Test-Path -LiteralPath $pkgIgnore) { Remove-Item -LiteralPath $pkgIgnore }
    node scripts/export-game-assets.mjs
    if ($LASTEXITCODE -ne 0) { throw 'asset export failed' }
    # Semantic section is authored by taxon-bindings.vibe; pack-export
    # evaluates it natively, embeds sem/taxonomy.q42, and repacks the HMCs.
    cargo run --offline --locked -p pack-export --bin pack-export -- --all
    if ($LASTEXITCODE -ne 0) { throw 'pack-export failed' }
} finally {
    Pop-Location
}
