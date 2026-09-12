# Salvage Captain TODO

Standards-alignment review: 2026-09-12

## Remaining

- [ ] Refactor the remaining functions over the §4.1 100-line maximum into
  cohesive helpers, including `data::validation::validate`,
  `validate_saved_runtime`, `finish_packing`, and the larger UI renderers
  (`draw_ship_with_selection`, `draw_site_card`, `draw_hold_panel`,
  `draw_travel`, `draw_debrief`, and `draw_target_panel`).

## Completion checks

- [ ] Run both release targets (Windows and `wasm32-unknown-unknown`) and
  `./publish.ps1` with no parameters after the remaining structural work;
  record any environment blocker.
