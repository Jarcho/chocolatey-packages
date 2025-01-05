import-module au

function global:au_GetLatest {
  $download_page = Invoke-WebRequest -Uri 'https://www.videohelp.com/software/madVR'
  $url     = $download_page.links | ? href -match 'madVR[0-9]{5}.zip$' | select -First 1 -expand href
  $file    = Split-Path -Path "$url" -Leaf
  $version = "$($file.substring(5, 1)).$($file.substring(6, 2)).$($file.substring(8, 2))"
  return @{ Version = $version; URL32 = $url }
}

function global:au_BeforeUpdate() {
  $headers = @{
    Referer = 'https://www.videohelp.com/software/madVR'
  }
  $fn = [System.IO.Path]::GetTempFileName()
  Invoke-WebRequest $Latest.Url32 -OutFile $fn -Headers $headers

  $Latest.Checksum32 = Get-FileHash $fn -Algorithm sha256 | % Hash
  rm $fn -ea ignore
}

function global:au_SearchReplace {
  @{
    "tools\chocolateyInstall.ps1" = @{
            "(?i)(^\`$url32\s*=\s*)('.*')"      = "`$1'$($Latest.URL32)'"
            "(?i)(^\`$checksum32\s*=\s*)('.*')" = "`$1'$($Latest.Checksum32)'"
    }
  }
}

update -NoCheckUrl -ChecksumFor none
