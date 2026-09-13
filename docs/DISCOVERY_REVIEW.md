# Renewable wreck discovery

Implemented on 2026-09-13. The original three wrecks remain compatible with old
saves; the captain can now discover additional physical ships throughout a run.

## Player loop

- Find wrecks supplies a free local merchant lead. Specialist coordinates cost
  100 credits and offer military wrecks at standing 2, plus research wrecks at 4.
- The active board holds six unfinished sites, paged three at a time. Finishing
  a wreck frees its slot. Partly salvaged sites retain their contents and routes.
- Returning to port or opening the board supplies a local lead when no local
  contract target remains and a board slot is available.
- Depleted sites appear in Archive. They retain recovery and voyage records;
  every departure mode rejects them before charging fuel or insurance.
- Contracts request an actual physical item. Local objectives fit starter
  equipment; specialist objectives vary and can need upgraded equipment.
- The contract briefing lists wreck equipment requirements and explains that
  the captain keeps the returned item after receiving the one-time bonus.

## State and content

`assets/data/discovery.json` owns weighted loot pools, standing thresholds,
board limits, generation ranges, reward tuning, and discovery interface copy.
Content loads through the toolkit and is semantically validated by the game.

The toolkit's `SeededRng` generates contents once at discovery. Each session owns
serialized wreck snapshots, a discovery seed, and a monotonically increasing
serial. Each salvage instance has a template reference, section and mount slot,
and its own immutable object snapshot. Integrity affects value and extraction
difficulty. Wreck condition, route danger and fuel distance also vary.

The data registries resolve those snapshots for the existing salvage, cargo,
contract, and journal systems. They are derived views; authored templates remain
unchanged. Loading validates instance identities and rebuilds the views before
checking active expeditions and cargo. Loading another save or starting a new
run clears previous derived entries. Legacy saves gain empty discovery state
without resetting their original site progress.

The first release uses recovery contracts and the existing authored section
layouts. Survey jobs, multi-item contracts, new hull classes and timed jobs are
future additions rather than requirements of the renewable loop.

## Verification

All 269 integration tests pass, including the source-size gate. Strict
all-target Clippy passes. Nineteen discovery and board regressions cover stable
generation, varied specialist objectives, standing and affordability, board
limits, archive navigation, old-save compatibility, duplicate-instance rejection,
and continuing through twelve new wrecks after exhausting all starter sites.

The duplicate-battery regression recovers two batteries independently, scans a
different section without removing its batteries, restores the active voyage
from a save, returns, receives the contract payout, and sells both cargo items.

The shared capture harness drove discovery and navigation actions and saved
the following inspected screens directly in `docs/verification/`:

- `ui_sites.png`: starter board with visible discovery controls.
- `ui_sites_discovery.png`: a newly discovered wreck and contract briefing.
- `ui_sites_board_full.png`: capacity reached and clear next-step guidance.
- `ui_sites_board_page.png`: second page and bounded navigation.
- `ui_sites_archive.png`: read-only depleted wreck record.
- `ui_sites_archive_empty.png`: empty archive and discovery instructions.
- `ui_salvage_generated.png`: repeated batteries on separate selectable mounts.

Both stages passed the default `publish.ps1`: Windows and WebGL releases built,
packages were produced, and Preview deployment and catalog synchronization
succeeded. Project Roost publish tracking was unavailable at localhost:80; this
did not block the builds or deployment.
