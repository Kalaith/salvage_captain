# Salvage Captain

Salvage Captain is a Macroquad workboat game about planning dangerous wreck
expeditions, recovering hardware, and keeping a patched ship operational.

## Setup

From this directory, install the Rust toolchain and the `wasm32-unknown-unknown`
target, then use Cargo for local checks. Game content is embedded from
`assets/data/`, so no runtime asset checkout is required.

## Controls

The game is touch-first: use the visible buttons and drag controls to choose a
wreck, plan the route, scan the workspace, extract targets, pack the hold, and
resolve returned cargo. Keyboard shortcuts are optional conveniences; the
screen always provides the equivalent visible control.

## Capture scenes and verification

The shared Macroquad capture harness selects scenes through the
`SALVAGE_CAPTAIN_CAPTURE` environment configuration. Captures belong directly
in `docs/verification/`; replace an existing capture when it represents the
same screen and state. The checked-in images cover the main menu, port,
workspace, packing, return travel, results, settings, and key recovery paths.

## Publishing

Run the project publisher without parameters after meaningful changes:

```powershell
.\publish.ps1
```

The publisher validates native and WebGL release builds and prepares the
generated `dist/webgl/` host. The root `catalog_thumbnail.png` is the catalog
image, and `game_page.json` owns the WebGL page metadata.

## Project conventions

Keep game rules in `state/` and `engine/`, immutable authored content in
`assets/data/`, and rendering in `ui/`. UI emits `UiAction` values; the `Game`
coordinator applies mutations and owns transitions. New Rust source files must
stay below the 800-line hard limit and use named module filenames.

## Salvage presentation

The workspace uses a wide side-on cutaway with a beam connecting the workboat to
the selected target. The bottom inspector keeps extraction decisions visible;
Details opens target specifications and risk calculations. Field controls keep
scan, power recovery, drone orders, cancellation, and return to the hold within
reach. See [the UI/UX review](docs/UI_UX_REVIEW.md) for the implemented scope and
the completed port preparation views.

## Flight presentation

Travel and return keep the workboat at a stable size while layered scenery and
the destination move past it. Use ARRIVE or CONTINUE to enter the wreck, or DOCK
NOW to reach the return debrief. ROUTE DETAILS holds the outbound preparation
summary; returned cargo values stay locked to their voyage quote. The pause and
Reduced Motion settings stop decorative movement without hiding the controls.

## Wreck selection

Select an illustrated wreck card to open its contract and preparation panel.
Plan and Crew cycle assignments; Buy intel improves the selected route.
Insurance toggles coverage, and Private haul declines the client contract.
Depart uses the selected options and checks fuel and coverage affordability.
Details reveals route risk, crew readiness, recovery progress and voyage history.

## Port preparation

Service contains fuel, repair scopes, field cells, and the refinery. Equipment
contains a module inspector and paged stock; tap a ship mount or stock card to
inspect it before buying or removing. Loadouts opens saved arrangements within
Equipment. Crew contains assignment, readiness, rest, and training. Browse
Wrecks remains at the bottom of every preparation view. View Grid shows the
physical cargo map. The port scales with the same logical canvas as gameplay.
