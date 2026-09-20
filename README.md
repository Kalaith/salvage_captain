# Salvage Captain

Salvage Captain is a Macroquad workboat game about planning dangerous wreck
expeditions, recovering hardware, and keeping a patched ship operational.

## Setup

From this directory, install the Rust toolchain and the `wasm32-unknown-unknown`
target, then use Cargo for local checks. Game content is embedded from
`assets/data/`, so no runtime asset checkout is required.

## Controls

The game is touch-first: use the visible buttons and drag controls to choose a
wreck, plan the route, scan the workspace, choose each target's hold position before extraction, and
resolve returned cargo. Keyboard shortcuts are optional conveniences; the
screen always provides the equivalent visible control.

## Screen decisions and viewport contract

These briefs are the current composition contract. Resource readouts, drawers,
and overlays support the phase decision; they are not permanent competing
panels.

| Phase | Current decision and dominant focus | Primary action and cost | Supporting / deferred information | Touch path and feedback |
| --- | --- | --- | --- | --- |
| Port | Keep the workboat operational and choose the next operation; the hangar and the selected service, module, or crew member are dominant. | Service, install, train, or `BROWSE WRECKS`; show credits, fuel, hull, and material cost beside the chosen action. | Service, Equipment, Crew, Cargo Hold, Save/Load, and the journal are drawers or utilities. | Tap a dock drawer, inspect a visible item, then tap its labelled action; the affected mount, resource, and feedback update. |
| Wreck selection | Compare viable wrecks and decide whether the selected trip is worth its risk; the three wreck cards are dominant. | `DEPART`; keep fuel including the return reserve, reward, coverage cost, contract access, and zero-access warning beside it. | Plan, Crew, intel, insurance, private haul, Details, Archive, and discovery controls are secondary or on demand. | Tap a card, optionally open `PREPARATION`, close it, and tap `DEPART`; selection and validation remain visible. |
| Transit | Read the short automatic flight and choose when to enter the next phase; the destination and progress are dominant. | `ARRIVE`/`CONTINUE` outbound or `DOCK NOW` homebound; show the relevant fuel/haul consequence beside the action. | Route Details, objective, danger, and crew warnings are contextual. | Tap the one visible advancement control; progress, destination, and warnings persist until the next phase. |
| Salvage | Decide which revealed target to recover or leave; the wreck, workboat, and selected beam path are dominant. | `SCAN` before discovery, then the valid transfer action; show remaining power, target cost, hold/clamp capacity, and return beside the action. | Target Details, field log, drones, power recovery, and hazard calculations open on demand or sit in quiet command space. | Tap `SCAN`, a bracketed target, its transfer command, a hold cell, then `RETURN WITH HAUL`; the world and a retrievable notice confirm results. |
| Placement / inventory | Place a pending item in the hold or decide how secured cargo affects the return; the hold grid or selected cargo row is dominant. | Tap a valid grid cell to commit placement; `RETURN WITH HAUL` remains visible with return fuel, risk, and policy. | Rotate, Drop, page navigation, and cargo details support the selected item only. | Tap the grid or one contextual cargo action; valid/invalid shapes, labels, and cancellation remain visible without hover. |
| Debrief | Resolve each returned item while understanding the settled voyage outcome; cargo disposition is dominant. | `SELL`, `INSTALL`, or `BREAK DOWN`, with exact value, price, prerequisite, and yield beside each choice. | Contract result, return risk, voyage journal, and pagination support the decision. | Tap one equally legible disposition, then page or return to Port; the item row and totals update. |

The utility route is separate from phase advancement: menu, pause, settings,
save/load, help, log, Details, and journal controls never share the strongest
treatment with `DEPART`, `SCAN`, `EXTRACT`, placement, arrival, or disposition.
Overlays suppress gameplay input until their visible `CLOSE`, `BACK`, or
`RESUME` control is used.

The normal design canvas is 1280×720 logical pixels. The first minimum landscape
validation candidate is 960×540 actual canvas pixels; this is a validation target,
not a promise that the current build already supports it. The project target is
44 CSS-pixel touch areas and 16 CSS-pixel essential body text at supported sizes.
Portrait canvases such as 390×844 are a probe until compact layout work is
complete; the game must show explicit landscape guidance rather than silently
shrinking or clipping the landscape composition. Embedded browser canvases use
the same logical-to-actual viewport conversion as fullscreen WebGL.

Select a target and tap LOAD CARGO, LOCK CLAMP, or ENGAGE TOW. In CHOOSE HOLD
SPACE, tap a grid cell for its top-left corner; ROTATE changes orientation.
A valid placement starts the pull and successful recovery secures the item
there immediately. BACK TO TARGET cancels placement without spending power.
RETURN WITH HAUL starts the return burn directly. INVENTORY optionally opens
secured cargo for inspection or rearrangement; BACK TO WRECK resumes salvaging,
and RETURN WITH HAUL also works from inventory. Inventory shows cargo capacity,
a numbered hold layout, and paged cargo rows with values and MOVE / ROTATE / DROP
controls. Return risk, fuel cost, and policy remain available below the hold.

## Capture scenes and verification

The shared Macroquad capture harness selects scenes through the
`SALVAGE_CAPTAIN_CAPTURE` environment configuration. Captures belong directly
in `docs/verification/`; replace an existing capture when it represents the
same screen and state. The checked-in images cover the main menu, port,
workspace, inventory, return travel, results, settings, and key recovery paths.

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
scan, power recovery, drone orders, cancellation, inventory, and returning with the haul within
reach. See [the UI/UX review](docs/UI_UX_REVIEW.md) for the implemented scope and
the completed port preparation views.

## Flight presentation

Travel and return keep the workboat at a stable size while layered scenery and
the destination move past it. Use ARRIVE or CONTINUE to enter the wreck, or DOCK
NOW to reach the return debrief. ROUTE DETAILS holds the outbound preparation
summary; returned cargo values stay locked to their voyage quote. The pause and
Reduced Motion settings stop decorative movement without hiding the controls.

## Wreck selection and discovery

Browse Wrecks opens a paged board with six normal active slots. Find wrecks adds a
free local lead; Specialist coordinates cost 100 credits and unlock military
wrecks at 2 standing and research wrecks at 4. Easy local work remains available
at every standing. When no local contract target remains, returning to port or
opening the board adds a local lead if a board slot is available. If all remaining
local salvage needs upgrades, a free local lead may exceed the normal board limit
so unfinished upgrade projects cannot block earning money. Existing wrecks stay saved.

Each discovered ship has a permanent identity, a weighted cargo roster, varied
condition, route danger, fuel distance and item integrity. Local contracts target
starter-accessible hardware; specialist contracts can request equipment-gated
salvage. New local wrecks contain 4–6 targets with 2–4 accessible to starter gear;
military wrecks contain 5–6 with 2–3 accessible; large research wrecks contain 6–8
with 1–2 accessible. These budgets stay fixed as the ship improves, so upgrades
open more of each saved wreck. The three authored starter wrecks are unchanged.

Cards show how many remaining targets the fitted, working equipment can attempt,
including section access and tractor capacity. Zero-access trips have a red
"Nothing in reach · wasted trip" warning, and inaccessible contract objectives
are called out separately. Departure remains available for deliberate visits.
Counts describe individual gear-compatible pulls; energy, hold capacity, hazards
and return risk still determine the haul. The briefing labels the equipment list
"For full recovery" so partial recovery does not appear to require every upgrade.

Hazardous targets already permit attempts without stabilization when the tractor
and section requirements are met. Integrity varies and adjusts sale value. This
balance pass retains those systems rather than adding rarity gates or random
chances to bypass missing heavy-lift equipment.
Contract rewards account for travel, danger and extraction difficulty. Clients
pay once for the marked physical item and the captain keeps the hardware.

Scanning reveals saved section contents. Repeated items have separate mounts,
cargo positions and recovery records. Return trips preserve all progress; neither
rescanning nor reloading rerolls loot. Partly salvaged wrecks stay on the active
board. Depleted wrecks move to the read-only Archive and cannot consume departure
fuel. Their voyage records remain in the captain's journal.

Select a wreck card to open its preparation panel. Plan and Crew cycle assignments;
Buy intel improves the selected route. Insurance toggles coverage, and Private
haul declines the client contract. Depart checks fuel and coverage affordability.
Details reveals route risk, crew readiness, recovery progress and voyage history.
Previous / Next and Archive / Active wrecks are visible touch controls.

Saved games retain the three original wrecks and all existing progress. Generated
wreck snapshots and the discovery sequence are saved with the captain's session.
Use Save at the port checkpoint to preserve the board along with the ship.

## Yard debrief

Returning with a contract target completes its mission and pays the reward
automatically. No hand-in is required: the recovered hardware remains yours to
sell, install, or break down. The briefing and debrief both explain this rule.
The debrief shows installation costs and breakdown yields beside each choice;
PREVIOUS / NEXT page through larger hauls. VOYAGE DETAILS opens the journal
for the full return record, and CLOSE JOURNAL returns to the cargo decisions.

## Port preparation

The port opens onto a full-width orbital hangar. The bottom dock opens Service,
Equipment, and Crew drawers; tap the active dock control again or CLOSE to
return to the unobstructed ship view. The top bar shows live resources and the
market cycle. Cool steel panels and green departure controls also carry into
the other gameplay screens.

Service contains fuel, repair scopes, field cells, and the refinery. Equipment
contains a module inspector and paged stock; tap a ship mount or stock card to
inspect it before buying or removing. Locked stock cards show the credit milestone
for their research blueprint; the Heavy Tractor blueprint unlocks the heavy salvage
gates. Loadouts opens saved arrangements within
Equipment. Crew contains assignment, readiness, rest, and training. Browse
Wrecks remains at the bottom of every preparation view. CARGO HOLD opens the
physical cargo map and storage upgrade control. The port scales with the same
logical canvas as gameplay.

The `port_heavy_tractor` capture shows the locked blueprint and its research
milestone before the Heavy Tractor is installed.

The workboat uses layered procedural hull art with dedicated equipment mounts.
All nine module types have distinct machinery, empty hardpoints retain mounting
plates, and selecting uninstalled stock previews its silhouette on the ship.
Offline equipment keeps its shape with dark indicators and visible damage.
The belly rail carries external salvage clear of the side modules, and landing
struts retract outside port. `port_upgraded` and `port_upgraded_damage` capture
scenes show the complete fit alongside the starter `port_equipment` scene.

## Captain's journal

Tap LOG in port to open the journal. Voyages has direct site filters, a newest-first
list, and NEWER / OLDER controls. Tap a voyage to read its Recovery or Preparation
& Cover page, including cleared section names. Career separates lifetime figures
from filed voyage totals. Operating Ledger groups ship service, field supplies,
and income; Awards shows every commendation and its requirement. CLOSE JOURNAL
returns to port. The journal uses the shared logical canvas and visible touch
controls throughout.

See [the journal review](docs/JOURNAL_REVIEW.md) for the readability findings and
verification. Capture scenes beginning with `logbook` cover each page and the
empty, filtered, older-page, and insurance states.
