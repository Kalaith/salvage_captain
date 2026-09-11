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
    [string[]]$Scenes = @("port", "port_damage", "port_crew_tired", "port_worn", "port_services", "port_services_low_funds", "port_loadouts", "port_repair_low_funds", "port_repaired", "port_preview", "port_refinery", "logbook", "sites", "sites_progress", "travel", "travel_cruise", "travel_scanner", "salvage_scan", "salvage_power", "salvage_power_cell_log", "salvage_clearance", "salvage_scanner", "salvage_drones", "salvage_drone_orders", "salvage_drones_log", "salvage_stabilize", "salvage_stabilized", "salvage_log", "salvage_revisit", "salvage_notice", "salvage_hazard_notice", "salvage_shift", "salvage_arrival", "salvage_extract", "salvage_capture", "salvage_clamp", "salvage_tow", "salvage_military", "salvage_research", "packing", "return_travel", "results", "paused"),
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
