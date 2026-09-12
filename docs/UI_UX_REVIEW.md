# UI/UX review and salvage redesign

The feedback correctly identifies composition, hierarchy, and depth as separate
problems. Scaling every element would preserve the crowded hierarchy. A side-on
cutaway is a good match for the existing workboat and the extraction loop; a full
isometric conversion is unnecessary.

## Implemented first: salvage

The wreck spans 73% of the 1280 by 720 logical viewport, with the workboat at its
exposed edge. Recessed rooms, hull thickness, upper-edge highlights, object
shadows, and foreground plating establish a shared viewpoint. A subtle aiming
beam connects the selected object before the animated extraction begins.

A compact bottom inspector shows target value, power cost, mass, duration, danger,
and available actions. Specifications and risk calculations are in a Details
sheet. The main header retains fuel, hull, cargo, and power. Navigation, log,
field power, drone orders, cancellation, and return remain visible touch controls.
Body copy uses the toolkit font with fixed-size wrapping rather than smaller text.
Secondary buttons are quiet; scan and extraction use the primary accent.

The layout review also exposed overlapping hit areas for shield/crate targets on
the military wreck and sensor/navigation targets on the research wreck. Their
mounts now occupy distinct rooms; regression coverage checks every authored
section for overlapping targets and control boundaries.

## Remaining screens

1. Travel and return: hold the ship at a stable size, move scenery, and bring the
   destination into view. Keep destination, progress, and actionable events in
   the default HUD. Return needs a small cargo/value summary until docking.
2. Wreck selection: lead with distinct silhouettes, danger, fuel, and reward.
   Put contract and preparation choices in one selected-site detail panel.
3. Port: retain the large ship and split the sidebar into Service, Equipment,
   and Crew. Anchor departure at the bottom of every view.

These screens have not been redesigned in this change. Their crowded layouts
need separate constraints and visual verification, not just the salvage colours.

## Verification

Validation completed with 216 passing tests, strict all-target Clippy, and the
800-line source gate. All 24 salvage captures were refreshed directly in
`docs/verification/`. Selected-target and Details screens were also inspected at
960 by 540; the normal captures use the 1280 by 720 logical layout.

The default publisher builds Windows and WebGL releases and deploys to Preview.
This is capture-based visual verification plus build/tests; it does not claim a
manual end-to-end browser playthrough. Dedicated portrait reflow remains a
separate improvement.
