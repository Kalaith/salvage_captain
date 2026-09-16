# Early-game wreck balance

New discoveries use per-class budgets instead of guaranteeing only one easy
entry target and filling every other mount without an access budget.

| Wreck | Total salvage targets | Starter gear can attempt |
| --- | --- | --- |
| Local merchant | 4–6 | 2–4 |
| Military specialist | 5–6 | 2–3 |
| Large research specialist | 6–8 | 1–2 |

These are individual equipment-compatible targets across the wreck, not a
guaranteed haul in one visit. Section access and target tractor requirements
count toward the budget. Power, cargo space, hazards and return risk still
apply. Local objectives retain a compact, hazard-free first target. Specialist
objectives can require upgrades and now warn about that before departure.

The budgets are measured against the authored starter fit. They do not rise as
the captain upgrades. Fitting the Heavy Tractor, Shield Module and Scanner
Module opens all equipment gates in the generated roster. Damaged equipment
is excluded from the readiness preview. The three original wrecks and existing
saved snapshots keep their contents.

A board filled with unfinished upgrade projects can accept a free local lead
when no accessible merchant salvage remains and the tractor still has power.
This preserves old wrecks while preventing the board limit from stopping early
income. The next lead closes this exception until its accessible work is gone.

Rarity is deferred. Integrity already varies item value, and unstabilized
hazardous salvage already allows risky attempts when the required equipment is
present. Randomly bypassing missing heavy-lift equipment would weaken the
upgrade decisions without addressing the original content distribution.

## Verification

- All 274 integration tests pass, including the 800-line source gate.
- Five new balance tests cover 450 generated wrecks, actual extraction gates,
  upgrades and damage, warnings after removal/reload, full-board recovery, and
  invalid tuning. All configured count values appear in the sampled seeds.
- Existing discovery persistence, transactional operations, duplicate-item
  recovery and board navigation coverage is preserved.
- Formatting and all-target Clippy with warnings denied pass.
- Headless screenshots inspected at desktop resolution:
  [mixed board](verification/ui_sites_balance.png),
  [no-access warning](verification/ui_sites_no_access.png),
  [four salvage mounts](verification/ui_salvage_generated_large.png),
  [local discovery](verification/ui_sites_discovery.png), and
  [details](verification/ui_sites_details.png).
- The default publisher builds native and WebGL releases and deploys to Preview.
  Project Roost tracking is unavailable because its local API refuses the
  connection; this does not prevent the Preview files or catalog from deploying.
