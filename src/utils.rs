use egui::emath::*;
use egui::style::{Visuals, WidgetVisuals, Widgets};

use crate::{
    ButtonsStyle, SeparatorStyle, Style, TabBarStyle, TabBodyStyle, TabInteractionStyle, TabStyle,
};

#[inline(always)]
pub fn expand_to_pixel(mut rect: Rect, ppi: f32) -> Rect {
    rect.min = map_to_pixel_pos(rect.min, ppi, f32::floor);
    rect.max = map_to_pixel_pos(rect.max, ppi, f32::ceil);
    rect
}

#[inline(always)]
pub fn map_to_pixel_pos(mut pos: Pos2, ppi: f32, map: fn(f32) -> f32) -> Pos2 {
    pos.x = map_to_pixel(pos.x, ppi, map);
    pos.y = map_to_pixel(pos.y, ppi, map);
    pos
}

#[inline(always)]
pub fn map_to_pixel(point: f32, ppi: f32, map: fn(f32) -> f32) -> f32 {
    map(point * ppi) / ppi
}

pub fn rect_set_size_centered(rect: &mut Rect, size: Vec2) {
    let center = rect.center();
    rect.set_width(size.x);
    rect.set_height(size.y);
    rect.set_center(center);
}

/// Shrink a rectangle so that the stroke is fully contained inside
/// the original rectangle.
pub fn rect_stroke_box(rect: Rect, width: f32) -> Rect {
    rect.expand(-f32::ceil(width / 2.0))
}

/// Fade a `egui_dock::Style` to a certain opacity
pub(super) fn fade_dock_style(style: &mut Style, factor: f32) {
    style.main_surface_border_stroke.color = style
        .main_surface_border_stroke
        .color
        .linear_multiply(factor);
    fade_tab_style(&mut style.tab, factor);
    fade_button_style(&mut style.buttons, factor);
    fade_seperator_style(&mut style.separator, factor);
    fade_tab_bar_style(&mut style.tab_bar, factor);
}

fn fade_tab_bar_style(style: &mut TabBarStyle, factor: f32) {
    style.hline_color = style.hline_color.linear_multiply(factor);
    style.bg_fill = style.bg_fill.linear_multiply(factor);
}

fn fade_seperator_style(style: &mut SeparatorStyle, factor: f32) {
    style.color_idle = style.color_idle.linear_multiply(factor);
    style.color_hovered = style.color_hovered.linear_multiply(factor);
    style.color_dragged = style.color_dragged.linear_multiply(factor);
}

fn fade_button_style(style: &mut ButtonsStyle, factor: f32) {
    style.close_tab_color = style.close_tab_color.linear_multiply(factor);
    style.close_tab_active_color = style.close_tab_active_color.linear_multiply(factor);
    style.close_tab_bg_fill = style.close_tab_bg_fill.linear_multiply(factor);
    style.add_tab_color = style.add_tab_color.linear_multiply(factor);
    style.add_tab_active_color = style.add_tab_active_color.linear_multiply(factor);
    style.add_tab_bg_fill = style.add_tab_bg_fill.linear_multiply(factor);
    style.add_tab_border_color = style.add_tab_border_color.linear_multiply(factor);
}

fn fade_tab_style(style: &mut TabStyle, factor: f32) {
    fade_tab_interaction_style(&mut style.active, factor);
    fade_tab_interaction_style(&mut style.inactive, factor);
    fade_tab_interaction_style(&mut style.focused, factor);
    fade_tab_interaction_style(&mut style.hovered, factor);
    fade_tab_body_style(&mut style.tab_body, factor);
}

fn fade_tab_interaction_style(style: &mut TabInteractionStyle, factor: f32) {
    style.outline_color = style.outline_color.linear_multiply(factor);
    style.bg_fill = style.bg_fill.linear_multiply(factor);
    style.text_color = style.text_color.linear_multiply(factor);
}

fn fade_tab_body_style(style: &mut TabBodyStyle, factor: f32) {
    style.stroke.color = style.stroke.color.linear_multiply(factor);
    style.bg_fill = style.bg_fill.linear_multiply(factor);
}

/// Fade a `egui::style::Visuals` to a certain opacity
pub(super) fn fade_visuals(visuals: &mut Visuals, factor: f32) {
    if let Some(override_text_color) = &mut visuals.override_text_color {
        *override_text_color = override_text_color.linear_multiply(factor);
    }
    visuals.hyperlink_color = visuals.hyperlink_color.linear_multiply(factor);
    visuals.faint_bg_color = visuals.faint_bg_color.linear_multiply(factor);
    visuals.extreme_bg_color = visuals.extreme_bg_color.linear_multiply(factor);
    visuals.code_bg_color = visuals.code_bg_color.linear_multiply(factor);
    visuals.warn_fg_color = visuals.warn_fg_color.linear_multiply(factor);
    visuals.error_fg_color = visuals.error_fg_color.linear_multiply(factor);
    visuals.window_fill = visuals.window_fill.linear_multiply(factor);
    visuals.panel_fill = visuals.window_fill.linear_multiply(factor);
    fade_widgets(&mut visuals.widgets, factor);
}

fn fade_widgets(widgets: &mut Widgets, factor: f32) {
    fade_widget_visuals(&mut widgets.noninteractive, factor);
    fade_widget_visuals(&mut widgets.inactive, factor);
    fade_widget_visuals(&mut widgets.hovered, factor);
    fade_widget_visuals(&mut widgets.active, factor);
    fade_widget_visuals(&mut widgets.open, factor);
}

fn fade_widget_visuals(visuals: &mut WidgetVisuals, factor: f32) {
    visuals.bg_fill = visuals.bg_fill.linear_multiply(factor);
    visuals.weak_bg_fill = visuals.weak_bg_fill.linear_multiply(factor);
    visuals.bg_stroke.color = visuals.bg_stroke.color.linear_multiply(factor);
    visuals.fg_stroke.color = visuals.fg_stroke.color.linear_multiply(factor);
}
