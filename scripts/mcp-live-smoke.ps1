#Requires -Version 5.1
<#
.SYNOPSIS
  Smoke-test the UseStakly Streamable HTTP MCP endpoint (read tools + optional write).

.EXAMPLE
  .\scripts\mcp-live-smoke.ps1 -Endpoint "http://127.0.0.1:4000/mcp" -Token "usk_..."

.EXAMPLE
  .\scripts\mcp-live-smoke.ps1 -Endpoint "https://api.usestakly.com/mcp" -Token $env:USESTAKLY_MCP_TOKEN
#>
param(
    [Parameter(Mandatory = $true)]
    [string]$Endpoint,
    [Parameter(Mandatory = $true)]
    [string]$Token,
    [string]$Repo = "vitejs/vite",
    [switch]$WriteSignal
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Split-Repo([string]$value) {
    $parts = $value.Trim().Split("/", 2)
    if ($parts.Count -ne 2 -or [string]::IsNullOrWhiteSpace($parts[0]) -or [string]::IsNullOrWhiteSpace($parts[1])) {
        throw "Repo must be owner/name (e.g. vitejs/vite)."
    }
    return $parts[0], $parts[1]
}

function Invoke-McpStep([string]$Label, [scriptblock]$Action) {
    Write-Host "  $Label..." -NoNewline
    try {
        & $Action | Out-Null
        Write-Host " OK" -ForegroundColor Green
    } catch {
        Write-Host " FAIL" -ForegroundColor Red
        throw $_
    }
}

function Send-McpRequest {
    param(
        [string]$Endpoint,
        [string]$Token,
        [hashtable]$Body,
        [string]$SessionId
    )
    $headers = @{
        Authorization = "Bearer $Token"
        Accept        = "application/json, text/event-stream"
    }
    if ($SessionId) {
        $headers["mcp-session-id"] = $SessionId
    }
    $json = $Body | ConvertTo-Json -Depth 10
    return Invoke-WebRequest -Uri $Endpoint -Method POST -Headers $headers `
        -ContentType "application/json; charset=utf-8" -Body $json -UseBasicParsing
}

function Parse-McpPayload([string]$Body) {
    $trimmed = $Body.Trim()
    if ($trimmed.StartsWith("data:")) {
        $line = ($trimmed -split "`r?`n" | Where-Object { $_ -match '^\s*data:\s*\{' } | Select-Object -First 1)
        if (-not $line) { throw "Unable to parse MCP SSE body." }
        $json = ($line -replace '^\s*data:\s*', '').Trim()
        return $json | ConvertFrom-Json
    }
    return $trimmed | ConvertFrom-Json
}

function Assert-McpOk {
    param($Response, [string]$Label)
    if ($Response.StatusCode -lt 200 -or $Response.StatusCode -ge 300) {
        $snippet = $Response.Content
        if ($snippet.Length -gt 200) { $snippet = $snippet.Substring(0, 200) }
        throw "${Label}: HTTP $($Response.StatusCode) $snippet"
    }
    $payload = Parse-McpPayload $Response.Content
    if ($payload.error) {
        $msg = if ($payload.error.message) { $payload.error.message } else { ($payload.error | ConvertTo-Json -Depth 5) }
        throw "${Label}: $msg"
    }
    return $payload
}

if ([string]::IsNullOrWhiteSpace($Token)) {
    throw "Token is required (usk_<64 hex> from /account or monitoring token)."
}
if ($Token -notmatch "^usk_[0-9a-fA-F]{64}$") {
    throw "Token must match usk_<64 hex>. Create one at /account."
}

$owner, $name = Split-Repo $Repo
$endpoint = $Endpoint.TrimEnd("/")
$script:SessionId = $null

Write-Host "UseStakly MCP smoke -> $endpoint (repo: $owner/$name)"

Invoke-McpStep "initialize" {
    $response = Send-McpRequest -Endpoint $endpoint -Token $Token -Body @{
        jsonrpc = "2.0"
        id      = 1
        method  = "initialize"
        params  = @{
            protocolVersion = "2025-06-18"
            capabilities    = @{}
            clientInfo      = @{ name = "mcp-live-smoke"; version = "1" }
        }
    }
    $script:SessionId = $response.Headers["mcp-session-id"]
    if (-not $script:SessionId) { throw "Missing mcp-session-id header on initialize." }
    Assert-McpOk $response "initialize" | Out-Null
}

Invoke-McpStep "search_github_repos" {
    $response = Send-McpRequest -Endpoint $endpoint -Token $Token -SessionId $script:SessionId -Body @{
        jsonrpc = "2.0"
        id      = 2
        method  = "tools/call"
        params  = @{
            name      = "search_github_repos"
            arguments = @{ query = "vite"; filter = "explore"; limit = 3 }
        }
    }
    $payload = Assert-McpOk $response "search_github_repos"
    $text = $payload.result.content[0].text | ConvertFrom-Json
    if ($text.count -lt 1) { throw "search returned zero results." }
}

Invoke-McpStep "get_repo_quality_context" {
    $response = Send-McpRequest -Endpoint $endpoint -Token $Token -SessionId $script:SessionId -Body @{
        jsonrpc = "2.0"
        id      = 3
        method  = "tools/call"
        params  = @{
            name      = "get_repo_quality_context"
            arguments = @{ owner = $owner; name = $name }
        }
    }
    $payload = Assert-McpOk $response "get_repo_quality_context"
    $text = $payload.result.content[0].text | ConvertFrom-Json
    if (-not $text.provenance.formula_version) {
        throw "Missing provenance.formula_version in context response."
    }
}

if ($WriteSignal) {
    Write-Host "WriteSignal: recording log_usage on $owner/$name" -ForegroundColor Yellow
    Invoke-McpStep "log_usage" {
        $response = Send-McpRequest -Endpoint $endpoint -Token $Token -SessionId $script:SessionId -Body @{
            jsonrpc = "2.0"
            id      = 4
            method  = "tools/call"
            params  = @{
                name      = "log_usage"
                arguments = @{
                    owner   = $owner
                    name    = $name
                    outcome = "build_success"
                }
            }
        }
        Assert-McpOk $response "log_usage" | Out-Null
    }
} else {
    Write-Host "Skipped log_usage (pass -WriteSignal to record a real signal)." -ForegroundColor DarkGray
}

Write-Host "MCP smoke passed." -ForegroundColor Green
