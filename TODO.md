# Salvage Captain TODO

Standards-alignment review: 2026-09-12

## Remaining

- [ ] Refactor the remaining functions over the §4.1 100-line maximum into
  cohesive helpers, including `begin_capture_scene`, `apply_action`,
  `validate_saved_runtime`, `finish_packing`, and the larger UI renderers.

- [ ] Split the remaining production modules at or above the §2.2 planning
  threshold (`data.rs`, `game.rs`, `state.rs`, `state/workspace.rs`, and the
  larger UI modules) by domain responsibility using named module files.

- [ ] Review the broad integration suites against the §11.3 target of no more
  than five cases per major feature. Preserve useful regression coverage while
  splitting by responsibility or consolidating table-driven inputs where that
  improves readability.

## Completion checks

- [ ] Run both release targets (Windows and `wasm32-unknown-unknown`) and
  `./publish.ps1` with no parameters after the remaining structural work;
  record any environment blocker.

- [ ] Recheck the flat `docs/verification/` directory after UI refactors and
  replace captures of the same screen/state instead of adding duplicates.
