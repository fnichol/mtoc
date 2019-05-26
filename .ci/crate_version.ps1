Param(
    [Switch]
    $help,

    [parameter(Position=0)]
    [String[]]
    $crate
)

function Write-Help {
    Write-Host @"
$program

Prints latest version of a Cargo crate

USAGE:
    $program [FLAGS]

FLAGS:
    -help Prints this message

"@
}

function main() {
    $script:program = "crate_version"

    if ($help) {
        Write-Help
        exit
    }

    Get-CrateVersion
}

function Get-CrateVersion {
    (cargo search --limit 1 --quiet "$crate" | Select-Object -First 1).
        Split('"')[1]
}

main
