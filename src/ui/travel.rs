//! Outbound flight keeps the ship prominent while the wreck approaches.

use super::*;

pub(crate) const TRAVEL_DURATION_SECONDS: f32 = 4.0;

pub fn draw_travel(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    transit::draw_transit(ctx, transit::FlightLeg::Outbound, actions);
}
