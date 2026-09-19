# 003 - Independent Salvage

Generated with the built-in image generation tool on 2026-09-19.
Promotional concept artwork featuring the game's patched workboat in close view.

## Generation prompt

Use case: ads-marketing
Asset type: full desktop wallpaper for the original game Salvage Captain.
Primary request: One finished 1920 x 1080 pixel landscape image, exact 16:9, full bleed. A ship-focused cinematic portrait entitled "Independent Salvage" for context only; do not write a title in the picture.
Subject: SC-07, a working salvage captain's compact patched industrial spacecraft, shown close enough to appreciate its practical engineering. It is a long LOW angular SLAB-SIDED workboat rather than a rounded rocket or sleek fighter. Raised forward wheelhouse near the right-hand bow, with a band of three cyan windows. Blunt tapered wedge bow and a small cyan tractor nozzle at the front. Exactly THREE compact engine pods vertically stacked at the left-hand stern, each with a soft amber exhaust aperture. Cool steel-grey plating with mismatched welded repairs, inset black service grilles, a few short orange hazard stripes, sparse scratches and exposed bolts. A low open rectangular utility gantry over the rear deck. Distinct modular side fittings: a strapped cylindrical fuel tank, rectangular battery enclosure, one circular cyan powered machinery module, and a folded small sensor dish. Parts have understandable scale and mechanical connections, with breathing room between modules.
Cargo: A continuous belly rail carries two recovered pieces of hardware: a compact rectangular industrial power unit and a scarred engine assembly. Each is firmly retained in a pair of visible rugged amber-painted clamps under the hull, clear of the side equipment and engines. No loose cargo or dangling cables. Landing struts folded fully against the hull. No deployed tractor beam.
Scene/backdrop: Open orbital night, mostly near-black navy starfield with sparse restrained stars. A very distant thin blue planetary crescent at the far right is only a quiet scale cue; no close planetary landscape filling the bottom half. No station and no other ships or wrecks. Give this useful little craft the entire scene.
Composition/camera: A dignified wide landscape ship portrait with the COMPLETE vessel visible and comfortable margins. Near-broadside three-quarter view from slightly below and aft, showing the visible flank, forward bridge, all three aft engine apertures and clamped belly cargo. The long workboat spans roughly 75 percent of image width across the middle and lower-right, bow gently rising toward the right. Nothing is cropped. Quiet dark upper-left quadrant for desktop icons, unbusy strip along the bottom for taskbar. Strong clearly readable industrial silhouette, not a spaceship technical sheet or product catalog.
Style/medium: Premium cinematic painted science-fiction concept art; finely rendered hard-surface engineering, subtle painterly edges, tactile materials and physically convincing structure. Maintain the Salvage Captain series language of worn navy/steel, amber work lights, cyan instrumentation. The mood is capable, resourceful, solitary and quietly proud.
Lighting: Cool rim light traces the gantry, bridge and upper hull. Soft neutral reflected light reveals the patched side plating and the belly cargo without flattening shadows. Amber engine glow and small service lamps; controlled cyan windows and emitter lenses. Selective sharp details and deep clean blacks, restrained bloom.
Constraints: Native output requested at 1920x1080, edge-to-edge 16:9. No lettering, numbers, logos, watermarks, UI, labels, captions, frames, montage, people, explosions, battle, guns, excessive exhaust trails, motion blur, bright nebulae or artificial space fog. Exactly three visible aft engine pods, bridge forward, attached belly cargo, complete unclipped silhouette.

## Source anchors

- `src/ui/ship_visual/hull.rs`: layered angular hull, three aft engine pods,
  forward cyan-window bridge, rear deck gantry, scratched plates, bow emitter.
- `assets/data/modules.json`: external fuel tank, battery, powered machinery,
  scanner and other independently fitted equipment.
- `src/ui/ship_visual/cargo.rs`: salvage carried beneath the hull on the
  external cargo rig.

## Review

The complete workboat fills the frame with three readable amber aft engines,
a raised forward bridge, a cyan bow nozzle and a clear patched-steel silhouette.
A strapped fuel tank, rectangular battery, circular powered module, dish,
service grilles and rear gantry give the craft specific industrial character.
Two recovered hardware units are visibly clamped beneath the hull. There are
no extended landing struts, text, UI or other large ships competing with it.
Quiet starfield above and below the vessel preserves usable desktop space.
The thin blue planetary crescent keeps the palette consistent while allowing
the ship, rather than scenery, to dominate this wallpaper.
Independent visual QA passed the ship identity, attached cargo, complete framing,
desktop breathing room and absence of unwanted text or major artifacts.

The generator returned 1672 x 941 pixels despite the native 1920 x 1080
request. The PNG is preserved as generated, with no resampling. It is not a
verified 1080p or 4K export; the earlier local-resizing choice remains pending.
