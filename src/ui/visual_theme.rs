//! Named colors and signal hierarchy for the industrial salvage presentation.

use macroquad::prelude::*;

pub fn space() -> Color {
    Color::new(0.015, 0.025, 0.05, 1.0)
}

pub fn structure() -> Color {
    Color::new(0.24, 0.30, 0.34, 1.0)
}

pub fn structure_dark() -> Color {
    Color::new(0.10, 0.14, 0.18, 1.0)
}

pub fn structure_light() -> Color {
    Color::new(0.42, 0.49, 0.51, 1.0)
}

pub fn amber() -> Color {
    Color::new(0.96, 0.55, 0.18, 1.0)
}

pub fn cyan() -> Color {
    Color::new(0.18, 0.82, 0.88, 1.0)
}

pub fn cyan_dim() -> Color {
    Color::new(0.08, 0.34, 0.40, 1.0)
}

pub fn warning() -> Color {
    Color::new(0.92, 0.30, 0.20, 1.0)
}

pub fn safe() -> Color {
    Color::new(0.20, 0.91, 0.66, 1.0)
}

pub fn text() -> Color {
    Color::new(0.85, 0.91, 0.94, 1.0)
}

pub fn text_dim() -> Color {
    Color::new(0.55, 0.67, 0.73, 1.0)
}

pub fn panel() -> Color {
    Color::new(0.025, 0.055, 0.075, 0.98)
}

pub fn panel_soft() -> Color {
    Color::new(0.045, 0.085, 0.11, 0.94)
}

pub fn site_accent(theme: &str) -> Color {
    match theme {
        "military" => warning(),
        "research" => cyan(),
        _ => amber(),
    }
}

pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, alpha.clamp(0.0, 1.0))
}

pub fn draw_space_field(elapsed: f32) {
    clear_background(space());
    for index in 0..54 {
        let x = ((index * 83) % 1240) as f32 + 20.0;
        let y = ((index * 47) % 500) as f32 + 104.0;
        let drift = (elapsed * (0.8 + index as f32 * 0.01) + index as f32).sin() * 3.0;
        let size = if index % 7 == 0 { 2.0 } else { 1.0 };
        draw_circle(
            x + drift,
            y,
            size,
            with_alpha(text_dim(), 0.18 + (index % 4) as f32 * 0.05),
        );
    }
    draw_line(
        24.0,
        588.0,
        1256.0,
        588.0,
        1.0,
        with_alpha(cyan_dim(), 0.24),
    );
}

pub fn draw_meter(rect: Rect, progress: f32, fill: Color, label: &str) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        with_alpha(structure_dark(), 0.9),
    );
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w * progress.clamp(0.0, 1.0),
        rect.h,
        fill,
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, structure_light());
    draw_text(label, rect.x + 8.0, rect.y + rect.h - 6.0, 13.0, text());
}

/// Body copy uses the toolkit's readable face and a fixed minimum size.
pub fn body(text: &str, rect: Rect, size: f32, color: Color) {
    macroquad_toolkit::ui::draw_text_block_ex(
        text,
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        macroquad_toolkit::ui::TextStyle::new(size, color).with_line_gap(3.0),
        size,
    );
}

pub fn surface(rect: Rect) {
    draw_rectangle(
        rect.x + 4.0,
        rect.y + 7.0,
        rect.w,
        rect.h,
        with_alpha(BLACK, 0.35),
    );
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, panel());
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, structure());
    draw_line(
        rect.x + 1.0,
        rect.y + 1.0,
        rect.right() - 1.0,
        rect.y + 1.0,
        1.0,
        cyan_dim(),
    );
}

/// Game-specific colors over the toolkit's release-triggered button widget.
pub fn button_style(
    tone: macroquad_toolkit::ui::ButtonTone,
    enabled: bool,
) -> macroquad_toolkit::ui::ButtonStyle {
    use macroquad_toolkit::ui::{ButtonStyle, ButtonTone};
    let accent = match tone {
        ButtonTone::Primary | ButtonTone::Positive => safe(),
        ButtonTone::Warning => amber(),
        ButtonTone::Danger => warning(),
        ButtonTone::Secondary | ButtonTone::Muted => structure_light(),
    };
    let emphasized = matches!(tone, ButtonTone::Primary | ButtonTone::Positive);
    ButtonStyle {
        normal: if emphasized {
            Color::new(0.025, 0.13, 0.105, 0.98)
        } else {
            panel()
        },
        hovered: Color::new(0.08, 0.19, 0.22, 1.0),
        pressed: Color::new(0.04, 0.12, 0.15, 1.0),
        border: with_alpha(accent, if enabled { 0.85 } else { 0.28 }),
        text_color: if emphasized { safe() } else { text() },
        disabled: Color::new(0.025, 0.045, 0.06, 0.95),
    }
}
