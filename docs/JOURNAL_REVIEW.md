# Captain's journal rebuild

## Review

The former logbook drew most information at 9–13 logical pixels, concatenated
up to thirteen voyage details into one line, and placed several lifetime and
operating summaries above the list without width bounds. Character clipping
hid career information. Filtering used the full archive count when advancing
pages, which could leave the requested offset beyond the filtered results.
The summary separator also sloped across the panel.

## Rebuilt experience

- Four visible pages: Voyages, Career, Operating Ledger, and Awards.
- A selectable voyage list retains the original run numbers and newest-first
  order. Direct site filters reset the page and selection; NEWER / OLDER stop
  at the filtered boundaries. The selected record always belongs to the
  visible page.
- Recovery and Preparation & Cover use individual labels and values. They show
  haul, contract, materials, return fuel, wreck condition, danger, clearance,
  planning, equipment, and insurance information, plus cleared section names.
- Career distinguishes lifetime figures from totals calculated from filed
  voyages. The ledger groups service, supplies, income, and equipment; haul
  valuation stays separate from recorded income.
- Awards lists all seven commendations with requirements and earned status.
- Empty archives and filters with no matches explain the next visible action.
- Toolkit text blocks constrain text to its assigned region. Body and metric
  labels use 18 logical pixels, metric values 25, and primary headings 28–34.
  The supplemental list outcome uses 17 pixels. Navigation controls are at
  least 44 logical pixels high; record touch areas are 60 pixels high.
- Authored journal copy lives in `assets/data/journal_ui.json`, loaded through
  the toolkit's labeled embedded-data loader. Navigation is transient UI state;
  existing persistent voyage and career data keeps its format.

## Verification

Five navigation regressions cover complete paging, filtered boundaries,
selection, empty/shrinking archives, and tab/subpage transitions. The full Cargo
test suite, source-size gate, formatting, and strict all-target Clippy pass.

Thirteen scenes were captured and visually inspected directly in
`docs/verification/`: the voyage record, preparation, insurance claim, older
page, site filter, career, filed totals, three ledger pages, awards, empty
archive, and no matches. Voyages, preparation, career, and awards were also
inspected at a 960 by 540 requested window size before restoring normal
captures. The shared 1280 by 720 logical canvas scales with the game; this does
not add a separate portrait layout. Verification uses deterministic native
captures and tests, not a manual browser playthrough.

The parameterless publisher builds Windows and WebGL releases, packages them,
and deploys to Preview. Deployment succeeded. Project Roost tracking emitted a
warning because its local service at 127.0.0.1:80 refused the connection.
