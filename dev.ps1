# dev.ps1 — Lance le stack dev UseStakly
# Usage depuis la racine du projet : .\dev.ps1

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
Set-Location $root

Write-Host "UseStakly dev stack" -ForegroundColor Cyan
Write-Host "==================="

# 0. Vérification des ports
$ports = @(4000, 5173)
foreach ($port in $ports) {
    $process = Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue
    if ($process) {
        Write-Warning "Le port $port est déjà utilisé par le PID $($process.OwningProcess). Assure-toi de fermer les instances précédentes."
    }
}

# 1. Docker
Write-Host "[1/3] Postgres (Docker compose up -d)..." -ForegroundColor Yellow
& docker compose up -d

# Attente sommaire pour que Postgres soit prêt (optionnel mais recommandé)
Write-Host "Attente de l'initialisation de la base de données..." -ForegroundColor DarkGray
Start-Sleep -Seconds 2

# 2. Backend
Write-Host "[2/3] Backend (cargo watch -x run)..." -ForegroundColor Yellow
if (Test-Path "$root\backend") {
    Start-Process pwsh -ArgumentList "-NoExit", "-Command", "`$Host.UI.RawUI.WindowTitle = 'UseStakly - Backend'; Set-Location '$root\backend'; cargo watch -x run"
} else {
    Write-Error "Dossier 'backend' non trouvé dans $root"
}

# 3. Frontend
Write-Host "[3/3] Frontend (npm run dev)..." -ForegroundColor Yellow
if (Test-Path "$root\frontend") {
    # On utilise npm car package-lock.json est présent
    Start-Process pwsh -ArgumentList "-NoExit", "-Command", "`$Host.UI.RawUI.WindowTitle = 'UseStakly - Frontend'; Set-Location '$root\frontend'; npm run dev"
} else {
    Write-Error "Dossier 'frontend' non trouvé dans $root"
}

Write-Host ""
Write-Host "Stack lance." -ForegroundColor Green
Write-Host "  Backend  -> http://localhost:4000"
Write-Host "  Frontend -> http://localhost:5173"
Write-Host ""
Write-Host "Pour arreter : fermer les deux fenetres PowerShell + 'docker compose down'."
