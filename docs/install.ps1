$ErrorActionPreference = "Stop"

$githubRepo = "DezBenedek/d"
$binaryName = "d.exe"
$assetName = "d-windows-x64.exe"
$downloadUrl = "https://github.com/$githubRepo/releases/latest/download/$assetName"
$installDir = "$env:LOCALAPPDATA\Programs\d"

New-Item -ItemType Directory -Force -Path $installDir | Out-Null
$destination = Join-Path $installDir $binaryName

Write-Host "Letoltes: $downloadUrl"
Invoke-WebRequest -Uri $downloadUrl -OutFile $destination

$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "User")
    Write-Host "Hozzaadva a PATH-hoz. Nyiss uj terminalt a hasznalathoz."
}

Write-Host "Telepitve: $destination"
