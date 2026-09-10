<#
.SYNOPSIS
    Headless screenshot harness for Salvage Captain.

.DESCRIPTION
    Thin wrapper around the shared macroquad-toolkit capture script. Builds the
    debug exe and drives it through the env-var capture hook
    (GAME_TEMPLATE_CAPTURE_*) provided by macroquad_toolkit::capture in
    src/main.rs. Named scenes reset runtime state before each capture.

.EXAMPLE
    ./scripts/capture_ui.ps1
    ./scripts/capture_ui.ps1 -Frames 60 -SkipBuild
#>
param(
    [string[]]$Scenes = @("port", "port_preview", "sites", "travel", "travel_cruise", "salvage_scan", "salvage_shift", "salvage_extract", "salvage_capture", "salvage_military", "salvage_research", "packing", "results", "paused"),
    [int]$Frames = 150,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$workspace = Split-Path -Parent $gameDir
if (-not (Test-Path (Join-Path $workspace "macroquad-toolkit"))) {
    $workspace = Split-Path -Parent $workspace
}
$shared = Join-Path $workspace "macroquad-toolkit\scripts\capture_ui.ps1"

& $shared -GameDir $gameDir -Prefix "SALVAGE_CAPTAIN" -Scenes $Scenes -Frames $Frames -OutputDir $OutputDir -SkipBuild:$SkipBuild
