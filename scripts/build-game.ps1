$ErrorActionPreference = 'Stop'
$repo = Resolve-Path (Join-Path $PSScriptRoot '..')
$crate = Join-Path $repo 'crates\rolling-commons-shell'
$qualia = Join-Path $repo '..\qualiaDB'
$wasmBindgenVersion = '0.2.125'
$wasmBindgenRoot = Join-Path $repo 'target\wasm-bindgen-cli'
$wasmBindgenBin = Join-Path $wasmBindgenRoot 'bin'
$wasmBindgenExe = Join-Path $wasmBindgenBin 'wasm-bindgen.exe'
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
        'C:\Users\tim_h\AppData\Local\nvm\v22.12.0',
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
    # Use an absolute crate path: wasm-pack compares Cargo metadata paths, and
    # this checkout cannot canonicalize sandboxed Windows roots reliably.
    wasm-pack build $crate --target web --out-dir ../../web/pkg --release --mode no-install
    if ($LASTEXITCODE -ne 0) { throw 'wasm-pack failed' }
    $pkgIgnore = Join-Path $repo 'web/pkg/.gitignore'
    if (Test-Path -LiteralPath $pkgIgnore) { Remove-Item -LiteralPath $pkgIgnore }
    node scripts/export-game-assets.mjs
    if ($LASTEXITCODE -ne 0) { throw 'asset export failed' }
    # Semantic section is authored by taxon-bindings.vibe; pack-export
    # evaluates it natively, embeds sem/taxonomy.q42, and repacks the HMCs.
    cargo run --locked -p pack-export -- --all
    if ($LASTEXITCODE -ne 0) { throw 'pack-export failed' }
} finally {
    Pop-Location
}
