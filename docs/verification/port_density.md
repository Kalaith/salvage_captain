# Port/Hangar desktop density verification

Compared the same deterministic `port` scene at **1920 × 1080**, with no management panel open. Both images use two capture frames and the toolkit's borderless fullscreen framebuffer capture; neither image is resized.

- [Before](ui_port_before.png)
- [After](ui_port.png)

The before image is retained specifically for this requested comparison. The normal `ui_port.png` and existing management screenshots are replaced in place.

| Measurement at 1920 × 1080 | Before | After |
| --- | ---: | ---: |
| Top status strip | 102 px | 78 px (24% shorter) |
| Bottom controls and edge space, measured from highest hit area | 183 px / 16.9% | 90 px / 8.3% |
| Uninterrupted world band between top and bottom HUD | 795 px / 73.6% | 912 px / 84.4% |
| Browse Wrecks visible face | 480 × 114 px | 300 × 54 px (70% less area) |
| Secondary action face height | 96 px | 54 px |
| Bottom control hit area height | 96–114 px | 66 px |

The world-band measure conservatively treats the whole width of both HUD strips as UI, including empty gaps. The ship's artwork, size, position, and hangar landmarks are unchanged. The floor continues through the space formerly occupied by the footer; its shading now sits behind the edge controls.

Credits, fuel, and hull use one line without individual separators or boxes. Materials remain in Service. The checkpoint is available over the port identifier; the market summary is included in the Service hover hint and remains available on the wreck-selection header. Cargo shows an icon, capacity, and one small meter. Service, Equipment, Crew, Cargo, LOG, and menu still emit their existing actions. Management drawers retain their close controls; selecting a different management control replaces the current drawer.

Instructions and the registry footer are no longer persistent. Short dock hints appear on hover. Transaction feedback remains visible while a management drawer is open and disappears with the drawer.

Visual review also covered `ui_port_services.png`, `ui_port_equipment.png`, `ui_port_crew.png`, `ui_port_grid.png`, `ui_port_repaired.png`, and `ui_logbook.png`. Drawers, selected states, repair feedback, and header navigation remain readable and clear of the bottom controls.

Validation: `cargo test` (including the source-size gate and existing five-case port suite) and `cargo clippy --all-targets -- -D warnings` passed. The port layout regression now also checks the requested header reduction, maximum dock height, and minimum unobstructed world band.

Default publish.ps1 completed native and WASM release builds, packaging, and deployment to Preview. Project Roost tracking reported a non-fatal connection refusal at 127.0.0.1:80; deployment itself succeeded. Final screenshots were recaptured from the published release binary.
