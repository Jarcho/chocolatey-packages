Import-Module au

function global:au_GetLatest {
    $download_page = Invoke-WebRequest -Uri 'https://github.com/WerWolv/ImHex/releases' -UseBasicParsing
    $url32 = $download_page.links | ? href -Match 'Windows.Portable.zip$' | select -First 1 -expand href
    "$url32" -Match '/v(?<version>.*)/'
    return @{
        Version = $matches['version']
        URL32   = "https://github.com$url32"
    }
}

function global:au_SearchReplace {
    @{
        "tools\chocolateyInstall.ps1" = @{
            "(?i)(^\`$url32\s*=\s*)('.*')"      = "`$1'$($Latest.URL32)'"
            "(?i)(^\`$checksum32\s*=\s*)('.*')" = "`$1'$($Latest.Checksum32)'"
        }
    }
}

update
