param(
  [string]$Api = "http://127.0.0.1:4000",
  [string]$Input = "docs/corpus/awesome-candidates-approved.json",
  [int]$Limit = 500,
  [int]$DelayMs = 750,
  [switch]$DryRun
)

$ErrorActionPreference = "Stop"
$raw = Get-Content -Raw -Path $Input | ConvertFrom-Json
$candidates = @($raw.candidates)
if ($candidates.Count -eq 0) {
  $candidates = @($raw)
}

$seen = @{}
$queue = [System.Collections.Generic.List[object]]::new()
foreach ($c in $candidates) {
  $slug = if ($c.key) { $c.key } else { "$($c.owner)/$($c.repo)".ToLower() }
  if ($seen.ContainsKey($slug)) { continue }
  $seen[$slug] = $true
  $queue.Add([pscustomobject]@{
    slug = $slug
    repo = if ($c.url) { $c.url } else { "https://github.com/$slug" }
  })
}

$stats = @{ added = 0; alreadyIndexed = 0; failed = 0; dryRun = [bool]$DryRun }
$results = [System.Collections.Generic.List[object]]::new()
$max = [Math]::Min($Limit, $queue.Count)
$rateLimitHits = 0

for ($i = 0; $i -lt $max; $i++) {
  $item = $queue[$i]
  Write-Host "[$($i + 1)/$max] $($item.slug)"
  if ($DryRun) {
    $results.Add([pscustomobject]@{ slug = $item.slug; status = "dry_run" })
    continue
  }
  try {
    $resp = Invoke-RestMethod -Method Post -Uri "$Api/api/repos/add" `
      -ContentType "application/json" `
      -Body (@{ repo = $item.repo } | ConvertTo-Json)
    if ($resp.alreadyIndexed) {
      $stats.alreadyIndexed++
      $status = "alreadyIndexed"
    } else {
      $stats.added++
      $status = "added"
    }
    $results.Add([pscustomobject]@{ slug = $item.slug; status = $status; artifactId = $resp.artifactId })
    $rateLimitHits = 0
  } catch {
    $msg = $_.Exception.Message
    if ($msg -match "rate limit|403|429") {
      $rateLimitHits++
      Write-Warning "Rate limit suspected ($rateLimitHits): $msg"
      if ($rateLimitHits -ge 3) {
        Write-Warning "Stopping after repeated rate-limit failures."
        break
      }
      Start-Sleep -Seconds 60
      $i--
      continue
    }
    $stats.failed++
    $results.Add([pscustomobject]@{ slug = $item.slug; status = "failed"; error = $msg })
    Write-Warning "Failed $($item.slug): $msg"
  }
  if ($i + 1 -lt $max) { Start-Sleep -Milliseconds $DelayMs }
}

$summaryPath = "docs/corpus/awesome-import-results.json"
$out = @{
  at = (Get-Date).ToUniversalTime().ToString("o")
  api = $Api
  input = $Input
  limit = $Limit
  stats = $stats
  results = $results
}
$out | ConvertTo-Json -Depth 6 | Set-Content -Path $summaryPath -Encoding utf8
Write-Host "Done. added=$($stats.added) alreadyIndexed=$($stats.alreadyIndexed) failed=$($stats.failed)"
Write-Host "Results: $summaryPath"
