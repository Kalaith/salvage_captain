# Salvage Captain TODO

Standards-alignment review: 2026-09-12

Baseline: `cargo test` passes 212 tests and `cargo fmt -- --check` passes. Strict
Clippy currently reports 69 errors. No Rust source file is over the 800-line
hard limit, but several files and functions are already in the refactoring band.

## Highest priority

- [ ] Add `tests/code_standards.rs` with the shared `macroquad_toolkit::source_gate`
  check and an empty exception list. Keep the gate running under `cargo test` so
  the 800-line rule cannot regress silently.

- [ ] Introduce `src/lib.rs` as the testable library surface, then migrate all
  test modules and test-only helpers out of `src/**` into this crate's `tests/`
  directory. Preserve the current 212 behavioral cases while removing the
  `#[cfg(test)] mod tests` declarations from production modules. The current
  test sources are `src/**/tests.rs` and `src/**/tests/*.rs`; there is no crate
  `tests/` directory yet.

- [ ] Refactor every function over the §4.1 100-line maximum into cohesive
  helpers. The largest current offenders include `begin_capture_scene`,
  `apply_action`, `validate_saved_runtime`, `draw_ship_with_selection`,
  `data::GameData::validate`, `draw_site_card`, `draw_target_panel`,
  `draw_hold_panel`, `draw_travel`, and `finish_packing`. Do not suppress the
  standard or move a function without separating a real responsibility.

- [ ] Restore strict Clippy cleanliness with `cargo clippy --all-targets -- -D
  warnings`. The current failures include the unused
  `complete_site_contract` method, manual `Default` implementations, several
  too-many-argument APIs, unnecessary `map_or` calls, manual clamping and
  division rounding, needless borrows, useless integer conversions, identical
  conditional branches, and two test-style warnings. Use parameter structs or
  smaller helpers where the warning identifies a design problem; add an
  allowance only with a nearby explanation of why it is intentional.

## Structure and maintainability

- [ ] Split the files already at or above the §2.2 planning threshold before
  adding more behavior. Current counts are:

  - `src/data.rs` — 800 lines
  - `src/game/capture.rs` — 789 lines
  - `src/ui/salvage_scene.rs` — 784 lines
  - `src/game.rs` — 783 lines
  - `src/state/workspace.rs` — 779 lines
  - `src/state.rs` — 773 lines
  - `src/ui/salvage_items.rs` — 765 lines
  - `src/ui/wreck_visual.rs` — 745 lines
  - `src/ui/ship_visual.rs` — 737 lines
  - `src/ui/site_cards.rs` — 732 lines
  - `src/state/tests.rs` — 704 lines
  - `src/ui/travel.rs` — 697 lines
  - `src/state/workspace/tests.rs` — 675 lines
  - `src/ui/port_panel.rs` — 666 lines
  - `src/ui/decision_panel.rs` — 618 lines

  Split by domain responsibility using named module files. Migrating tests
  first will reduce the production-file counts for the affected state/data
  modules; remeasure after that migration and continue splitting any remaining
  file above 600 lines.

- [ ] Review the test suites against the §11.3 target of no more than five
  cases per major feature. The current broad suites needing responsibility
  splits or table-driven consolidation are `src/data/tests.rs` (22 cases),
  `src/state/tests.rs` (38), `src/state/workspace/tests.rs` (27),
  `src/state/ledger/tests.rs` (11), `src/state/career/tests.rs` (10),
  `src/state/crew/tests.rs` (8), `src/state/contracts/tests.rs` (8), and
  `src/engine/workspace/tests.rs` (8). Preserve useful regression coverage;
  do not delete tests just to meet the target.

- [ ] Add short `//!` module documentation to test modules as they are moved
  into `tests/`, keeping the existing production-module documentation intact.

- [ ] Add a project-local `README.md` or `PROJECT_AGENTS.md` for setup,
  controls, capture scenes, publishing, and project-specific conventions. Keep
  the shared `AGENTS.md`, `CODE_STANDARDS.md`, `MACROQUAD_TOOLKIT.md`, and
  `GAME_DEVELOPMENT_GUIDE.md` copies synchronized from `rust_management/docs/`.

## Completion checks

- [ ] Run `cargo fmt -- --check`, `cargo test`, strict Clippy, both release
  targets (Windows and `wasm32-unknown-unknown`), and `.\publish.ps1` with no
  parameters after the structural work. Record any environment blocker.
- [ ] Recheck the flat `docs/verification/` directory after UI changes and
  replace captures of the same screen/state instead of adding duplicates.

## Already aligned in this review

- JSON game data uses the toolkit embedded-data loaders and project-owned
  validation.
- Named module filenames are in use; no `mod.rs` files or ambiguous module
  pairs were found.
- Visible mouse/touch controls and state-specific prompts cover the primary
  gameplay flow and recovery paths.
- `publish.ps1`, `game_page.json`, `catalog_thumbnail.png`, and the generated
  WebGL layout convention are present; no root `index.html` was found.
