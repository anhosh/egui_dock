use duplicate::duplicate;
use egui::{
    CornerRadius, CursorIcon, EventFilter, Key, Rect, Response, Sense, Ui, Vec2, WidgetInfo,
    WidgetType, accesskit, vec2,
};
use paste::paste;

use crate::{
    DockArea, Node, NodePath, SeparatorStyle, SplitNode, Style, SurfaceIndex,
    dock_area::state::State, utils::map_to_pixel,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(in crate::widgets::dock_area) enum SeparatorAxis {
    /// vertical line, left/right split
    X,
    /// horizontal line, top/bottom split
    Y,
}

#[derive(Copy, Clone, Debug)]
pub(super) struct SeparatorHandle {
    pub(super) path: NodePath,
    pub(super) axis: SeparatorAxis,
    /// Painted rect
    pub(super) separator: Rect,
    /// Grab area
    pub(super) interact_rect: Rect,
}

#[derive(Clone, Debug)]
pub(super) struct SeparatorJunction {
    pub(super) members: Vec<usize>,
    pub(super) rect: Rect,
}

/// Returns the junctions of perpendicular separators.
fn find_junctions(handles: &[SeparatorHandle], merge_distance: f32) -> Vec<SeparatorJunction> {
    let (x_seps, y_seps): (Vec<_>, Vec<_>) =
        (0..handles.len()).partition(|&i| match handles[i].axis {
            SeparatorAxis::X => true,
            SeparatorAxis::Y => false,
        });

    // Filter for perpendicular separators pairs only
    let mut junctions: Vec<SeparatorJunction> = Vec::new();
    for &x in x_seps.iter() {
        for &y in y_seps.iter() {
            let (a, b) = (handles[x].interact_rect, handles[y].interact_rect);
            if a.intersects(b) {
                junctions.push(SeparatorJunction {
                    members: vec![x, y],
                    rect: a.intersect(b),
                });
            }
        }
    }

    // Merge to a point
    let merge_pad = merge_distance / 2.0;
    let mut merging = true;
    while merging {
        merging = false;
        'scan: for a in 0..junctions.len() {
            for b in (a + 1)..junctions.len() {
                if !junctions[a]
                    .rect
                    .expand(merge_pad)
                    .intersects(junctions[b].rect.expand(merge_pad))
                {
                    continue;
                }
                let other = junctions.remove(b);
                junctions[a].rect = junctions[a].rect.union(other.rect);
                junctions[a].members.extend(other.members);
                junctions[a].members.sort_unstable();
                junctions[a].members.dedup();
                merging = true;
                break 'scan; // this is needed to recompute `junctions.len()` after `remove()`
            }
        }
    }

    junctions
}

/// Returns the index in `handles` of the members of the junction being dragged, if any.
fn junction_frozen_members(
    state: &mut State,
    surf_index: SurfaceIndex,
    handles: &[SeparatorHandle],
) -> Option<Vec<usize>> {
    let members = state.dragged_junction.as_ref()?;
    if members.first()?.0.surface != surf_index {
        return None;
    }

    let resolved = members
        .iter()
        .map(|&(path, axis)| {
            handles
                .iter()
                .position(|handle| handle.path == path && handle.axis == axis)
        })
        .collect::<Option<Vec<_>>>();

    if resolved.is_none() {
        state.dragged_junction = None;
    }
    resolved
}

fn junction_rect(handles: &[SeparatorHandle], members: &[usize]) -> Rect {
    let mut rect = Rect::NOTHING;
    for &a in members {
        for &b in members {
            let (ra, rb) = (handles[a].interact_rect, handles[b].interact_rect);
            if handles[a].axis != handles[b].axis && ra.intersects(rb) {
                rect = rect.union(ra.intersect(rb));
            }
        }
    }
    rect
}

fn arrow_key_offset(ui: &Ui, response: &Response, step: f32) -> Option<Vec2> {
    let should_respond = ui.input(|i| i.modifiers.command || i.modifiers.shift);

    if response.has_focus() {
        // Prevent the default behaviour of removing focus from the separators when the
        // arrow keys are pressed
        ui.memory_mut(|m| {
            m.set_focus_lock_filter(
                response.id,
                EventFilter {
                    horizontal_arrows: should_respond,
                    vertical_arrows: should_respond,
                    tab: false,
                    escape: false,
                },
            )
        });
    }

    if !response.has_focus() || !should_respond {
        return None;
    }

    ui.input(|i| {
        if i.key_pressed(Key::ArrowUp) {
            Some(vec2(0., -step))
        } else if i.key_pressed(Key::ArrowDown) {
            Some(vec2(0., step))
        } else if i.key_pressed(Key::ArrowLeft) {
            Some(vec2(-step, 0.))
        } else if i.key_pressed(Key::ArrowRight) {
            Some(vec2(step, 0.))
        } else {
            None
        }
    })
}

fn apply_separator_delta(
    split: &mut SplitNode,
    axis: SeparatorAxis,
    delta: Vec2,
    style: &SeparatorStyle,
) {
    let (range, delta) = match axis {
        SeparatorAxis::X => (split.rect.width(), delta.x),
        SeparatorAxis::Y => (split.rect.height(), delta.y),
    };

    if range > 0.0 {
        let min = (style.extra / range).min(1.0);
        let max = 1.0 - min;
        let (min, max) = (min.min(max), max.max(min));
        split.fraction = (split.fraction + delta / range).clamp(min, max);
    }
}

impl<Tab> DockArea<'_, Tab> {
    /// Attempts to show a separator and returns an [`SeparatorHandle`] if drawn.
    pub(super) fn show_separator(
        &mut self,
        ui: &mut Ui,
        path: NodePath,
        fade_style: Option<&Style>,
    ) -> Option<SeparatorHandle> {
        assert!(self.dock_state[path.surface][path.node].is_parent());

        // If either of the children is collapsed, we don't want the user to interact with the separator
        if (self.dock_state[path.left_node()].is_collapsed()
            || self.dock_state[path.right_node()].is_collapsed())
            && self.dock_state[path.surface][path.node].is_vertical()
        {
            return None;
        }

        let style = fade_style.unwrap_or_else(|| self.style.as_ref().unwrap());
        let pixels_per_point = ui.ctx().pixels_per_point();
        let mut handle = None;

        duplicate! {
            [
                orientation   dim_point  dim_size  sep_axis  ak_orientation;
                [Horizontal]  [x]        [width]   [X]       [Vertical];
                [Vertical]    [y]        [height]  [Y]       [Horizontal];
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

                response.widget_info(|| WidgetInfo::new(WidgetType::ResizeHandle));

                let arrow_key_offset = arrow_key_offset(ui, &response, style.separator.arrow_key_step_distance);

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

                handle = Some(SeparatorHandle {
                    path,
                    axis: SeparatorAxis::sep_axis,
                    separator,
                    interact_rect,
                });

                // Update 'fraction' interaction after drawing separator,
                // otherwise it may overlap on other separator / bodies when
                // shrunk fast.
                let ak_steps = ui.input(|input| {
                    input.num_accesskit_action_requests(response.id, accesskit::Action::Increment) as f32
                        - input.num_accesskit_action_requests(response.id, accesskit::Action::Decrement) as f32
                });
                let ak_set_value = ui.input(|input| {
                    input
                        .accesskit_action_requests(response.id, accesskit::Action::SetValue)
                        .find_map(|request| match request.data {
                            Some(accesskit::ActionData::NumericValue(value)) => Some(value as f32),
                            _ => None,
                        })
                });
                let mut delta = arrow_key_offset.unwrap_or(response.drag_delta());
                delta.dim_point += ak_steps * style.separator.arrow_key_step_distance;
                if let Some(value) = ak_set_value {
                    delta.dim_point += (value.clamp(0.0, 1.0) - split.fraction) * rect.dim_size();
                }
                apply_separator_delta(split, SeparatorAxis::sep_axis, delta, &style.separator);

                if response.double_clicked() {
                    split.fraction = 0.5;
                }

                // Allow assistive technologies to adjust the separator fraction.
                let fraction = split.fraction;
                let step = style.separator.arrow_key_step_distance / rect.dim_size().max(1.0);
                ui.ctx().accesskit_node_builder(response.id, |node| {
                    node.set_orientation(accesskit::Orientation::ak_orientation);
                    node.set_numeric_value(fraction.into());
                    node.set_min_numeric_value(0.0);
                    node.set_max_numeric_value(1.0);
                    node.set_numeric_value_step(step.into());
                    node.add_action(accesskit::Action::Increment);
                    node.add_action(accesskit::Action::Decrement);
                    node.add_action(accesskit::Action::SetValue);
                });
            }
        }

        handle
    }

    pub(super) fn show_separator_junctions(
        &mut self,
        ui: &mut Ui,
        state: &mut State,
        surf_index: SurfaceIndex,
        handles: &[SeparatorHandle],
        fade_style: Option<&Style>,
    ) {
        let style = fade_style.unwrap_or_else(|| self.style.as_ref().unwrap());
        let mut junctions = find_junctions(handles, style.separator.junction_merge_distance);

        // Keep the junction that is being dragged consistent across frames
        if let Some(frozen) = junction_frozen_members(state, surf_index, handles) {
            junctions.retain(|junction| !junction.members.iter().any(|m| frozen.contains(m)));
            junctions.insert(
                0,
                SeparatorJunction {
                    rect: junction_rect(handles, &frozen),
                    members: frozen,
                },
            );
        }

        for junction in junctions {
            let interact_rect = junction
                .rect
                .expand(style.separator.extra_interact_width / 2.0);

            // Keying on the members is necessary to keep the drag going in case the tree is
            // restructured between frames.
            let mut paths: Vec<NodePath> =
                junction.members.iter().map(|&m| handles[m].path).collect();
            paths.sort_unstable_by_key(|path| (path.surface.0, path.node.0));
            let id = self.id.with("separator_junction").with(&paths);

            // Junctions should sense click and drags but not focus.
            // NOTE: don't use `Sense::click_and_drag()` here because it sets `Sense::FOCUSABLE` as well.
            let response = ui
                .interact(interact_rect, id, Sense::CLICK | Sense::DRAG)
                .on_hover_and_drag_cursor(CursorIcon::Move);
            response.widget_info(|| WidgetInfo::new(WidgetType::ResizeHandle));
            ui.ctx()
                .accesskit_node_builder(response.id, |node| node.set_hidden());

            if response.drag_started() {
                state.dragged_junction = Some(
                    junction
                        .members
                        .iter()
                        .map(|&m| (handles[m].path, handles[m].axis))
                        .collect(),
                );
            }

            let arrow_key_offset =
                arrow_key_offset(ui, &response, style.separator.arrow_key_step_distance);

            if response.dragged() || response.hovered() || response.has_focus() {
                let color = if response.dragged() {
                    style.separator.color_dragged
                } else {
                    style.separator.color_hovered
                };
                for &member in junction.members.iter() {
                    ui.painter()
                        .rect_filled(handles[member].separator, CornerRadius::ZERO, color);
                }
            }

            // Each member takes the same offset in pixels, clamped against its own bounds, so
            // members that are only near-aligned keep the offset between them.
            let delta = arrow_key_offset.unwrap_or(response.drag_delta());
            let reset = response.double_clicked();
            for &member in junction.members.iter() {
                let handle = handles[member];
                if let Node::Horizontal(split) | Node::Vertical(split) =
                    &mut self.dock_state[handle.path]
                {
                    if reset {
                        split.fraction = 0.5;
                    } else {
                        apply_separator_delta(split, handle.axis, delta, &style.separator);
                    }
                }
            }
        }
    }
}
