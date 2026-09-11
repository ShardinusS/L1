# Etape post-build de Trunk.
#
# 1. Duplique index.html en 404.html : un hebergeur statique renvoie alors
#    l'application pour /systemes, /information, etc.
# 2. En release, reduit le bundle WebAssembly avec wasm-opt. Trunk sait le
#    faire lui-meme (data-wasm-opt), mais sous Windows il passe a binaryen un
#    chemin prefixe par \\?\ que celui-ci n'ouvre pas; on l'appelle donc
#    directement, sur un chemin nettoye.
#
# Le script ne fait jamais echouer le build : au pire le bundle reste tel quel.

function Clean-Path([string]$p) {
    if (-not $p) { return $p }
    return ($p -replace '^\\\\\?\\', '')
}

function Find-WasmOpt {
    $onPath = Get-Command wasm-opt -ErrorAction SilentlyContinue
    if ($onPath) { return $onPath.Source }
    $cache = Join-Path $env:LOCALAPPDATA 'trunkrs\trunk\cache'
    if (Test-Path $cache) {
        $found = Get-ChildItem $cache -Recurse -Filter 'wasm-opt.exe' -ErrorAction SilentlyContinue |
                 Sort-Object FullName -Descending | Select-Object -First 1
        if ($found) { return $found.FullName }
    }
    return $null
}

try {
    $dir = Clean-Path $env:TRUNK_STAGING_DIR
    if (-not $dir -or -not (Test-Path $dir)) { $dir = Clean-Path $env:TRUNK_DIST_DIR }
    if (-not $dir -or -not (Test-Path $dir)) { $dir = Join-Path $PSScriptRoot '..\dist' }
    if (-not (Test-Path $dir)) {
        Write-Output "post-build : dossier de sortie introuvable, rien a faire."
        exit 0
    }

    $index = Join-Path $dir 'index.html'
    if (Test-Path $index) {
        Copy-Item $index (Join-Path $dir '404.html') -Force
        Write-Output 'post-build : 404.html ecrit (repli SPA).'
    }

    if ($env:TRUNK_PROFILE -ne 'release') { exit 0 }

    $wasmOpt = Find-WasmOpt
    if (-not $wasmOpt) {
        Write-Output 'post-build : wasm-opt introuvable, bundle laisse tel quel.'
        exit 0
    }

    foreach ($wasm in @(Get-ChildItem $dir -Filter '*_bg.wasm' -ErrorAction SilentlyContinue)) {
        $before = $wasm.Length
        $tmp = "$($wasm.FullName).opt"
        & $wasmOpt -Oz --output $tmp $wasm.FullName
        if ($LASTEXITCODE -ne 0 -or -not (Test-Path $tmp)) {
            Write-Output "post-build : wasm-opt a echoue sur $($wasm.Name), bundle laisse tel quel."
            if (Test-Path $tmp) { Remove-Item $tmp -Force }
            continue
        }
        Move-Item $tmp $wasm.FullName -Force
        $after = (Get-Item $wasm.FullName).Length
        $pct = [math]::Round(100 - ($after / $before * 100))
        Write-Output ("post-build : {0} {1:N0} -> {2:N0} octets (-{3} %)" -f $wasm.Name, $before, $after, $pct)
    }
}
catch {
    Write-Output "post-build : ignore ($($_.Exception.Message))"
}

exit 0
