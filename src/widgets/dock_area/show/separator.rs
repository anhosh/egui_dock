use duplicate::duplicate;
use egui::{CornerRadius, CursorIcon, EventFilter, Key, Sense, Ui, Vec2};
use paste::paste;

use crate::{DockArea, Node, NodePath, Style, utils::map_to_pixel};

impl<Tab> DockArea<'_, Tab> {
    pub(super) fn show_separator(
        &mut self,
        ui: &mut Ui,
        path: NodePath,
        fade_style: Option<&Style>,
    ) {
        assert!(self.dock_state[path.surface][path.node].is_parent());

        // If either of the children is collapsed, we don't want the user to interact with the separator
        if (self.dock_state[path.left_node()].is_collapsed()
            || self.dock_state[path.right_node()].is_collapsed())
            && self.dock_state[path.surface][path.node].is_vertical()
        {
            return;
        }

        let style = fade_style.unwrap_or_else(|| self.style.as_ref().unwrap());
        let pixels_per_point = ui.ctx().pixels_per_point();

        duplicate! {
            [
                orientation   dim_point  dim_size;
                [Horizontal]  [x]        [width];
                [Vertical]    [y]        [height];
            ]
            if let Node::orientation(split) = &mut self.dock_state[path.surface][path.node] {
                let rect = split.rect;
                let mut separator = rect;

                let midpoint = rect.min.dim_point + rect.dim_size() * split.fraction;
                separator.min.dim_point = midpoint - style.separator.width * 0.5;
                separator.max.dim_point = midpoint + style.separator.width * 0.5;

                let mut expand = Vec2::ZERO;
                expand.dim_point += style.separator.extra_interact_width / 2.0;
                let interact_rect = separator.expand2(expand);

                let response = ui.allocate_rect(interact_rect, Sense::click_and_drag())
                    .on_hover_and_drag_cursor(paste!{ CursorIcon::[<Resize orientation>]});

                let should_respond_to_arrow_keys = ui.input(|i| i.modifiers.command || i.modifiers.shift);

                if response.has_focus() {
                    // Prevent the default behaviour of removing focus from the separators when the
                    // arrow keys are pressed
                    ui.memory_mut(|m| m.set_focus_lock_filter(response.id, EventFilter {
                        horizontal_arrows: should_respond_to_arrow_keys,
                        vertical_arrows: should_respond_to_arrow_keys,
                        tab: false,
                        escape: false
                    }));
                }

                let arrow_key_offset = if response.has_focus() && should_respond_to_arrow_keys {
                    if ui.input(|i| i.key_pressed(Key::ArrowUp)) {
                        Some(egui::vec2(0., -16.))
                    } else if ui.input(|i| i.key_pressed(Key::ArrowDown)) {
                        Some(egui::vec2(0., 16.))
                    } else if ui.input(|i| i.key_pressed(Key::ArrowLeft)) {
                        Some(egui::vec2(-16., 0.))
                    } else if ui.input(|i| i.key_pressed(Key::ArrowRight)) {
                        Some(egui::vec2(16., 0.))
                    } else {
                        None
                    }
                } else {
                    None
                };

                let midpoint = rect.min.dim_point + rect.dim_size() * split.fraction;
                separator.min.dim_point = map_to_pixel(
                    midpoint - style.separator.width * 0.5,
                    pixels_per_point,
                    f32::round,
                );
                separator.max.dim_point = map_to_pixel(
                    midpoint + style.separator.width * 0.5,
                    pixels_per_point,
                    f32::round,
                );

                let color = if response.dragged() {
                    style.separator.color_dragged
                } else if response.hovered() || response.has_focus() {
                    style.separator.color_hovered
                } else {
                    style.separator.color_idle
                };

                ui.painter().rect_filled(separator, CornerRadius::ZERO, color);

                // Update 'fraction' interaction after drawing separator,
                // otherwise it may overlap on other separator / bodies when
                // shrunk fast.
                let range = rect.max.dim_point - rect.min.dim_point;
                if range > 0.0 {
                    let min = (style.separator.extra / range).min(1.0);
                    let max = 1.0 - min;
                    let (min, max) = (min.min(max), max.max(min));
                    let delta = arrow_key_offset.unwrap_or(response.drag_delta()).dim_point;
                    split.fraction = (split.fraction + delta / range).clamp(min, max);
                }

                if response.double_clicked() {
                    split.fraction = 0.5;
                }
            }
        }
    }
}
