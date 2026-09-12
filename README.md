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
