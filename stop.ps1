# stop.ps1 — Coupe le stack dev UseStakly (backend + frontend + Postgres)
# Usage depuis la racine du projet : .\stop.ps1

$ErrorActionPreference = "Continue"
$root = $PSScriptRoot
Set-Location $root

Write-Host "UseStakly dev stack - stop" -ForegroundColor Cyan
Write-Host "=========================="

# 1. Tuer les arbres pwsh lances par dev.ps1 (reconnus via leur CommandLine,
#    pas via MainWindowTitle qui est vide sous Windows Terminal).
$targets = @("UseStakly - Backend", "UseStakly - Frontend")
$killedRoots = @()

foreach ($title in $targets) {
    $procs = Get-CimInstance Win32_Process -Filter "Name = 'pwsh.exe' OR Name = 'powershell.exe'" -ErrorAction SilentlyContinue |
        Where-Object { $_.CommandLine -and $_.CommandLine -like "*$title*" }

    if (-not $procs) {
        Write-Host "[$title] Aucune fenetre active." -ForegroundColor DarkGray
        continue
    }

    foreach ($p in $procs) {
        Write-Host "[$title] taskkill /F /T PID $($p.ProcessId)..." -ForegroundColor Yellow
        # taskkill /T tue l'arbre entier (pwsh + cargo + cargo-watch + backend, ou pwsh + npm + node/vite).
        & taskkill.exe /F /T /PID $p.ProcessId | Out-Null
        $killedRoots += $p.ProcessId
    }
}

# 2. Filet de securite : si un serveur a ete lance hors dev.ps1 et tient encore le port,
#    tuer par port directement. Utilise aussi taskkill /T pour couvrir cargo-watch residuel.
$ports = @(
    @{ Port = 4000; Label = "Backend" },
    @{ Port = 5173; Label = "Frontend" }
)

foreach ($entry in $ports) {
    $port = $entry.Port
    $label = $entry.Label
    $pids = Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue |
        Select-Object -ExpandProperty OwningProcess -Unique

    if (-not $pids) {
        Write-Host "[$label] Port $port libre." -ForegroundColor DarkGray
        continue
    }

    foreach ($procId in $pids) {
        if ($killedRoots -contains $procId) { continue }
        Write-Host "[$label] Port $port toujours occupe par PID $procId, kill force..." -ForegroundColor Yellow
        & taskkill.exe /F /T /PID $procId | Out-Null
    }
}

# 3. Docker compose down (Postgres)
Write-Host "Docker compose down..." -ForegroundColor Yellow
& docker compose down

# 4. Verification finale
Start-Sleep -Milliseconds 500
$still = Get-NetTCPConnection -LocalPort 4000,5173 -ErrorAction SilentlyContinue
if ($still) {
    Write-Warning "Ports encore occupes :"
    $still | Select-Object LocalPort, OwningProcess | Format-Table -AutoSize
    exit 1
} else {
    Write-Host ""
    Write-Host "Stack arrete. Ports 4000 / 5173 libres." -ForegroundColor Green
}
