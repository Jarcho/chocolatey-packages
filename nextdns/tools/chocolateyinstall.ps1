$ErrorActionPreference = 'Stop';
$toolsPath = Split-Path $MyInvocation.MyCommand.Definition
$url32 = 'https://nextdns.io/download/windows/stable'
$checksum32 = '7e6f1f73fd290083ff31202287c68dbc80865bb64f7bc58e9fd0b3e14c211ce7'

$cert = Get-ChildItem Cert:\CurrentUser\TrustedPublisher -Recurse | Where-Object { $_.Thumbprint -eq '5E66E0CA2367757E800E65B770629026E131A7DC' }
if (!$cert) {
    Start-ChocolateyProcessAsAdmin "certutil -addstore 'TrustedPublisher' '$toolsPath\openvpn.cer'"
}

$packageArgs = @{
    packageName    = 'nextdns'
    fileType       = 'exe'
    silentArgs     = '/S'
    validExitCodes = @(0)
    url            = $url32
    checksum       = $checksum32
    checksumType   = 'sha256'
}

Install-ChocolateyPackage @packageArgs
