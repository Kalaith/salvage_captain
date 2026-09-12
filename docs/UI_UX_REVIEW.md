# UI/UX review and salvage redesign

## Shared ship chrome

The Port telemetry bar is now the shared visual shell for the complete game:
Main Menu, wreck selection, outbound and return transit, salvage workspace,
packing, debrief, pause, settings, and the Port all use the same 68-pixel
resource header. It keeps the SC-07 identity, credits, fuel, hull, materials,
market cycle, navigation, and menu affordances in stable positions while each
screen retains its own scene controls and artwork. Shared steel surfaces,
cyan/amber signal lines, restrained button states, and the operational footer
keep overlays and page transitions in the same visual language.

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

## Implemented next: wreck selection

Three large side-on wreck illustrations now lead the briefing. Each card shows
only its name, planned route danger, required fuel including return reserve,
and likely salvage. The entire card is a touch target with a clear selected state.
The wreck silhouettes are shared with flight destinations.

One shared inspector shows the selected contract and its actual reward including
standing and streak bonuses. Plan, Crew and Buy intel are secondary controls;
insurance and private haul are choices feeding one consistently placed Depart
button. Insufficient fuel or insurance funds visibly disable departure. Finished
or failed contracts and tired crew have explicit states. Details holds route risk,
crew readiness, recovery progress and prior voyage value. Selection does not start
an expedition or alter the active destination until departure.

The old overlapping footer is removed; interaction messages occupy a dedicated
strip above the cards. Fixed-size toolkit body text replaces the dense pixel copy.

## Implemented next: port

The large workboat remains in its hangar, alongside three functional sidebar
tabs: Service, Equipment, and Crew. Service owns fuel, the three repair scopes,
field cells, and refining. Equipment presents one module inspector, four stock
choices per page, and saved loadouts within the same rail. Selecting a ship
mount opens Equipment; selecting stock previews it before a separate purchase.
Crew owns role assignment, readiness, rest, and training.

Browse Wrecks stays anchored below every preparation view, including loadouts.
The header, cargo strip, inspector, and crew/loadout copy use readable toolkit
text. Interaction messages have a dedicated area above the ship. Disabled
service and equipment controls show the relevant quote or reason; refueling
uses the same authoritative quote for its displayed price and transaction,
including partial fills when credits are low. The cargo map suppresses ship
mount interactions while open.

Port now shares the game's 1280 by 720 logical canvas, preserving the full
composition when a browser window is smaller. Dedicated portrait reflow remains
separate work.

## Verification

Validation completed with 226 passing tests, strict all-target Clippy, and the
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

Wreck selection adds five regression tests for destination choices, mutually
exclusive private haul and coverage, details/selection state, authoritative
fuel and insurance checks, and disjoint card touch areas. Sixteen selection
captures cover all wrecks, preparation, Details, low fuel/credits, insurance,
private haul, tired crew, contract completion/failure and progression. Default,
Details, low-fuel and recovery screens were also inspected at 960 by 540.
Military and research travel captures were refreshed for the shared hull art.

The required parameterless `publish.ps1` completed successfully for this change:
Windows release, WebGL release, packaging and Preview deployment all passed.

Port adds five regression tests covering separated tab/content/departure regions,
complete bounded stock pagination, exact full/partial refuel transactions,
full-tank/low-credit rejection, and large credit balances. All 231 tests, strict
all-target Clippy, formatting, and the 800-line source gate pass. Twenty-two
port captures were refreshed or added directly in `docs/verification/`. Service,
last-page Equipment, tired Crew, Loadouts, and Cargo Map were also inspected at
960 by 540 before restoring their normal captures. Verification is capture-based,
not a manual end-to-end browser playthrough.

The parameterless `publish.ps1` completed successfully for the port change:
Windows release, WebGL release, packaging, and Preview deployment all passed.
