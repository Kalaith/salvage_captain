# Salvage Captain TODO

## Remaining

## UI_STYLE review — 2026-09-20

Audit and planning only. Implement the tasks below in order of player impact,
respecting the stated dependencies. No UI implementation was changed by this
review. The existing TODO contained only its title and empty Remaining heading;
there were no tasks or completion checkboxes to merge or remove.

### Evidence and scope

- Read `AGENTS.md`, `UI_STYLE.md`, `CODE_STANDARDS.md`,
  `GAME_DEVELOPMENT_GUIDE.md`, `README.md`, and `IMPLEMENTATION.md` (the project's
  consolidated design/implementation document). No project-local
  `PROJECT_AGENTS.md` or separately named GDD was found. The local `UI_STYLE.md`
  matches the canonical `../rust_management/docs/UI_STYLE.md` by SHA-256.
- Inspected the phase dispatcher, shared header, logical viewport/pointer
  conversion, salvage layout and targeting, placement/inventory, wreck
  preparation, port drawers, transit, debrief, menus, and field-log code.
- Visually inspected existing images directly in `docs/verification/`:
  `ui_port.png`, `ui_port_services.png`, `ui_sites.png`, `ui_sites_balance.png`,
  `ui_travel.png`, `ui_salvage_capture.png`, `ui_salvage_scan.png`,
  `ui_salvage_generated_large.png`, `ui_salvage_details.png`,
  `ui_salvage_placement.png`, `ui_salvage_log.png`, `ui_inventory_full.png`,
  `ui_results_many.png`, `ui_logbook.png`, `ui_main_menu.png`, and `ui_paused.png`.
  These are historical captures of mixed dimensions, not fresh runtime evidence.
  Some contain obsolete labels/layouts (e.g. RETURN TO HOLD in generated salvage,
  the older port behind Pause, and the earlier wreck-readiness copy). Findings
  below use screenshots only where current code corroborates the relevant trait.
- Read `docs/UI_UX_REVIEW.md`, `docs/JOURNAL_REVIEW.md`, and
  `docs/verification/port_density.md` as prior verification history, not as proof
  of this audit's browser/touch coverage. No game launch, new capture, minimum-size
  playthrough, or live touch test was performed. Those checks remain explicit
  follow-up work below. Builds/publishing are unnecessary for this TODO-only edit.
- Preserve the successful game-specific work: the large hangar, collapsible
  drawers, side-on salvage targets, one Depart control, on-demand Details,
  visible placement cancellation, and paged cargo/journal views. The game is
  substantially adapted beyond the template; do not replace it with a new
  dashboard or mistake intentional management comparisons for clutter.

### Verified findings and implementation tasks

- [ ] **UI-01 — Record current screen decisions and viewport contracts before recomposing screens.**
  **Scope:** `README.md`, `IMPLEMENTATION.md` §§3, 6, 10–11; all gameplay phases.
  **Observed:** README describes controls but declares no minimum canvas size or
  complete UI_STYLE §1 screen briefs. The implementation document still includes
  older MVP assumptions and conflicting prescriptions (permanent materials in
  §6.1 versus compact operational facts in §11.12); current code has moved on.
  An implementing agent could restore obsolete panels or remove working systems.
  **Change:** Write the seven-field screen brief for Port, wreck selection,
  transit, salvage, placement/inventory, and debrief, plus utility-overlay rules.
  Specify one dominant decision, supporting area, quiet utilities, contextual
  information, and the primary action/cost per state. Declare normal and minimum
  actual canvas sizes and portrait/embedded-browser behavior; use 1280×720 normal
  and 960×540 as the initial minimum validation candidate, not an already verified
  support promise. Reconcile obsolete design passages while retaining historical
  completion evidence in the existing review documents.
  **Accept/verify:** An agent can identify the intended 2–3 attention regions and
  visible touch path for every phase without consulting old screenshots. Check the
  briefs against the current dispatcher in `src/ui.rs` and actual browser canvas
  dimensions before accepting the declared minimum. This is the dependency for
  UI-02 through UI-04; do not spend this task on decorative styling.

- [ ] **UI-02 — Recompose salvage around the wreck, current operating limits, and the selected action.**
  **Scope:** `src/ui/header.rs::draw_standard_header/draw_salvage_header`,
  `src/ui/salvage_scene.rs`, `src/ui/salvage_scene/command_panel.rs`,
  `src/ui/salvage_scene/power_cycle.rs`, `src/ui/extraction_panel.rs`,
  `src/ui/scene_layout.rs`, `src/ui/section_nav.rs`.
  **Observed (code + scan/capture screenshots):** The bright shared header shows
  credits, Alloy, Electronics, market cycle, identity and status during extraction,
  but neither remaining workspace power nor hold capacity. The lower inspector
  shows a target's power cost without the available balance. An 838×144 inspector
  surface is rendered even with no target, filled with instructions and objective
  prose; section name is repeated below the section tabs. Header, section band,
  world, field controls and inspector compete for attention.
  **Change:** Keep operational fuel/hull and a concise power/cargo readout; remove
  market/material telemetry from salvage and expose it in preparation/economy
  views. Put remaining power and scan/pull/stabilization costs beside the relevant
  controls, including reset fuel cost and field-cell stock. Collapse the empty
  inspector to a brief first-use cue; show the full inspector only on selection.
  Consolidate the section identity and remove the FIELD CONTROLS heading and
  routine no-hazard prose where labels/state already suffice. Reclaim the space
  for the wreck and beam path. Keep Scan primary before discovery and the valid
  transfer action primary on selection; keep return, inventory, cancellation,
  shortages, capability requirements and consequential hazard warnings visible.
  **Accept/verify:** At 1280×720, 960×540 and 1920×1080, the wreck/selected target
  draws the eye first, one action area supports it, and utilities remain quiet.
  Before spending power the player can see cost and balance without opening a
  log. Exercise scan → select → place → pull, depleted power, full hold, stabilized
  and blocked targets, drones, cancellation and return using touch only. Check
  authored and four-target generated sections; selection targets must remain
  aligned after the layout/camera changes. Depends on UI-01; coordinate UI-03/07.

- [ ] **UI-03 — Replace whole-canvas shrinkage with usable compact layouts.**
  **Scope:** `src/game.rs::draw` virtual-UI setup, `src/ui.rs::UiContext/button`,
  `src/ui/scene_layout.rs`, port/transit layouts, site cards, placement, inventory,
  debrief, field log, journal, pause/settings.
  **Observed (code; small-device experience untested):** Rendering always uses
  1280×720 logical dimensions and fixed rectangles. A 44-logical-pixel port/menu
  target becomes 33 physical pixels at a 960×540 canvas; Inventory is only 30
  logical pixels high in the salvage command panel. Fixed virtual coordinates
  keep things in bounds but do not establish readable text or usable touch sizes.
  **Change:** Use shared toolkit viewport/layout/pointer helpers for a compact
  mode that collapses secondary sections and reflows inspectors before scaling
  text. Keep the world and main action visible; use explicit pages or bounded
  scrolling for collections. Establish a project target of at least 44 CSS-pixel
  touch hit areas and 16 CSS-pixel essential body text at supported sizes (these
  are acceptance targets for this work, not claimed numeric shared-doc rules).
  Enlarge the small Inventory and close controls. Define supported behavior for
  narrower/portrait canvases rather than silently shrinking the landscape screen.
  **Accept/verify:** Test actual browser canvases at 1280×720 and 960×540, desktop
  1920×1080, and 390×844 portrait plus 844×390 landscape probes; clearly document
  any unsupported orientations. Check 100% and 150% display scaling, embedded and
  fullscreen modes, long generated names, large balances, full hold, expanded
  inspectors and dense drawers. Tap grid corners/targets after resize and confirm
  render/picking agreement. Do not claim success from bounds tests or resized PNGs.
  Depends on UI-01/02 composition; reuse this layout work in remaining tasks.

- [ ] **UI-04 — Make wreck comparison and departure the focus; disclose optional preparation on demand.**
  **Scope:** `src/ui/site_cards.rs::draw_site_selection`,
  `src/ui/site_cards/{board,card,preparation,details}.rs`, selection/discovery JSON.
  **Observed (code + sites/balance screenshots):** A permanent explanatory strip,
  five-control discovery/archive/page toolbar, three illustrated cards and three
  preparation columns are visible together. Plan, Crew, intel, insurance and
  private haul all receive large controls on the first trip. The permanent
  discovery/gear-readiness lesson competes with comparing viable work.
  **Change:** Keep the wreck comparison dominant and one concise selected-wreck
  departure summary beside/below it. Move optional plan/crew/intel/coverage editing
  behind a visible Preparation control with a compact summary of active choices;
  put private-haul selection with the contract. Keep fuel including reserve,
  reward, contract-target accessibility, zero-access warning and selected coverage
  cost beside Depart even when preparation is closed. Quiet archive/pagination,
  omit irrelevant one-page paging controls, and introduce specialist discovery
  through contextual unlock information rather than a permanent tutorial line.
  Retain discoverable free local work and an explanation of locked specialist leads.
  **Accept/verify:** The player can compare wrecks and explain the selected trip's
  costs/risks before departure; changing preparation immediately updates the
  visible summary. Check starter, full board, empty archive, no-access, low-fuel,
  insufficient-insurance-funds, tired-crew and completed-contract scenes at normal
  and minimum sizes. Touch through discovery → selection → preparation → close →
  Depart and Archive → Active; no recovery route may disappear. Depends on UI-01/03.

- [ ] **UI-05 — Separate phase actions from utilities and remove irrelevant shared telemetry.**
  **Scope:** `src/ui/header.rs`, `src/ui/transit.rs::draw_header`,
  `src/ui/main_menu.rs`, `src/ui/salvage_items/manifest.rs`,
  `src/ui/decision_panel.rs`, site-selection header.
  **Observed (code + travel/inventory/menu screenshots):** The standard header
  carries the same market/material dashboard into transit, inventory and the
  main menu. PORT/BACK controls share the same boxed utility strip with the
  hamburger menu; inventory also provides BACK TO WRECK at the bottom. The main
  menu even renders an empty disabled navigation button through this shared shell.
  **Change:** Give each phase a minimal header using UI-01's relevant facts.
  Reserve the utility area for menu/help/log access; put phase advancement or
  return beside its associated gameplay area, visually separate from menu/save/
  exit/settings. Keep one clearly named Back to Wreck route in inventory and one
  primary Arrive/Continue/Dock route in transit; preserve any distinct supported
  abort/recovery path with an explicit label. Use a title-focused main-menu header
  with no empty controls. Keep sale/install balances and exact costs at debrief.
  **Accept/verify:** At normal/minimum sizes, a touch player distinguishes continuing
  play from opening utilities without reading a row of unrelated buttons. Check
  both transit legs, inventory, unresolved/empty debrief, main menu, Pause →
  Settings → Resume, and checkpoint Save/Load. Preserve action semantics and save
  availability; no confirmation or new abort behavior is implied by this task.
  Depends on UI-01/02; extend the phase-aware header rather than duplicate it.

- [ ] **UI-06 — Give transaction feedback a lifetime and make older field events retrievable.**
  **Scope:** `src/game/runtime.rs::note/update_runtime`, `src/game.rs`,
  `src/ui/port_panel/chrome.rs::draw_context_hint`, `src/ui/header.rs::draw_footer`,
  `src/ui/site_cards.rs`, `src/ui/workspace_log.rs::draw_log_entries/draw_log_summary`.
  **Observed:** Code retains `message` until replacement; port transactions remain
  visible while a drawer is open and can be suppressed by a hover hint. Selection
  and inventory/debrief reuse the same untimed message. Salvage notices already
  expire after five seconds, but the field log shows only the last nine events,
  with an “earlier events retained” label and no paging. The log screenshot also
  shows tiny 10–13-pixel text and two dense aggregate lines above bordered rows.
  **Change:** Distinguish transient success notices, actionable errors and current
  state in coordinator-owned feedback. Expire ordinary success messages; keep
  unresolved blockers beside the action until fixed/dismissed and retain meaningful
  outcomes in accessible history. Do not let hover erase a new transaction result.
  Add visible Older/Newer or scrolling and a local Close control to the field log;
  use readable wrapped event rows, move aggregate counters behind a Summary view,
  and remove redundant row boxes/banners. Preserve existing timed salvage feedback.
  **Accept/verify:** At normal/minimum sizes, repair/refuel/purchase/intel and cargo
  actions briefly announce the change, then leave updated state. Errors remain
  understandable and retryable. Generate over nine field events and use touch to
  retrieve an earlier loss/cancel/power-use event and return to play. Verify pause
  timing, long names, empty log and reduced motion; old messages must not reappear
  in unrelated phases. Depends on UI-03 for overlay sizing; do not change economy.

- [ ] **UI-07 — Replace persistent lessons and hover-only explanations with reopenable contextual help.**
  **Scope:** `src/game/prompts.rs`, `src/ui/extraction_panel.rs::draw_empty_target_panel`,
  `src/ui/salvage_items.rs::draw_hold`, `src/ui/port_panel/chrome.rs::draw_context_hint`,
  menu/help actions in `src/ui.rs`, related `assets/data/*_ui.json` and discovery copy.
  **Observed (code + scan/inventory screenshots):** No dedicated reopenable Help
  action exists; an empty target inspector repeats the scan/select lesson, and
  inventory repeats the MOVE instruction whenever cargo exists. Port checkpoint
  and market explanations are attached to hover hit areas, with no explicit tap
  disclosure in `draw_context_hint`. Repetition consumes space while touch users
  cannot deliberately reopen the same guidance.
  **Change:** Add a visible Help entry and short first-use/current-step cues for
  Scan → target → transfer → hold cell, return and cargo movement. Dismiss each
  completed step and allow reopening by touch. Put checkpoint wording with Save/
  Load and market details in a tap-accessible economy/preparation view; retain
  optional hover hints. Keep explanations for unfamiliar controls and blocked
  actions contextual, and do not remove essential placement instructions before
  the player learns that a cell tap commits the pull. Load new copy through JSON.
  **Accept/verify:** A fresh touch-only player can finish a first haul and later
  reopen help without hover/keyboard. A repeat run contains no completed tutorial
  paragraphs. Check help dismissal/reopening at normal/minimum sizes, no-save and
  existing-save states, placement cancel, inventory move/cancel, and locked actions.
  Depends on UI-02/04 composition; coordinate UI-06 so help and events are distinct.

- [ ] **UI-08 — Make hold placement visually primary and simplify inventory/debrief decision surfaces.**
  **Scope:** `src/ui/workspace_placement.rs`, `src/ui/ship_grid.rs`,
  `src/ui/salvage_items.rs` and `salvage_items/manifest.rs`,
  `src/ui/decision_panel.rs` and `decision_panel/manifest.rs`.
  **Observed (code + placement/inventory/results screenshots):** Placement uses a
  900×606 sheet but a 5×5 grid only about 290 pixels square; ROTATE gets the primary
  button tone although tapping the grid advances play. Inventory emphasizes MOVE
  on every row alongside repeated DROP/ROTATE buttons. Debrief devotes its upper
  area to separate settlement/return panels and repeats ownership/instructions in
  the banner, subtitle and rows, then highlights both Sell and Install on each row.
  **Change:** Enlarge/frame the placement grid as the main decision, subordinate
  Rotate and the detached footprint sample, and expose placement validity with
  text/shape as well as color. Keep Back to Target and no-cost cancel clear.
  In inventory, use compact selectable cargo rows and one contextual action set,
  retaining grid-number mapping, values, objective markers and a visible selection
  affordance. Put return risk/fuel/policy beside Return with Haul. In debrief,
  consolidate settlement into one concise summary, remove repeated “yours to use”
  and instruction prose, and use spacing/separators instead of nested cards.
  Preserve exact sale value, install price/prerequisite and breakdown yield; give
  equally valid disposition choices equal emphasis rather than inventing a default.
  **Accept/verify:** Placement is understood as the action, not rotation. Cargo
  decisions dominate the debrief, with at most one supporting outcome area. At
  normal/minimum sizes, tap valid/invalid/rotated placement, cancel, select and move
  cargo across pages, drop an item, change return policy, and resolve sell/install/
  breakdown choices. Cover full/empty holds, pending cargo, long labels, many/last
  result pages, low funds, failed contract and private haul. No cost or warning is
  lost by subtraction. Depends on UI-03/05/07; preserve current transaction rules.

### Further inspection required — do not treat these as visually proven defects

- [ ] **UI-09 — Verify camera framing and attention during live dense salvage before changing zoom.**
  **Scope:** `src/ui/scene_layout.rs::salvage_layout/target_rect`,
  `src/ui/salvage_scene.rs::draw_section_shift`, `src/ui/scan_overlay.rs`, wreck,
  drone and hazard visuals; `src/game/capture/scenes/salvage.rs`.
  **Evidence/gap:** Static captures show a useful wide cutaway; code fixes the
  wreck to 930×324 logical pixels and the ship to 330×172. Section “camera shift”
  draws a sweep over fixed geometry rather than moving a camera; revealed targets
  all pulse continuously. Screenshots cannot establish whether these effects
  distract, whether the large wreck feels section-sized, or whether dense targets
  remain easy to pick in motion. Do not add manual zoom merely to satisfy a checklist.
  **Action:** After UI-02/03, run authored merchant/military/research and generated
  four-target sections through arrival, scan, selection, extraction, drones and
  section transition at 1280×720 and the declared minimum. Check reduced motion and
  display scaling. If targets are too small, enlarge/reframe the active section
  using shared drawing/picking transforms; if pulsing distracts, settle unselected
  brackets after discovery and reserve motion for the current operation. Reconcile
  the documented camera description with the intended final behavior.
  **Accept/verify:** Ship, target and beam remain readable and selectable throughout
  the sequence, urgent warnings outrank ambience, and no more than 2–3 regions
  demand attention. Record observed results and captures; if no change is needed,
  close this task with evidence rather than implementing speculative zoom/panning.

- [ ] **UI-10 — Complete current native/WebGL/touch verification and refresh stale evidence.**
  **Scope:** `src/game/capture.rs` and capture scenes, `docs/verification/`, README
  viewport declaration and prior review notes; every changed screen above.
  **Gap:** Historical captures differ from current labels/layouts; prior reviews
  explicitly stop short of a browser playthrough. Small-screen overflow, touch
  release behavior, pointer occlusion in placement, overlay click-through, and
  portrait usability are not verified by this audit. Dense Equipment/Crew/Loadouts
  and all journal subpages also need current interactive coverage; do not assume
  the inspected Service/journal screenshots represent all of them.
  **Action:** Use the existing capture harness to refresh supported normal, dense,
  expanded, failure and first-use states, replacing matching captures directly in
  `docs/verification/`. Run the UI-03 size matrix on the actual browser canvas and
  touch path from New Game through preparation, travel, scan, selection, rotated/
  failed placement, extraction cancel, inventory, return, disposition, port service
  and Save/Load; inspect/dismiss Details, logs, Help, Pause and Settings. Verify
  transaction feedback after it expires. Open follow-ups only for reproduced gaps.
  **Accept/verify:** Report canvas sizes, input method, states, current screenshots
  and remaining limitations against every UI_STYLE §9 criterion. After each
  independently useful implementation change, run relevant existing regressions
  and parameterless `./publish.ps1`, reporting success or blocker. Preserve useful
  tests in `tests/` and the 800-line source limit. A build or capture-only check must
  not be reported as a touch pass. This task closes the audit's verification gaps;
  it does not require publishing this documentation-only commit.
