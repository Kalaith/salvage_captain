//! Homebound flight with a compact haul summary until docking.

use super::*;

pub(crate) const RETURN_TRAVEL_DURATION_SECONDS: f32 = 3.0;

pub fn draw_return_travel(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    transit::draw_transit(ctx, transit::FlightLeg::Homebound, actions);
}
