use std::ops::BitOrAssign;

use egui::{
    emath::{inverse_lerp, GuiRounding},
    vec2, Context, Id, LayerId, NumExt, Order, Painter, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2,
};

use crate::{
    AllowedSplits, NodeIndex, NodePath, Split, Style, SurfaceIndex, TabDestination, TabInsert,
    TabPath,
};

#[derive(Debug, Clone)]
pub(super) struct HoverData {
    /// Rect of the hovered element.
    pub rect: Rect,

    /// The "address" of the tab/node being hovered over.
    pub dst: TreeComponent,

    /// If a tab title or the tab head is hovered, this is the rect of it.
    pub tab: Option<Rect>,
}

/// Specifies the location of a tab on the tree, used when moving tabs.
#[derive(Debug, Clone)]
pub(super) struct DragData {
    pub src: TreeComponent,
    pub rect: Rect,
}

#[derive(Debug, Clone)]
pub(super) enum TreeComponent {
    Surface(SurfaceIndex),
    Node(NodePath),
    Tab(TabPath),
}

impl TreeComponent {
    pub(super) fn as_tab_destination(&self) -> TabDestination {
        match *self {
            TreeComponent::Surface(surface) => TabDestination::EmptySurface(surface),
            TreeComponent::Node(dst) => TabDestination::Node(dst, TabInsert::Append),
            TreeComponent::Tab(dst) => {
                TabDestination::Node(dst.node_path(), TabInsert::Insert(dst.tab))
            }
        }
    }

    pub(super) fn node_address(&self) -> (SurfaceIndex, Option<NodeIndex>) {
        match *self {
            TreeComponent::Surface(surface) => (surface, None),
            TreeComponent::Node(dst) => (dst.surface, Some(dst.node)),
            TreeComponent::Tab(dst) => (dst.surface, Some(dst.node)),
        }
    }

    pub(super) fn surface_address(&self) -> SurfaceIndex {
        match *self {
            TreeComponent::Surface(surface)
            | TreeComponent::Node(NodePath { surface, .. })
            | TreeComponent::Tab(TabPath { surface, .. }) => surface,
        }
    }

    pub(super) fn is_surface(&self) -> bool {
        matches!(self, TreeComponent::Surface(_))
    }
}

fn make_overlay_painter(ui: &Ui) -> Painter {
    let id = Id::new("overlay");
    let layer_id = LayerId::new(Order::Foreground, id);
    ui.ctx().layer_painter(layer_id)
}

fn draw_highlight_rect(rect: Rect, ui: &Ui, style: &Style) {
    let painter = make_overlay_painter(ui);
    painter.rect(
        rect.expand(style.overlay.hovered_leaf_highlight.expansion),
        style.overlay.hovered_leaf_highlight.corner_radius,
        style.overlay.hovered_leaf_highlight.color,
        style.overlay.hovered_leaf_highlight.stroke,
        StrokeKind::Inside,
    );
}

// Draws one of the Tab drop destination icons inside `rect`, which one you get is specified by `is_top_bottom`.
fn button_ui(
    rect: Rect,
    ui: &Ui,
    lock: &mut bool,
    mouse_pos: Pos2,
    style: &Style,
    split: Option<Split>,
) -> bool {
    let visuals = &style.overlay;
    let button_stroke = Stroke::new(1.0, visuals.button_color);
    let painter = make_overlay_painter(ui);
    painter.rect_stroke(rect, 0.0, visuals.button_border_stroke, StrokeKind::Inside);
    let rect = rect.shrink(rect.width() * 0.1);
    painter.rect_stroke(rect, 0.0, button_stroke, StrokeKind::Inside);
    let rim = { Rect::from_two_pos(rect.min, rect.lerp_inside(vec2(1.0, 0.1))) };
    painter.rect(
        rim,
        0.0,
        visuals.button_color,
        Stroke::NONE,
        StrokeKind::Inside,
    );

    if let Some(split) = split {
        for line in DASHED_LINE_ALPHAS.chunks(2) {
            let start = rect.lerp_inside(lerp_vec(split, line[0]));
            let end = rect.lerp_inside(lerp_vec(split, line[1]));
            painter.line_segment([start, end], button_stroke);
        }
    }
    let is_mouse_over = rect
        .expand(style.overlay.feel.interact_expansion)
        .contains(mouse_pos);
    if is_mouse_over && !*lock {
        let vertical_alphas = vec2(1.0, 0.5);
        let horizontal_alphas = vec2(0.5, 1.0);
        let rect = match split {
            Some(Split::Above) => Rect::from_min_size(rect.min, rect.size() * vertical_alphas),
            Some(Split::Left) => Rect::from_min_size(rect.min, rect.size() * horizontal_alphas),
            Some(Split::Below) => {
                let min = rect.lerp_inside(lerp_vec(Split::Below, 0.0));
                Rect::from_min_size(min, rect.size() * vertical_alphas)
            }
            Some(Split::Right) => {
                let min = rect.lerp_inside(lerp_vec(Split::Right, 0.0));
                Rect::from_min_size(min, rect.size() * horizontal_alphas)
            }
            _ => rect,
        };
        painter.rect_filled(rect, 0.0, style.overlay.selection_color);
    }
    lock.bitor_assign(is_mouse_over);
    is_mouse_over
}

const DASHED_LINE_ALPHAS: [f32; 8] = [
    0.0625, 0.1875, 0.3125, 0.4375, 0.5625, 0.6875, 0.8125, 0.9375,
];

#[derive(PartialEq, Eq)]
enum LockState {
    /// Lock is unlocked.
    Unlocked,

    /// Lock remains locked, but can be unlocked.
    SoftLock,

    /// Lock is locked forever.
    HardLock,
}

#[derive(Debug, Clone)]
pub(super) struct DragDropState {
    pub hover: HoverData,
    pub drag: DragData,
    pub pointer: Pos2,
    /// Is some when the pointer is over rect, f64 holds the time when the lock was last active.
    pub locked: Option<f64>,
}

impl DragDropState {
    // Determines if the hover data implies we're hovering over a tab or the tab title bar.
    pub(super) fn is_on_title_bar(&self) -> bool {
        self.hover.tab.is_some()
    }

    pub(super) fn resolve_icon_based(
        &mut self,
        ui: &Ui,
        style: &Style,
        allowed_splits: AllowedSplits,
        windows_allowed: bool,
        window_bounds: Rect,
    ) -> Option<TabDestination> {
        assert!(!self.is_on_title_bar());

        draw_highlight_rect(self.hover.rect, ui, style);
        let mut hovering_buttons = false;
        let total_button_spacing = style.overlay.button_spacing * 2.0;
        let (rect, pointer) = (self.hover.rect, self.pointer);
        let rect = rect.shrink(style.overlay.button_spacing);
        let shortest_side = ((rect.width() - total_button_spacing) / 3.0)
            .min((rect.height() - total_button_spacing) / 3.0)
            .min(style.overlay.max_button_size);

        let mut destination: Option<TabDestination> = windows_allowed
            .then(|| TabDestination::Window(Rect::from_min_size(pointer, self.drag.rect.size())));

        let center = rect.center();
        let rect = Rect::from_center_size(center, Vec2::splat(shortest_side));

        if button_ui(rect, ui, &mut hovering_buttons, pointer, style, None) {
            match self.hover.dst {
                TreeComponent::Node(path) => {
                    destination = Some(TabDestination::Node(path, TabInsert::Append))
                }
                TreeComponent::Surface(surface) => {
                    destination = Some(TabDestination::EmptySurface(surface))
                }
                _ => (),
            }
        }

        for split in [Split::Below, Split::Right, Split::Above, Split::Left] {
            match allowed_splits {
                AllowedSplits::TopBottomOnly if !split.is_top_bottom() => continue,
                AllowedSplits::LeftRightOnly if !split.is_left_right() => continue,
                AllowedSplits::None => continue,
                _ => {
                    let offset_value = shortest_side + style.overlay.button_spacing;
                    let offset_vector = match split {
                        Split::Above => vec2(0.0, -offset_value),
                        Split::Below => vec2(0.0, offset_value),
                        Split::Left => vec2(-offset_value, 0.0),
                        Split::Right => vec2(offset_value, 0.0),
                    };
                    if button_ui(
                        Rect::from_center_size(center + offset_vector, Vec2::splat(shortest_side)),
                        ui,
                        &mut hovering_buttons,
                        pointer,
                        style,
                        Some(split),
                    ) {
                        if let TreeComponent::Node(path) = self.hover.dst {
                            destination = Some(TabDestination::Node(path, TabInsert::Split(split)))
                        }
                    }
                }
            }
        }
        let hovering_rect = self.hover.rect.contains(pointer);
        let target_lock_state = match (hovering_rect, hovering_buttons) {
            (false, false) => LockState::Unlocked,
            (_, true) => LockState::HardLock,
            (true, _) => LockState::SoftLock,
        };
        self.update_lock(target_lock_state, style, ui.ctx());
        if let Some(TabDestination::Window(rect)) = destination {
            let rect = self.window_preview_rect(rect);
            let rect_bounded = constrain_rect_to_area(ui, rect, window_bounds);
            draw_window_rect(rect_bounded, ui, style);
        }
        destination
    }

    pub(super) fn resolve_traditional(
        &mut self,
        ui: &Ui,
        style: &Style,
        allowed_splits: AllowedSplits,
        windows_allowed: bool,
        window_bounds: Rect,
    ) -> Option<TabDestination> {
        // If windows are not allowed, any hover over a window is immediately disallowed.
        if !windows_allowed && self.hover.dst.surface_address() != SurfaceIndex::main() {
            return None;
        }
        draw_highlight_rect(self.hover.rect, ui, style);

        // Deals with hovers over tab bar and tab titles.
        if let Some(rect) = self.hover.tab {
            draw_drop_rect(rect, ui, style);
            let target_lock_state = if rect.contains(self.pointer) {
                LockState::SoftLock
            } else {
                LockState::Unlocked
            };
            self.update_lock(target_lock_state, style, ui.ctx());
            return Some(self.hover.dst.as_tab_destination());
        }

        // Main cases, splits, window creations, etc.
        let (hover_rect, pointer) = (self.hover.rect, self.pointer);
        let center = hover_rect.center();

        let (tab_insertion, overlay_rect) = {
            // A reverse lerp of the pointers position relative to the hovered leaf rect.
            // Range is (-0.5, -0.5) to (0.5, 0.5)
            let a_pos = (Pos2::new(
                inverse_lerp(hover_rect.x_range().into(), pointer.x).unwrap(),
                inverse_lerp(hover_rect.y_range().into(), pointer.y).unwrap(),
            ) - Pos2::new(0.5, 0.5))
            .to_pos2();

            let center_drop_rect = Rect::from_center_size(
                Pos2::ZERO,
                Vec2::splat(style.overlay.feel.center_drop_coverage),
            );
            let window_drop_rect = Rect::from_center_size(
                Pos2::ZERO,
                Vec2::splat(style.overlay.feel.window_drop_coverage),
            );

            // Find out what kind of tab insertion (if any) should be used to move this widget.
            if center_drop_rect.contains(a_pos) {
                (Some(TabInsert::Append), Rect::EVERYTHING)
            } else if window_drop_rect.contains(a_pos) {
                match windows_allowed {
                    true => (None, Rect::NOTHING),
                    false => (Some(TabInsert::Append), Rect::EVERYTHING),
                }
            } else {
                // Assessing if were above/below the two linear functions x-y=0 and -x-y=0 determines
                // what "diagonal" quadrant were in.
                let a_pos = match allowed_splits {
                    AllowedSplits::All => a_pos,
                    AllowedSplits::LeftRightOnly => Pos2::new(a_pos.x, 0.0),
                    AllowedSplits::TopBottomOnly => Pos2::new(0.0, a_pos.y),
                    AllowedSplits::None => Pos2::ZERO,
                };
                if a_pos == Pos2::ZERO {
                    match windows_allowed {
                        true => (None, Rect::NOTHING),
                        false => (Some(TabInsert::Append), Rect::EVERYTHING),
                    }
                } else {
                    match (a_pos.x - a_pos.y > 0., -a_pos.x - a_pos.y > 0.) {
                        (true, true) => (
                            Some(TabInsert::Split(Split::Above)),
                            Rect::everything_above(center.y),
                        ),
                        (false, true) => (
                            Some(TabInsert::Split(Split::Left)),
                            Rect::everything_left_of(center.x),
                        ),
                        (true, false) => (
                            Some(TabInsert::Split(Split::Right)),
                            Rect::everything_right_of(center.x),
                        ),
                        (false, false) => (
                            Some(TabInsert::Split(Split::Below)),
                            Rect::everything_below(center.y),
                        ),
                    }
                }
            }
        };

        let default_value = windows_allowed
            .then(|| TabDestination::Window(Rect::from_min_size(pointer, self.drag.rect.size())));
        let final_result = tab_insertion.map_or(default_value, |tab| match self.hover.dst {
            TreeComponent::Surface(surface) => Some(TabDestination::EmptySurface(surface)),
            TreeComponent::Node(path) => Some(TabDestination::Node(path, tab)),
            _ => None,
        });

        self.update_lock(LockState::SoftLock, style, ui.ctx());

        // Draw the overlay
        match final_result {
            Some(TabDestination::Window(rect)) => {
                let rect = self.window_preview_rect(rect);
                let rect_bounded = constrain_rect_to_area(ui, rect, window_bounds);
                draw_window_rect(rect_bounded, ui, style);
            }
            Some(_) => {
                draw_drop_rect(hover_rect.intersect(overlay_rect), ui, style);
            }
            None => (),
        }

        final_result
    }

    fn update_lock(&mut self, target_state: LockState, style: &Style, ctx: &Context) {
        match self.locked.as_mut() {
            Some(lock_time) => {
                if target_state == LockState::HardLock {
                    *lock_time = ctx.input(|i| i.time);
                }
                let window_hold = if !self.hover.dst.surface_address().is_main() {
                    ctx.request_repaint();
                    self.is_locked(style, ctx)
                } else {
                    false
                };
                if target_state == LockState::Unlocked && !window_hold {
                    self.locked = None;
                }
            }
            None => {
                if target_state != LockState::Unlocked {
                    self.locked = Some(ctx.input(|i| i.time));
                }
            }
        }
    }

    pub(super) fn is_locked(&self, style: &Style, ctx: &Context) -> bool {
        match self.locked.as_ref() {
            Some(lock_time) => {
                let elapsed = ctx.input(|i| (i.time - lock_time) as f32);
                ctx.request_repaint();
                elapsed < style.overlay.feel.max_preference_time
            }
            None => false,
        }
    }

    fn window_preview_rect(&self, rect: Rect) -> Rect {
        if self.drag.src.surface_address() == SurfaceIndex::main() {
            Rect::from_min_size(rect.min, rect.size() * 0.8)
        } else {
            rect
        }
    }
}

#[inline(always)]
const fn lerp_vec(split: Split, alpha: f32) -> Vec2 {
    if split.is_top_bottom() {
        vec2(alpha, 0.5)
    } else {
        vec2(0.5, alpha)
    }
}

// Draws a filled rect describing where a tab will be dropped.
#[inline(always)]
fn draw_drop_rect(rect: Rect, ui: &Ui, style: &Style) {
    let painter = make_overlay_painter(ui);
    painter.rect_filled(rect, 0.0, style.overlay.selection_color);
}

// Draws a stroked rect describing where a tab will be dropped.
#[inline(always)]
fn draw_window_rect(rect: Rect, ui: &Ui, style: &Style) {
    let painter = make_overlay_painter(ui);
    painter.rect_stroke(
        rect,
        0.0,
        Stroke::new(
            style.overlay.selection_stroke_width,
            style.overlay.selection_color,
        ),
        StrokeKind::Inside,
    );
}

/// An adapted version of the [`egui::Area`]s code for restricting an area rect to a bound.
fn constrain_rect_to_area(ui: &Ui, rect: Rect, mut bounds: Rect) -> Rect {
    if rect.width() > bounds.width() {
        // Allow overlapping side bars.
        let screen_rect = ui.ctx().content_rect();
        (bounds.min.x, bounds.max.x) = (screen_rect.min.x, screen_rect.max.x);
    }
    if rect.height() > bounds.height() {
        // Allow overlapping top/bottom bars:
        let screen_rect = ui.ctx().content_rect();
        (bounds.min.y, bounds.max.y) = (screen_rect.min.y, screen_rect.max.y);
    }

    let mut pos = rect.min;

    // Constrain to screen, unless window is too large to fit:
    let margin_x = (rect.width() - bounds.width()).at_least(0.0);
    let margin_y = (rect.height() - bounds.height()).at_least(0.0);

    pos.x = pos.x.at_most(bounds.right() + margin_x - rect.width()); // move left if needed
    pos.x = pos.x.at_least(bounds.left() - margin_x); // move right if needed
    pos.y = pos.y.at_most(bounds.bottom() + margin_y - rect.height()); // move right if needed
    pos.y = pos.y.at_least(bounds.top() - margin_y); // move down if needed

    pos = pos.round_to_pixels(ui.painter().pixels_per_point());

    Rect::from_min_size(pos, rect.size())
}
