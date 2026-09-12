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

## Implemented next: travel and return

Both legs now use the same flight composition: a stable 470-pixel workboat,
parallax stars and debris, and a substantial destination approaching from the
right. Merchant, military, and research wrecks retain their visual identities;
the return destination is a lit industrial docking bay. Recesses, hull edges,
and foreground debris carry over the salvage screen's shallow depth.

The default HUD contains fuel, hull, cargo capacity, destination, progress, and
one primary action in a consistent position. Travel retains a compact objective
and danger readout, with operating-plan, crew, coverage, and contract details in
Route Details. Return keeps only the recovered count, settled estimated value,
and any relevant setback; the complete breakdown remains in the docking debrief.

Arrive and Dock remain available to skip the short automatic flights. Arrival
holds the scene until the player continues. Motion derives from the state's
elapsed time, so pause freezes the scene. Reduced Motion keeps the world and
ship stationary while progress and the action controls still work.

## Remaining screens

1. Wreck selection: lead with distinct silhouettes, danger, fuel, and reward.
   Put contract and preparation choices in one selected-site detail panel.
2. Port: retain the large ship and split the sidebar into Service, Equipment,
   and Crew. Anchor departure at the bottom of every view.

These screens still need their own layout constraints and visual verification.

## Verification

Validation completed with 221 passing tests, strict all-target Clippy, and the
800-line source gate. All 24 salvage captures were refreshed directly in
`docs/verification/`. Selected-target and Details screens were also inspected at
960 by 540; the normal captures use the 1280 by 720 logical layout.

Travel and return add five regression tests for stable composition, control
boundaries, reduced motion, settled cargo quotes, and empty/legacy returns.
Twenty-one flight captures cover departure, cruise, approach, arrival, route
details, private haul, damage, empty cargo, pause, and reduced motion. Route
Details and return screens were also checked at 960 by 540.

The normal debug executable was running and locked on Windows, so these tests
and captures used the explicit `x86_64-pc-windows-msvc` target build. Strict
all-target Clippy passed on the default target. The default publisher builds
Windows and WebGL releases and deploys to Preview.
This is capture-based visual verification plus build/tests; it does not claim a
manual end-to-end browser playthrough. Dedicated portrait reflow remains a
separate improvement.
