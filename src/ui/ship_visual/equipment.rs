//! Recognizable machinery silhouettes fitted to the workboat's hardpoints.

use super::{ink::ShipInk, visual_theme as theme};
use macroquad::prelude::*;

pub(super) fn draw(ink: &ShipInk, kind: &str, elapsed: f32, damaged: bool) {
    let signal = if damaged {
        theme::structure_dark()
    } else {
        theme::cyan()
    };
    match kind {
        "engine" => engine(ink, signal),
        "tank" => tank(ink),
        "antenna" => antenna(ink, signal),
        "scanner" => scanner(ink, signal),
        "reactor" => reactor(ink, signal, elapsed, damaged),
        "shield" => shield(ink, signal),
        "drone_bay" => drones(ink, signal),
        "battery" => battery(ink, signal),
        "plating" => plating(ink),
        _ => casing(ink),
    }
}

fn casing(ink: &ShipInk) {
    ink.polygon(
        &[
            (8., 16.),
            (22., 5.),
            (80., 5.),
            (94., 19.),
            (94., 64.),
            (80., 76.),
            (18., 76.),
            (8., 65.),
        ],
        theme::space(),
    );
    ink.polygon(
        &[
            (12., 19.),
            (24., 10.),
            (78., 10.),
            (89., 21.),
            (89., 60.),
            (77., 69.),
            (22., 69.),
            (12., 60.),
        ],
        theme::structure(),
    );
    ink.line((24., 11.), (76., 11.), 3., theme::text_dim());
    ink.bolts((21., 20., 58., 40.), theme::text_dim());
}

fn engine(ink: &ShipInk, signal: Color) {
    casing(ink);
    for y in [25., 38., 51.] {
        ink.panel((19., y, 28., 7.), theme::space());
        ink.line((20., y), (43., y), 1., theme::text_dim());
    }
    ink.disc((64., 40.), 19., theme::space());
    ink.ring((64., 40.), 15., theme::text_dim());
    ink.disc((64., 40.), 10., theme::cyan_dim());
    ink.disc((64., 40.), 4., signal);
    ink.panel((28., 65., 43., 4.), theme::amber());
}

fn tank(ink: &ShipInk) {
    ink.panel((12., 16., 76., 52.), theme::space());
    ink.polygon(
        &[
            (5., 29.),
            (16., 19.),
            (85., 19.),
            (96., 29.),
            (96., 52.),
            (85., 63.),
            (16., 63.),
            (5., 52.),
        ],
        theme::structure_light(),
    );
    ink.panel((17., 43., 68., 18.), theme::structure());
    ink.line((19., 25.), (83., 25.), 3., theme::text_dim());
    for x in [25., 70.] {
        ink.panel((x, 17., 6., 49.), theme::structure_dark());
        ink.panel((x + 1., 20., 2., 42.), theme::text_dim());
    }
    ink.panel((42., 32., 16., 12.), theme::amber());
    ink.line((88., 38.), (99., 38.), 4., theme::structure_dark());
    ink.disc((92., 38.), 3., theme::warning());
}

fn antenna(ink: &ShipInk, signal: Color) {
    ink.panel((31., 59., 46., 14.), theme::space());
    ink.polygon(
        &[(38., 60.), (50., 48.), (62., 48.), (70., 60.)],
        theme::structure_light(),
    );
    ink.line((56., 56.), (56., 9.), 4., theme::structure_light());
    ink.line((55., 29.), (25., 18.), 3., theme::structure());
    ink.line((29., 10.), (24., 25.), 3., theme::text_dim());
    ink.line((56., 41.), (82., 27.), 3., theme::structure());
    ink.line((79., 20.), (87., 32.), 3., theme::text_dim());
    ink.disc((56., 8.), 3., signal);
    ink.panel((42., 63., 21., 3.), signal);
}

fn scanner(ink: &ShipInk, signal: Color) {
    ink.panel((22., 60., 54., 13.), theme::space());
    ink.line((45., 62.), (57., 34.), 7., theme::structure_light());
    ink.polygon(
        &[
            (20., 19.),
            (32., 35.),
            (58., 44.),
            (84., 34.),
            (93., 17.),
            (70., 27.),
            (43., 27.),
        ],
        theme::structure(),
    );
    ink.line((21., 18.), (43., 27.), 3., theme::text_dim());
    ink.line((43., 27.), (70., 27.), 3., theme::text_dim());
    ink.line((70., 27.), (92., 17.), 3., theme::text_dim());
    ink.line((58., 37.), (64., 11.), 3., theme::structure_light());
    ink.disc((65., 10.), 4., signal);
    ink.disc((46., 59.), 5., theme::structure());
}

fn reactor(ink: &ShipInk, signal: Color, elapsed: f32, damaged: bool) {
    casing(ink);
    ink.disc((50., 40.), 28., theme::space());
    ink.disc((50., 40.), 24., theme::structure_light());
    ink.disc((50., 40.), 18., theme::structure_dark());
    let pulse = if damaged {
        0.
    } else {
        0.12 + (elapsed * 2.).sin().abs() * 0.08
    };
    ink.disc((50., 40.), 26., theme::with_alpha(signal, pulse));
    ink.ring((50., 40.), 14., signal);
    ink.disc((50., 40.), 6., signal);
    for p in [(50., 14.), (50., 66.), (24., 40.), (76., 40.)] {
        ink.disc(p, 3., theme::amber());
    }
}

fn shield(ink: &ShipInk, signal: Color) {
    casing(ink);
    ink.polygon(
        &[(31., 20.), (70., 20.), (73., 42.), (50., 62.), (28., 42.)],
        theme::space(),
    );
    ink.polygon(
        &[(36., 25.), (65., 25.), (67., 40.), (50., 54.), (34., 40.)],
        theme::cyan_dim(),
    );
    ink.line((38., 27.), (63., 27.), 2., signal);
    ink.line((50., 32.), (50., 46.), 3., signal);
    for x in [16., 82.] {
        ink.line((x, 28.), (x, 51.), 4., theme::text_dim());
    }
}

fn drones(ink: &ShipInk, signal: Color) {
    casing(ink);
    ink.panel((17., 24., 66., 35.), theme::space());
    for x in [34., 66.] {
        ink.panel((x - 12., 29., 24., 25.), theme::structure_dark());
        ink.line((x - 10., 34.), (x + 10., 34.), 3., theme::text_dim());
        ink.line((x - 9., 45.), (x + 9., 45.), 3., theme::text_dim());
        ink.panel((x - 4., 31., 8., 16.), theme::structure_light());
        ink.disc((x, 36.), 2.5, signal);
    }
    ink.line((21., 63.), (80., 63.), 3., theme::amber());
}

fn battery(ink: &ShipInk, signal: Color) {
    casing(ink);
    for x in [26., 44., 62.] {
        ink.panel((x, 22., 13., 36.), theme::space());
        ink.panel((x + 2., 25., 9., 29.), theme::structure_light());
        ink.panel((x + 4., 28., 5., 7.), signal);
        ink.line((x + 3., 43.), (x + 10., 43.), 2., theme::structure_dark());
    }
    ink.line((25., 62.), (75., 62.), 3., theme::amber());
}

fn plating(ink: &ShipInk) {
    for offset in [0., 7.] {
        ink.polygon(
            &[
                (8. + offset, 18. + offset),
                (77. + offset, 10. + offset),
                (92. + offset, 53. + offset),
                (26. + offset, 68. + offset),
            ],
            theme::space(),
        );
        ink.polygon(
            &[
                (13. + offset, 21. + offset),
                (73. + offset, 15. + offset),
                (85. + offset, 50. + offset),
                (29. + offset, 61. + offset),
            ],
            theme::structure_light(),
        );
        ink.line(
            (28. + offset, 53. + offset),
            (79. + offset, 43. + offset),
            3.,
            theme::structure(),
        );
    }
    ink.bolts((30., 31., 42., 18.), theme::space());
    ink.line((43., 33.), (63., 30.), 4., theme::amber());
}
