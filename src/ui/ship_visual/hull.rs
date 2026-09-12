//! Layered workboat hull, with clear decks reserved for optional equipment.

use super::{ink::ShipInk, visual_theme as theme};
use macroquad::prelude::*;

pub(super) fn draw(ink: &ShipInk, elapsed: f32, docked: bool) {
    draw_frame(ink);
    draw_drive(ink, elapsed);
    draw_bridge(ink);
    draw_deck(ink);
    draw_rig(ink, docked);
    draw_emitter(ink, elapsed);
}

fn draw_frame(ink: &ShipInk) {
    ink.polygon(
        &[
            (65., 122.),
            (211., 99.),
            (291., 119.),
            (806., 108.),
            (966., 172.),
            (982., 207.),
            (819., 281.),
            (158., 281.),
            (65., 248.),
        ],
        theme::space(),
    );
    ink.polygon(
        &[
            (84., 126.),
            (209., 108.),
            (285., 127.),
            (801., 118.),
            (957., 179.),
            (966., 204.),
            (813., 269.),
            (161., 269.),
            (84., 240.),
        ],
        theme::structure(),
    );
    ink.polygon(
        &[
            (84., 126.),
            (209., 108.),
            (285., 127.),
            (801., 118.),
            (922., 166.),
            (798., 143.),
            (282., 147.),
            (207., 128.),
            (84., 145.),
        ],
        theme::structure_light(),
    );
    ink.polygon(
        &[
            (84., 223.),
            (179., 245.),
            (810., 243.),
            (966., 193.),
            (966., 204.),
            (813., 269.),
            (161., 269.),
            (84., 240.),
        ],
        theme::structure_dark(),
    );
    ink.line((179., 267.), (810., 267.), 4., theme::structure_light());
    ink.line((811., 266.), (954., 207.), 3., theme::structure_light());
    ink.polygon(
        &[(831., 158.), (919., 176.), (949., 191.), (854., 217.)],
        theme::structure_dark(),
    );
    ink.line((867., 174.), (918., 183.), 5., theme::amber());
    ink.line((870., 185.), (905., 190.), 2., theme::structure_light());
    for x in [187., 270., 408., 643., 798.] {
        ink.line((x, 153.), (x - 9., 242.), 2., theme::structure_dark());
        ink.bolts((x + 5., 156., 0., 81.), theme::structure_light());
    }
    // Sparse scratches retain the patched industrial finish at hangar scale.
    for (x, y) in [(216., 162.), (397., 173.), (667., 161.), (794., 232.)] {
        ink.line((x, y), (x + 13., y - 3.), 1., theme::text_dim());
        ink.line((x + 4., y + 4.), (x + 9., y + 2.), 1., theme::text_dim());
    }
}

fn draw_drive(ink: &ShipInk, elapsed: f32) {
    ink.panel((65., 107., 93., 158.), theme::structure_dark());
    ink.panel((87., 115., 20., 143.), theme::structure_light());
    for y in [112., 163., 214.] {
        ink.polygon(
            &[
                (13., y + 5.),
                (68., y),
                (88., y + 9.),
                (88., y + 34.),
                (68., y + 41.),
                (13., y + 35.),
            ],
            theme::space(),
        );
        ink.panel((20., y + 9., 60., 25.), theme::structure());
        ink.line((24., y + 10.), (72., y + 10.), 3., theme::structure_light());
        for x in [32., 44., 56.] {
            ink.line((x, y + 16.), (x, y + 29.), 3., theme::structure_dark());
        }
        let glow = 0.7 + (elapsed * 2.4 + y).sin() * 0.12;
        ink.disc(
            (14., y + 22.),
            20.,
            theme::with_alpha(theme::amber(), glow * 0.1),
        );
        ink.panel((9., y + 12., 9., 20.), theme::amber());
        ink.panel((9., y + 17., 5., 10.), Color::new(1., 0.88, 0.55, glow));
    }
}

fn draw_bridge(ink: &ShipInk) {
    ink.polygon(
        &[
            (623., 124.),
            (650., 62.),
            (786., 62.),
            (827., 106.),
            (827., 147.),
            (632., 147.),
        ],
        theme::space(),
    );
    ink.polygon(
        &[
            (637., 121.),
            (660., 72.),
            (782., 72.),
            (815., 110.),
            (815., 135.),
            (639., 135.),
        ],
        theme::structure(),
    );
    ink.polygon(
        &[
            (657., 72.),
            (782., 72.),
            (791., 82.),
            (655., 82.),
            (638., 119.),
        ],
        theme::structure_light(),
    );
    ink.polygon(
        &[(670., 90.), (777., 90.), (799., 113.), (662., 113.)],
        theme::cyan_dim(),
    );
    ink.line((672., 92.), (774., 92.), 3., theme::cyan());
    for x in [701., 738., 772.] {
        ink.line((x, 90.), (x + 4., 114.), 4., theme::structure_dark());
    }
    ink.line((651., 132.), (800., 132.), 2., theme::text_dim());
    ink.panel((778., 121., 13., 4.), theme::amber());
    ink.panel((793., 121., 9., 4.), theme::safe());
}

fn draw_deck(ink: &ShipInk) {
    ink.panel((265., 153., 369., 33.), theme::structure_dark());
    ink.line((271., 157.), (627., 157.), 2., theme::structure_light());
    for x in (280..620).step_by(17) {
        ink.line((x as f32, 166.), (x as f32, 180.), 3., theme::space());
    }
    ink.panel((304., 92., 317., 17.), theme::structure_dark());
    ink.line((306., 93.), (615., 93.), 3., theme::structure_light());
    ink.line((225., 121.), (225., 55.), 4., theme::structure_dark());
    ink.line((225., 55.), (599., 55.), 4., theme::structure_light());
    ink.line((599., 55.), (617., 122.), 3., theme::structure_dark());
    for x in [248., 419., 603., 802.] {
        ink.panel((x - 5., 275., 10., 10.), theme::space());
        ink.disc((x, 278.), 3., theme::amber());
    }
    ink.panel((181., 194., 50., 32.), theme::structure_dark());
    for i in 0..4 {
        let x = 184. + i as f32 * 11.;
        ink.line((x, 218.), (x + 9., 200.), 4., theme::amber());
    }
    if ink.rect.w > 420. {
        let p = ink.point((179., 185.));
        draw_text("SC-07", p.x, p.y, 15. * ink.scale(), theme::text());
    }
}

fn draw_rig(ink: &ShipInk, docked: bool) {
    let bottom = if docked { 345. } else { 305. };
    for x in [265., 690.] {
        ink.panel((x, 268., 27., 20.), theme::space());
        ink.line(
            (x + 14., 283.),
            (x + 34., bottom - 5.),
            10.,
            theme::structure_dark(),
        );
        ink.line(
            (x + 11., 283.),
            (x + 30., bottom - 7.),
            3.,
            theme::text_dim(),
        );
        ink.line((x - 14., bottom), (x + 66., bottom), 7., theme::structure());
        ink.line(
            (x - 14., bottom - 2.),
            (x + 66., bottom - 2.),
            2.,
            theme::structure_light(),
        );
    }
    // The belly rail remains unobstructed by the side-mounted upgrade pods.
    ink.line((184., 287.), (823., 287.), 8., theme::space());
    ink.line((184., 286.), (823., 286.), 2., theme::structure_light());
}

fn draw_emitter(ink: &ShipInk, elapsed: f32) {
    ink.polygon(
        &[
            (892., 178.),
            (892., 153.),
            (942., 146.),
            (959., 165.),
            (949., 216.),
            (902., 224.),
        ],
        theme::space(),
    );
    ink.panel((906., 150., 32., 61.), theme::structure_light());
    ink.panel((912., 160., 28., 39.), theme::structure_dark());
    ink.line((920., 161.), (920., 44.), 9., theme::structure_dark());
    ink.line((920., 158.), (920., 45.), 3., theme::cyan_dim());
    ink.disc((920., 44.), 18., theme::with_alpha(theme::cyan(), 0.08));
    ink.disc((920., 44.), 10., theme::structure_light());
    ink.disc((920., 44.), 6., theme::cyan());
    ink.line((925., 176.), (979., 176.), 10., theme::structure_dark());
    ink.line((958., 176.), (979., 176.), 4., theme::cyan());
    ink.disc(
        super::TRACTOR_NOZZLE,
        13.,
        theme::with_alpha(theme::cyan(), 0.08 + (elapsed * 2.).sin().abs() * 0.05),
    );
}
