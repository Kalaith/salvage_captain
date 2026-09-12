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
    [string[]]$Scenes = @("port", "port_equipment", "port_equipment_last", "port_crew", "port_crew_low_funds", "port_crew_veteran", "port_grid", "port_damage", "port_crew_tired", "port_crew_training", "port_worn", "port_cargo_bay", "port_cargo_bay_low_funds", "port_services", "port_services_low_funds", "port_services_no_materials", "port_services_full_cells", "port_loadouts", "port_repair_low_funds", "port_repaired", "port_preview", "port_refinery", "logbook", "sites", "sites_military", "sites_research", "sites_details", "sites_insured", "sites_low_fuel", "sites_low_credits", "sites_tired", "sites_completed", "sites_failed", "sites_contract_streak", "sites_route_familiarity", "sites_crew_progress", "sites_crew_veteran", "sites_progress", "sites_private_haul", "travel", "travel_departure", "travel_approach", "travel_arrived", "travel_details", "travel_military", "travel_research", "travel_reduced_motion", "travel_paused", "return_departure", "return_approach", "return_arrived", "return_empty", "return_damage", "return_reduced_motion", "travel_cargo_bay", "travel_familiarity", "travel_cruise", "travel_scanner", "salvage_scan", "salvage_details", "salvage_route_familiarity", "salvage_power", "salvage_power_cell_log", "salvage_clearance", "salvage_scanner", "salvage_drones", "salvage_drone_orders", "salvage_drones_log", "salvage_stabilize", "salvage_stabilized", "salvage_log", "salvage_revisit", "salvage_notice", "salvage_hazard_notice", "salvage_shift", "salvage_arrival", "salvage_extract", "salvage_capture", "salvage_clamp", "salvage_tow", "salvage_military", "salvage_research", "packing", "packing_private_haul", "return_travel", "return_travel_private_haul", "results", "results_private_haul", "paused"),
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
