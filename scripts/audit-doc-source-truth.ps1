#Requires -Version 5.1
<#
.SYNOPSIS
  Scan active docs for stale legacy references (snippet era, old CLI, migrations, MCP rate-limit wording).

.EXAMPLE
  .\scripts\audit-doc-source-truth.ps1
#>
$ErrorActionPreference = "Stop"

$patterns = @(
  "v0.1.3",
  "17 migrations",
  "read tools and protocol calls do not yet",
  "pas encore de rate-limit globale",
  "rate-limit handling (ETags",
  "computation priors dérivés côté events API",
  "Endpoint POST /api/snippets",
  "/api/snippets",
  "search_library",
  "get_snippet",
  "Project-DK/Project-K"
)

$activeRoots = @(
  "AGENTS.md",
  "CLAUDE.md",
  "GEMINI.md",
  "README.md",
  "TODO.md",
  "docs/README.md",
  "docs/source-of-truth.md",
  "docs/architecture-backend-current.md",
  "docs/mcp-protocol.md",
  "docs/trust-model-v1.md",
  "docs/tech-stack.md",
  "docs/ops-mcp-coolify-hardening.md",
  "docs/dev-workflow.md",
  "docs/security-audit-2026-04-21.md",
  "docs/audits/user-journey-audit-2026-04-23.md",
  "docs/plans/remaining-work-2026-05-03.md",
  "docs/validation/live-release-checklist.md"
)

$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

function Find-PatternInFiles {
  param(
    [string]$Pattern,
    [string[]]$Paths
  )

  $existing = $Paths | Where-Object { Test-Path $_ }
  if ($existing.Count -eq 0) {
    return @()
  }

  if (Get-Command rg -ErrorAction SilentlyContinue) {
    $output = & rg -n --fixed-strings $Pattern $existing 2>$null
    if ($LASTEXITCODE -eq 0) {
      return $output
    }
    return @()
  }

  $hits = @()
  foreach ($path in $existing) {
    $lines = Select-String -Path $path -Pattern ([regex]::Escape($Pattern)) -SimpleMatch
    foreach ($line in $lines) {
      $hits += "{0}:{1}:{2}" -f $line.Path, $line.LineNumber, $line.Line.TrimEnd()
    }
  }
  return $hits
}

$failed = $false

foreach ($pattern in $patterns) {
  $matches = Find-PatternInFiles -Pattern $pattern -Paths $activeRoots
  if ($matches.Count -gt 0) {
    $failed = $true
    Write-Host ""
    Write-Host "Potential stale documentation pattern: $pattern" -ForegroundColor Yellow
    $matches | ForEach-Object { Write-Host $_ }
  }
}

if ($failed) {
  Write-Host ""
  Write-Host "Documentation drift audit found potential stale active-doc references." -ForegroundColor Red
  exit 1
}

Write-Host "Documentation drift audit passed." -ForegroundColor Green
