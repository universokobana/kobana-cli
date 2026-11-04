# azure-sign.ps1 — Authenticode-sign a file with AzureSignTool against the
# code-signing certificate in Azure Key Vault (kia-codesigning). Invoked by
# Tauri's bundle.windows.signCommand (injected in release.yml), one file per call.
#
# Why a wrapper (not AzureSignTool inline in signCommand): Tauri splits
# signCommand naively, spawns the child from the bundler's working directory
# (not the repo root), and swallows its stderr. A wrapper lets us (a) keep the
# Azure client secret out of tauri.conf.json and the CI log — it's read from the
# environment here, never passed as a logged argument — (b) resolve AzureSignTool
# robustly, and (c) tee the full output to azure-sign.log for diagnosis.
#
# Required env (set by the workflow from GitHub secrets):
#   AZURE_KV_URL         vault URL, e.g. https://kia-codesigning.vault.azure.net
#   AZURE_KV_CERT_NAME   certificate name in the vault (the EV code-signing cert)
#   AZURE_CLIENT_ID / AZURE_CLIENT_SECRET / AZURE_TENANT_ID   service principal
# Optional:
#   AZURE_SIGN_TS_URL    RFC3161 timestamp authority (default: DigiCert)
param([Parameter(Mandatory = $true)][string]$File)

$ErrorActionPreference = 'Stop'

foreach ($v in 'AZURE_KV_URL','AZURE_KV_CERT_NAME','AZURE_CLIENT_ID','AZURE_CLIENT_SECRET','AZURE_TENANT_ID') {
  if (-not (Test-Path "env:$v") -or [string]::IsNullOrWhiteSpace((Get-Item "env:$v").Value)) {
    throw "azure-sign: missing required env var $v"
  }
}

$ts = if ($env:AZURE_SIGN_TS_URL) { $env:AZURE_SIGN_TS_URL } else { 'http://timestamp.digicert.com' }

# Resolve AzureSignTool: PATH first, then the dotnet global-tools dir (not always
# on PATH on Windows runners).
$tool = (Get-Command AzureSignTool -ErrorAction SilentlyContinue)?.Source
if (-not $tool) {
  $candidate = Join-Path $env:USERPROFILE '.dotnet\tools\AzureSignTool.exe'
  if (Test-Path $candidate) { $tool = $candidate }
}
if (-not $tool) { throw "azure-sign: AzureSignTool not found on PATH or in ~/.dotnet/tools" }

& $tool sign `
  --azure-key-vault-url $env:AZURE_KV_URL `
  --azure-key-vault-certificate $env:AZURE_KV_CERT_NAME `
  --azure-key-vault-client-id $env:AZURE_CLIENT_ID `
  --azure-key-vault-client-secret $env:AZURE_CLIENT_SECRET `
  --azure-key-vault-tenant-id $env:AZURE_TENANT_ID `
  --timestamp-rfc3161 $ts `
  --file-digest sha256 `
  --verbose `
  $File 2>&1 | Tee-Object -FilePath azure-sign.log

if ($LASTEXITCODE -ne 0) { throw "azure-sign: AzureSignTool failed (exit $LASTEXITCODE)" }
Write-Host "azure-sign: signed $File"
