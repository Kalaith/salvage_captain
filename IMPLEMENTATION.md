# Salvage Captain — Implementation Document

## Purpose and status

This is the canonical implementation document for Salvage Captain. It
consolidates the gameplay design, visual identity, current implementation
plan, project development guidance, toolkit constraints, and the current
verification evidence into one build reference.

The project already contains a playable mechanical MVP. The work described
here is therefore a visual-identity implementation and controlled expansion,
not a replacement of the salvage economy or packing rules. The existing
engine remains the authority for fuel, risk, packing, disposition, upgrades,
save/load, and deterministic tests.

The product target is:

> A detailed 2D industrial space-salvage game where a tiny working vessel
> crawls around enormous derelicts, scanning, cutting, and tractor-beaming
> valuable machinery until it becomes a mobile salvage industry.

## 1. Reviewed baseline

### 1.1 What exists now

The current crate is a Rust 2021 Macroquad game using the shared
`macroquad-toolkit`. It already has:

- A `Game` coordinator with explicit `GameState` and `StateTransition` flow.
- Port, site selection, salvage packing, results, and pause states.
- Data-driven configuration in `assets/data/` for the economy, sites,
  salvage objects, modules, and texture references.
- A 5×5 ship grid shared by installed modules and temporary salvage.
- Fuel costs, safe-return checks, refuelling, repairs, seeded risk outcomes,
  repeat-visit condition values, and progression thresholds.
- Sell, install, break-down, discard, and leave-behind decisions.
- Versioned persistence and validation for layouts, references, and saves.
- Separate engine modules for packing, expedition, economy, installation,
  risk, and progression.
- Touch/mouse-capable visible controls and a capture-aware verification
  harness with captures for Port, site selection, packing, results, and pause.

### 1.2 What the current presentation proves

The existing captures prove that the mechanical loop is legible at the
logical 1280×720 layout. They do not yet prove the intended visual fantasy.
The current presentation is a dark, compact management UI with a ship grid and
text panels. It does not yet show:

- A small industrial ship beside a wreck that is visibly much larger.
- A side-on salvage workspace with section-scale camera framing.
- Travel as a progression showcase.
- Scanner pulses, tractor-beam extraction, cutting, towing, or stabilization.
- Salvage moving from wreck to ship or remaining attached externally.
- Drones as a visible progression system.
- Persistent visual changes to a wreck after parts are removed.
- A hangar or shipyard view where upgrades are represented by physical
  machinery.

The root `catalog_thumbnail.png` is still a generic toolkit-template capture.
It must be replaced with a title or Port capture once the visual foundation is
in place.

### 1.3 Decisions carried forward

- Keep the existing four-screen mechanical slice as a fallback and test path
  while the scene-based presentation is added.
- Keep the MVP economy readable: credits, fuel, Alloy, and Electronics.
- Keep the existing two-material mapping for Medical Supplies; its current
  Electronics yield resolves the earlier design mismatch.
- Keep mechanics simpler than the effects imply. A beam animation may suggest
  strain, mass, and risk while the authoritative result remains a deterministic
  engine operation.
- Do not add direct piloting, real-time dogfighting, crew simulation, or a
  procedural galaxy as part of this visual pass.

## 2. Experience loop to implement

The player should understand the following sequence after one expedition:

1. At the station, inspect the current ship, resources, damage, and available
   wrecks.
2. Choose a wreck using fuel cost, danger, known sections, and reward hints.
3. Watch the ship travel automatically toward the selected wreck.
4. Arrive at a camera-framed section of a derelict. The wreck fills much of
   the view; the player ship is small but readable beside it.
5. Scan the section to reveal candidate components and hazards.
6. Select a component and issue a visible command: `Scan`, `Extract`, `Cut`,
   `Tow`, `Stabilize`, or `Abandon` as appropriate to its state.
7. Resolve the extraction through a short, readable sequence. The item moves
   toward the ship, enters internal cargo, or attaches to an external clamp.
8. Pack recoverable items into the same physical ship space used by installed
   modules. Leave behind anything that is not worth the space, time, energy,
   or risk.
9. Return to the station and choose whether each recovered item is sold,
   installed, or broken down.
10. Repair, refuel, and visibly improve the ship before choosing the next
    wreck.

The player is an operator making extraction decisions. The ship repositions
and performs routine movement automatically; the game is not a flight
simulator.

## 3. Visual identity rules

These rules are acceptance criteria for every new screen, asset, and effect.

### 3.1 Industrial scale

- The player ship is an industrial work vessel: a tow truck, workboat, or
  mobile rig, never a sleek fighter.
- Early silhouettes are small, patched, asymmetrical, and visibly practical.
  Include tanks, clamps, antennae, exposed machinery, work lights, cables,
  and utility housings.
- Major wrecks are substantially larger than the ship. Do not routinely fit a
  whole capital ship inside the viewport.
- The active wreck section is the level. Bridge, Engineering, Reactor, Cargo,
  Habitation, Medical, and Command are navigable sections rather than only
  labels in a menu.
- Camera framing stays close enough to show hull detail, machinery, beams,
  sparks, damage, drones, and component movement.

### 3.2 Industrial science-fiction palette

Use a restrained dark palette with bright signals reserved for action:

| Token | Initial direction | Use |
| --- | --- | --- |
| Space | Near-black navy | Background and negative space |
| Structure | Dirty steel and blue-grey | Wreck and ship hulls |
| Work light | Warm amber/orange | Lamps, extraction progress, active tools |
| Scanner/tractor | Cyan/teal | Scans, targeting, beam paths, valid interaction |
| Warning | Red | Hazards, damage, instability, blocked actions |
| Safe/complete | Muted green | Successful resolution and safe return only |
| Text | Warm off-white with dim grey secondary text | Readability and hierarchy |

The current dark UI theme is a suitable foundation. Consolidate colors into
named visual-theme data or shared style constants rather than scattering new
color literals across screen modules. Critical information must never be
communicated by color alone; pair color with labels, shape, meter, or icon.

### 3.3 Screen composition

World scenes must preserve space for the ship, wreck, and extraction action.
Use layers in this order:

1. Space field, distant debris, and restrained parallax.
2. Wreck hull, section geometry, cables, openings, and damage.
3. Player ship and physical installed equipment.
4. Salvage targets, beams, drones, sparks, fragments, and extraction motion.
5. Temporary scan overlays, hazard markers, and target brackets.
6. Compact contextual panels, command buttons, resource strip, and notices.

Management screens may use larger panels, but the ship must remain visible in
the hangar or workshop whenever equipment is being compared or installed.

## 4. Target state and module architecture

Keep the existing named module layout and split new responsibilities before
any Rust file approaches the 800-line hard limit. UI reads state and returns
intents; engine services resolve rules and return explicit results.

### 4.1 State additions

Add these states only when their underlying interaction is functional:

- `Travel`: automatic ship transit and arrival briefing.
- `SalvageWorkspace`: camera-framed wreck section, scanning, target
  selection, extraction commands, hazards, and visible cargo transfer.
- `Hangar` or an expanded `Port`: ship presentation and physical module
  installation view.

Retain `SalvagePacking` as the reliable mechanical cargo-resolution state
until the workspace can exercise the same rules. The intended transition is:

```text
Port/Hangar
    -> Site Selection
    -> Travel
    -> Salvage Workspace
    -> Packing / Return
    -> Results / Disposition
    -> Port/Hangar
```

### 4.2 Recommended presentation modules

Use named files under the existing module roots:

```text
src/
  state/
    travel.rs
    salvage_workspace.rs
    hangar.rs
  ui/
    travel.rs
    salvage_scene.rs
    scan_overlay.rs
    extraction_panel.rs
    hangar.rs
    visual_theme.rs
```

If rendering helpers become substantial, extract a focused `render.rs` root
and child modules for world layers, ship rendering, wreck rendering, and
effects. Do not place simulation decisions inside these renderers.

### 4.3 Shared coordinate model

Use one camera/layout calculation for drawing, pointer hit-testing, target
selection, and ship placement. The logical design remains 1280×720, but all
scene layouts must be derived from the current viewport and safe margins.
Use `macroquad-toolkit` camera, pointer, timing, color, math, particle, and
capture helpers instead of project-local equivalents.

## 5. Data contract changes

Static definitions remain JSON under `assets/data/` and are loaded through the
toolkit data-loading path. Rust owns typed schemas and game-specific
validation. Avoid adding a project-local generic JSON loader.

### 5.1 Site and wreck data

Extend each site with data needed to stage a visual wreck without embedding
content rules in rendering code:

- Wreck class and visual theme.
- Background/hull asset references.
- Named sections and their connected neighbors.
- Candidate target IDs per section.
- Known information level and scanner reveal rules.
- Hazard tags and readable hazard descriptions.
- Persistent condition or removed-target keys.
- Optional arrival and departure text.

The three MVP sites remain visually distinct:

- `Merchant Wreck`: cleaner cargo geometry, warm utility lights, compact
  containers, and low hazard density.
- `Military Wreck`: fractured armour, exposed weapon or shield systems,
  harsher warning lights, and unstable sections.
- `Research Vessel`: laboratory structures, cyan instrumentation, unusual
  power signatures, and higher uncertainty.

### 5.2 Salvage target data

Retain the current economy and footprint fields. Add presentation and
extraction fields as the workspace is implemented:

- Mass class and visual silhouette/icon.
- Integrity and extraction difficulty.
- Extraction duration and energy cost.
- Required tool or module capability.
- Hazard type and warning text.
- Transfer mode: internal cargo, external clamp, or tow.
- Optional scan-only information and value range.
- Extraction animation profile.

The current `sale_value`, breakdown yields, installation module, and footprint
remain authoritative for post-expedition decisions.

### 5.3 Ship and module data

Extend module definitions with visual fields:

- Mount or attachment location.
- Hull silhouette contribution.
- Work-light, scanner, beam-emitter, drone-bay, clamp, or cargo-pod visual.
- External-cargo capacity and presentation rules.
- Scanner, tractor, drone, and stabilization effect descriptors.

The ship renderer must be able to show progression from one weak beam and a
small clamp to multiple emitters, external cargo mounts, scanner arrays,
drone bays, cargo pods, and stabilization equipment. Do not turn these into a
second abstract stat screen.

### 5.4 Visual theme and asset registry

Populate the currently empty texture manifest and asset registry with exact
references as art arrives. Add a small visual-theme definition for palette,
stroke widths, panel opacity, animation timings, and camera defaults. Missing
optional art must fall back to labeled geometric placeholders so the game
remains playable and publishing errors remain obvious.

## 6. Scene implementations

### 6.1 Port and hangar

The Port is the safe checkpoint and the quiet contrast to salvage operations.
Keep the existing readable resource strip and disposition controls, but add a
large side-on ship/hangar presentation.

Required behavior:

- Show the ship silhouette, installed modules, damage, clamps, tanks, and
  external salvage.
- Highlight the physical location affected by an install or removal choice.
- Keep Sell, Install, Scrap/Break Down, Repair, Refuel, Browse Wrecks, Save,
  and Load as visible tap/click targets.
- Show credits, fuel/capacity, hull, Alloy, and Electronics without requiring
  a submenu.
- Make the installed grid and the physical ship tell the same story.

Implementation approach:

1. Keep the current port controls and grid as the authoritative interaction.
2. Add a ship renderer beside or behind the grid.
3. Animate a selected module from storage to its mount when installation is
   accepted.
4. Replace the placeholder/catalog art only after the ship renderer is
   stable at multiple viewport sizes.

### 6.2 Site selection

Keep the three-card decision surface, but make it a mission briefing rather
than a generic menu. Each card should show a cropped wreck or section image,
fuel cost including safe-return buffer, danger, condition, known reward,
visited sections, and the reason the site is attractive.

The danger preview must be explicit. A seeded setback is acceptable; an
invisible failure is not. Scanner and hull modules should visibly improve the
briefing rather than only changing a hidden number.

### 6.3 Travel

Travel is an automatic short scene, not a loading spinner or a flight-control
segment.

Show:

- The current ship moving toward the destination.
- Destination and wreck class.
- Fuel before and after travel.
- A simple progress indicator and estimated arrival.
- Optional contract or briefing line.

Use the same ship renderer as the Port. Newly installed equipment, clamps,
cargo pods, and external salvage must be visible here. A deterministic
timeline is sufficient; the sequence should be skippable through a visible
`ARRIVE`/`CONTINUE` button so touch users are never trapped.

### 6.4 Salvage workspace

This is the primary visual implementation milestone. Frame one section of a
large wreck, with the player ship nearby and enough scale to show physical
work. The entire wreck stays off-screen; selecting a connected section pans
the camera or transitions to the next framed view while the ship automatically
follows.

World interaction:

- `SCAN` sends a pulse over the active hull section and temporarily reveals
  usable components, condition, approximate value, and hazards.
- Selecting a target opens a compact contextual panel with name, integrity,
  mass, value/range, extraction time, risk, and the next visible command.
- `EXTRACT`, `CUT`, `TOW`, and `STABILIZE` are shown only when valid for the
  target. `ABANDON` is always available when a target has been selected.
- A target that is too heavy, unsafe, or beyond current capability explains
  why in text and visual treatment.
- The ship remains visible while the selected target is worked whenever the
  framing allows it.

The first implementation may use authored rectangular target zones and a
small number of target nodes. The visual scene must not require a physics
simulation or free-form piloting.

### 6.5 Tractor-beam and extraction sequence

Use a short deterministic timeline with these phases:

1. Beam emitter aligns and establishes a cyan/teal connection.
2. Target vibrates; strained hull connections and warning marks appear.
3. Sparks, fragments, arcs, or dust sell the physical stress.
4. The component tears loose or is cut free.
5. It travels along a curved path toward the ship.
6. Internal cargo receives it, or an external clamp/pod secures it.
7. The result panel reports the authoritative outcome and remaining options.

Use toolkit timing, easing, particles, camera shake, and projectile/travel
helpers where they fit. The animation must not decide the outcome. The engine
resolves success, damage, lost salvage, emergency repair, or forced abandon
from the visible capability, danger, and seeded run state.

### 6.6 Scanning and hazards

Scanning should change what the player can read without permanently covering
the art in neon. Use a travelling pulse and temporary outlines/brackets.

The visual hazard vocabulary should include unstable fuel, electrical arcs,
radiation, spinning sections, unexploded ammunition, reactor instability,
structural collapse, magnetic interference, automated defenses, decompression,
and moving debris. Start with two or three hazards in the MVP scene and expand
only when their consequences are clear.

Hazard feedback must answer three questions immediately:

- What is dangerous?
- What tool or ship capability changes the risk?
- What will happen if the player continues?

### 6.7 Packing and return

The current 5×5 packing screen remains the authoritative fallback and should
be visually connected to the workspace:

- Items found in the scene appear as the same silhouettes/colors in cargo.
- A large item may be shown attached externally while still consuming the
  configured capacity or risk budget.
- The grid shows installed equipment and temporary salvage in one space
  budget.
- Placement failures explain bounds/overlap/insufficient capacity with a
  footprint ghost and text.
- `LEAVE`, `DROP`, `LEAVE ALL`, `PLACE`, `ROTATE`, and `RETURN WITH HAUL` are
  visible controls, not keyboard-only shortcuts.

### 6.8 Results and disposition

Keep the existing three-choice resolution, but show a small physical result
summary for each item: recovered into cargo, clamped externally, damaged,
lost, or abandoned. Install should preview the changed grid and ship
silhouette before confirmation. Sell and break-down should expose the exact
credit/material result immediately.

## 7. Progression presentation

Progression is visual first and numerical second.

### Early game: scrappy scavenger

- One weak, unstable tractor emitter.
- Basic scanner with limited reveal.
- Small cargo capacity and a few external clamps.
- Slow extraction and incomplete site information.
- Ship silhouette is visibly patched and under-equipped.

The first wreck should communicate that far more value exists than the ship
can currently take.

### Mid game: capable salvage ship

- Two tractor systems or a stronger emitter.
- Better scanner and several external cargo mounts.
- A visible drone bay and improved utility lighting.
- More reliable work in military or research sections.
- Wreck scenes support several simultaneous tools without becoming cluttered.

### Late game: mobile salvage platform

Arrival at a major derelict should show a coordinated operation:

1. The vessel stabilizes.
2. Work lights illuminate the wreck.
3. Scanner arrays deploy and wash across the hull.
4. Drone bays open and drones launch.
5. Tractor arrays rotate into position.
6. Cargo pods deploy and stabilization equipment anchors sections.
7. Dismantling begins.

This is still an industrial machine, not a battleship. Hostile vessels or
time pressure can create urgency, but they should not redefine the game as a
dogfight.

## 8. Persistent wrecks and save rules

Keep all existing safe-checkpoint behavior. Add visual persistence as a
versioned extension of the current save model.

Each persistent wreck should track:

- Exploration percentage.
- Known and unknown sections.
- Removed target IDs.
- Remaining valuable systems.
- Hazard/condition changes.
- Locked sections and the capability needed to access them.

When a target is extracted, the next visit must show a physical difference:
an empty mount, torn cables, a hollow engine bay, shifted debris, or a sealed
route. Returning later with stronger equipment should unlock previously
unrecoverable targets.

Migration requirements:

- Increase the save version explicitly when workspace and wreck fields are
  introduced.
- Migrate old MVP saves to a safe Port state with existing resources, layout,
  and site progress intact.
- Never restore an overlapping layout or an invalid target reference.
- Save at Port and other safe transitions, never halfway through a drag or
  extraction animation.
- Report corrupt or incompatible saves clearly and preserve the last safe
  state.

## 9. Asset and placeholder workflow

Build the visual system with placeholders first, then replace only the assets
that improve readability or identity.

### Required first-pass assets

- One player-ship side silhouette with module mount points.
- Three wreck section backdrops, one for each MVP site.
- A small target/salvage silhouette set for power, hull, cargo, navigation,
  scanning, and defense categories.
- Tractor emitter and beam variants.
- Scan pulse and target bracket effects.
- Clamp, cargo pod, drone, spark, arc, and warning markers.
- A title/Port capture suitable for `catalog_thumbnail.png`.

### Placeholder requirements

Procedural rectangles, silhouettes, lines, and labels are acceptable while
the engine is being proved. They must preserve the intended scale, contrast,
interaction states, and target readability. A missing optional texture should
fall back to an obvious labeled placeholder and should not prevent the game
from starting.

Every loaded texture must be represented in the asset registry/texture
manifest. Keep transparent sprites as PNG; use JPEG only for large opaque
backgrounds if the toolkit path and alpha audit permit it.

## 10. UI and accessibility requirements

- The wreck and ship remain the dominant visual content during salvage.
- Contextual panels appear for the selected target; avoid permanent walls of
  information.
- Scanner overlays are temporary and readable.
- Bright cyan, amber, red, and green states are paired with labels or shape.
- Selected, hovered, disabled, invalid, and completed states are distinct.
- Text remains readable at 1280×720 and common resizes.
- Mouse and touch can complete every tutorial, core interaction, and recovery
  action. Keyboard shortcuts are supplemental only.
- Any travel or extraction sequence has a visible continue, skip, or cancel
  path when waiting would otherwise trap the player.
- Tutorial copy names the visible control or direct gesture, for example
  `Tap SCAN`, `Tap EXTRACT`, or `Drag the footprint onto the grid`.
- No player-facing HUD or tutorial depends on a keyboard command alone.

## 11. Delivery phases and gates

### Phase 0 — Consolidated foundation

- Keep the existing mechanical MVP passing.
- Replace the stale implementation plan with this document.
- Keep `AGENTS.md` as workspace instruction; verification images remain
  evidence rather than design documentation.
- Confirm the current data and save model before visual work begins.

Gate: `cargo fmt --check`, `cargo test`, and `./publish.ps1` pass; existing
verification captures remain reproducible.

### Phase 1 — Visual foundation

- Add the visual theme tokens and shared scene layout helpers.
- Add a placeholder industrial ship renderer with mount points.
- Add a placeholder wreck renderer with section framing and camera pan.
- Replace the toolkit-template catalog thumbnail with a Salvage Captain
  title/Port capture.
- Extend capture scenes to cover the ship and wreck framing.

Gate: a resized window still keeps the ship, wreck, controls, and resources
readable; no interaction uses coordinates different from its rendering.

### Phase 2 — Travel and hangar presentation

- Add the automatic Travel state and skippable arrival sequence.
- Reuse the ship renderer in Travel and Port/Hangar.
- Show installed modules, external salvage, clamps, and equipment physically.
- Keep current Port actions authoritative while the hangar presentation is
  introduced.

Gate: a player can start a new game, choose a site, watch travel, arrive,
continue by touch, return to Port, and see ship changes persist.

### Phase 3 — Salvage workspace vertical slice

- Add one authored wreck workspace, preferably Merchant Wreck first.
- Implement scan pulse, target selection, contextual extraction panel, and
  one complete tractor-beam extraction sequence.
- Connect extraction results to the existing cargo, packing, risk, and results
  engines.
- Provide visible `SCAN`, `EXTRACT`, `LEAVE`, and `RETURN` paths.

Gate: a fresh run proves the full visual loop from Port to a large wreck,
including one successful extraction, one insufficient-capability or hazard
case, packing, disposition, and visible return to the station.

### Phase 4 — Risk, hazards, and site variety

- Add two or three readable industrial hazards.
- Add Military Wreck and Research Vessel visual profiles.
- Add section navigation and temporary scanner reveals.
- Show why ship modules change danger, extraction, or information.

Gate: the three sites are visually and mechanically distinct, and a player can
explain why a site or target is risky before committing.

### Phase 5 — Persistent wrecks and progression

- Persist sections, removed targets, and visible wreck condition.
- Add external salvage effects to travel, capacity, or risk where justified.
- Add recovered equipment that can be restored and installed later.
- Add first drone, scanner, clamp, and multi-beam progression visuals.

Gate: revisiting a wreck shows a physical change, and a stronger ship can
access a previously abandoned target without introducing a second opaque
economy.

### Phase 6 — Polish and release hardening

- Replace the highest-value placeholders with authored or generated assets.
- Tune beam, scan, sparks, debris, drone, and camera effects.
- Verify accessibility, touch interaction, save migration, and WebGL parity.
- Refresh all verification captures and catalog presentation.

Gate: `publish.ps1` passes from the project directory, native and WebGL builds
work, the capture set is current, and a first-time player can complete the
MVP loop without developer instructions.

## 12. Verification plan

### Automated tests

Maintain focused tests for:

- JSON loading, duplicate IDs, references, footprint validity, and starting
  layouts.
- Grid bounds, overlap, rotation, occupancy, and cargo abandonment.
- Fuel affordability, safe-return buffer, refuelling, and module effects.
- Sell/install/break-down resolution and installation/removal costs.
- Seeded risk outcomes and visible hazard capability modifiers.
- Section discovery, removed targets, and repeat-visit condition changes.
- Save/load round trips, migrations, invalid layouts, and bad references.
- Progression unlocks and external cargo/capacity rules.

Keep tests in separate child files using the existing named-module convention.
Keep every `.rs` file below 800 physical lines, including test files.

### Manual matrix

- Start a new game and complete the first safe return by mouse and touch.
- Attempt to depart without enough fuel for a safe return.
- Scan an unknown section and verify temporary reveals.
- Extract a small item, a large item, and an item that cannot currently fit.
- Trigger a hazard and verify that the warning identifies the consequence.
- Use `ABANDON` and return without corrupting cargo state.
- Install a module and verify grid, ship silhouette, Travel, and risk/readout
  changes.
- Sell, install, and break down at least one compatible item each.
- Revisit a partially explored wreck and verify visual condition changes.
- Save at Port, load, and verify resources, layout, and wreck progress.
- Resize the window and check camera, grid, contextual panels, and hit-tests.
- Run native and WebGL publishing and inspect the generated page.

### Capture set

Store captures directly in `docs/verification/`, replacing an existing image
when it represents the same state. The target set is:

- `ui_port.png`
- `ui_sites.png`
- `ui_travel.png`
- `ui_salvage_scan.png`
- `ui_salvage_extract.png`
- `ui_packing.png`
- `ui_results.png`
- `ui_paused.png`

The current captures may be retained as baseline evidence until their matching
visual states are replaced. Capture runs must use the shared toolkit harness,
not a separate project-local rendering path.

## 13. Definition of done

The visual implementation is complete when:

- The player operates a visibly industrial salvage vessel rather than a menu
  abstraction or combat craft.
- Wrecks read as enormous, sectioned places, with close camera framing and
  persistent physical changes.
- Travel showcases the current ship and is fully touch-completable.
- Scanning, selection, extraction, hazards, and abandonment are visible,
  understandable, and deterministic underneath the effects.
- Tractor beams are a signature interaction, and recovered objects visibly
  move into cargo or attach externally when practical.
- Station/hangar management keeps the ship visible and ties upgrades to
  physical locations.
- Early, mid, and late ship progression is visible without becoming a
  battleship progression track.
- The existing fuel, risk, packing, disposition, persistence, and progression
  rules remain testable and understandable.
- All three site types and the initial salvage/module libraries are data-driven
  and validated.
- Native Windows and WebGL publishing pass, captures are current, and the
  catalog thumbnail represents Salvage Captain rather than the toolkit.

## 14. Working rules for future changes

- Implement one complete interaction before adding decorative breadth.
- Prefer a small authored scene with a readable camera over a large empty
  background.
- Keep mechanics in `engine`/`state`; keep drawing and input intents in `ui`.
- Add data fields before adding code branches for content variation.
- Use toolkit helpers for shared loading, colors, math, timing, camera, input,
  effects, settings, persistence, and capture behavior.
- Treat every visible action as a touch action first; keyboard shortcuts may
  supplement it.
- Run formatting, tests, publishing, and relevant capture checks after each
  meaningful phase.
- When a file approaches 800 lines, split a cohesive responsibility before
  continuing.
