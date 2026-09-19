# 004 - Under the Broken Keel

Generated and refined with the built-in image generation tool on 2026-09-19.
Promotional concept artwork based on the authored military wreck.

## Generation prompt

Use case: ads-marketing
Asset type: cinematic desktop wallpaper for the original game Salvage Captain.
Primary request: Exactly one finished full-bleed 16:9 landscape image, requested native resolution 3840 x 2160 pixels. "Under the Broken Keel" is the scene title for context, NOT text to put in the image.
Scene: Silent salvage beneath a huge abandoned frontier frigate whose heavily armoured keel has split open. View obliquely upward under the wreck, so massive offset armour slabs frame a long jagged breach through the middle of the vessel. Exposed bulkheads, thick structural ribs, fractured titanium plates and torn service runs have real physical thickness. The wreck dominates the upper-middle and right of the scene; its forward section climbs into the upper-right darkness. A few separated armour fragments hang nearby with lots of clear vacuum between them. This is an old, dangerous derelict, not a ship exploding now.
Salvage story: Deep in the open armoured deck, one recoverable shield-emitter assembly remains in a damaged circular mount, glowing cyan. Small dim red-orange warning lamps trace the adjacent exposed engineering spine, revealing how far the rupture runs into the frigate. SC-07 holds position below and left of the breach, angling its bow toward the recovery point. A slender cyan industrial tractor field has attached to the shield-emitter assembly for a careful pull. Two tiny compact survey drones flank the recovery area with pinpoint inspection lights. All machinery has a plausible origin and attachment.
Workboat identity: SC-07 is much smaller than the frigate, a long low angular patched steel-gray workboat, with a raised FORWARD bridge near its bow, cyan windows, exactly THREE amber engine pods stacked vertically at its aft end, bolted side machinery, sparse orange hazard stripes, cyan bow tractor lens and belly cargo rail. Struts are retracted. Give its outline enough light to recognize this capable little worker against the enormous armoured wreck. No people.
Composition: Immersive low-angle wide view with a strong diagonal created by the broken keel. The giant rupture and shield mount are the focal area; small workboat and inspection drones establish scale. Keep the leftmost upper quarter predominantly quiet black-navy starfield for desktop icons and leave a calm bottom edge. Readable silhouette and distinct foreground, work area and deep interior layers. This is a new dramatic military-wreck composition, not a rectangular cargo bay or a station dock. No planet filling the background.
Style/materials: Premium original science-fiction concept painting, richly detailed but coherent hard-surface engineering, subtle hand-painted finish and cinematic contrast. Thick slate and graphite armour, scored edges, curled structural metal, recessed conduits, cold brushed titanium, older welded repairs. Restrained color, visually believable mass.
Lighting/mood: Very cold blue-white rim light grazes the outer armour. Selective cyan light at the shield mount and tractor apparatus, low red-orange warning lights deeper in the wreck, small amber engines on SC-07. Deep shadows remain legible. Tense, patient, resourceful recovery in the silence of space. No fog, flames or atmospheric smoke.
Constraints: Landscape wallpaper at full requested 3840x2160 if supported. No title, text, logos, watermark, UI, captions, labels, borders, montage, alien biology, combat, active weapons, explosions, excessive lens flare, colorful nebula or bright plasma clouds. Keep the salvage action unambiguous and the workboat separate from the frigate's structure.

## Refinement prompt

Use case: precise-object-edit
Edit this Salvage Captain wallpaper with ONE targeted correction to the small workboat at lower left.
Its raised wheelhouse is currently too far back, close to the three amber aft engines. MOVE that entire raised bridge/wheelhouse, including its cyan windows and short roof antenna, FORWARD along the hull to the right-hand forward third, immediately behind the cyan bow tractor emitter. The front of the boat is the RIGHT end where the cyan tractor field originates. The three stacked amber engines remain at the LEFT stern. Restore the old wheelhouse location to low flat deck plating. Keep the boat long, low and angular, with a clear forward raised bridge. Exactly three amber aft engines.
Preserve everything else: camera and framing, small workboat position and size, all three engines, patched steel hull, side equipment, enormous diagonal split-keel frigate, fractured armour slabs, glowing shield-emitter target, narrow cyan tractor field, exactly two inspection drones, red-orange spine lamps, cold rim lighting, sparse stars, debris placement, and quiet upper-left space.
Do not introduce lettering, logos, watermark, UI, new objects, extra engines or extra bridges. Retain the original cinematic finish and 16:9 landscape framing. Output the highest supported native desktop resolution, ideally 3840x2160.

## Source anchors

- `assets/data/sites.json`, military wreck: frontier frigate split at the keel,
  armour fragments, glowing shield mounts and warning-lit engineering spine.
- `src/ui/ship_visual/hull.rs`: low patched hull, forward cyan bridge and
  three aft engine pods.
- `src/ui/drone_visual.rs`: up to two compact survey drones supporting the
  selected salvage target.

## Review

The large diagonal keel rupture, thick armour and exposed ribs establish a
distinct military wreck. A narrow cyan field connects SC-07 to the exposed
shield assembly, with two small inspection drones beside the target. Sparse
red-orange lamps trace the internal spine without suggesting active combat.
The starfield at upper left leaves desktop breathing room. No text or UI is
present. The workboat remains separate from the surrounding debris.

The first image placed the workboat bridge too far aft. A targeted refinement
moved the bridge to the forward third and restored the old position to low
deck plating, while retaining all three engines and the recovery composition.
The selected final image was visually inspected after that correction.
Independent final QA confirmed the forward bridge, all three engines, both
drones and the tractor connection, with no material regression.

The tool returned 1672 x 941 pixels despite the requested desktop resolution.
This file is the untouched native PNG, not a verified 4K export. The earlier
question about local resizing remains unanswered.
