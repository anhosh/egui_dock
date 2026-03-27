use egui::{ecolor::*, CornerRadius, Margin, Stroke};

/// Left or right alignment for tab add button.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[allow(missing_docs)]
pub enum TabAddAlign {
    Left,
    Right,
}

/// Lets you change how tabs and the [`DockArea`](crate::DockArea) should look and feel.
/// [`Style`] is divided into several, more specialized structs that handle individual
/// elements of the UI.
///
/// Your [`Style`] can inherit all its properties from an [`egui::Style`] through the
/// [`Style::from_egui`] function.
///
/// Example:
///
/// ```rust
/// # use egui_dock::{DockArea, DockState, OverlayType, Style, TabAddAlign, TabViewer};
/// # use egui::{Ui, WidgetText};
/// # struct MyTabViewer;
/// # impl TabViewer for MyTabViewer {
/// #     type Tab = ();
/// #     fn title(&mut self, tab: &mut Self::Tab) -> WidgetText { WidgetText::default() }
/// #     fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {}
/// # }
/// # egui::__run_test_ctx(|ctx| {
/// # #[allow(deprecated)]
/// # egui::CentralPanel::default().show(ctx, |ui| {
/// # let mut dock_state = DockState::new(vec![]);
/// // Inherit the look and feel from egui.
/// let mut style = Style::from_egui(ui.style());
///
/// // Modify a few fields.
/// style.overlay.overlay_type = OverlayType::HighlightedAreas;
/// style.buttons.add_tab_align = TabAddAlign::Left;
///
/// // Use the style with the `DockArea`.
/// DockArea::new(&mut dock_state)
///     .style(style)
///     .show_inside(ui, &mut MyTabViewer);
/// # });
/// # });
/// #
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[allow(missing_docs)]
pub struct Style {
    /// Sets padding to indent from the edges of the window. By `Default` it's `None`.
    pub dock_area_padding: Option<Margin>,

    pub main_surface_border_stroke: Stroke,
    pub main_surface_border_rounding: CornerRadius,

    pub buttons: ButtonsStyle,
    pub separator: SeparatorStyle,
    pub tab_bar: TabBarStyle,
    pub tab: TabStyle,
    pub overlay: OverlayStyle,
}

/// Specifies the look and feel of buttons.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ButtonsStyle {
    /// Color of the close tab button.
    pub close_tab_color: Color32,

    /// Color of the active close tab button.
    pub close_tab_active_color: Color32,

    /// Color of the background close tab button.
    pub close_tab_bg_fill: Color32,

    /// Left or right aligning of the add tab button.
    pub add_tab_align: TabAddAlign,

    /// Color of the add tab button.
    pub add_tab_color: Color32,

    /// Color of the active add tab button.
    pub add_tab_active_color: Color32,

    /// Color of the add tab button's background.
    pub add_tab_bg_fill: Color32,

    /// Color of the add tab button's left border.
    pub add_tab_border_color: Color32,

    /// Color of the close all tabs button.
    pub close_all_tabs_color: Color32,

    /// Color of the active close all tabs button.
    pub close_all_tabs_active_color: Color32,

    /// Color of the close all tabs button's background.
    pub close_all_tabs_bg_fill: Color32,

    /// Color of the close all tabs button's left border.
    pub close_all_tabs_border_color: Color32,

    /// Color of disabled close all tabs button.
    pub close_all_tabs_disabled_color: Color32,

    /// Color of the collapse tabs button.
    pub collapse_tabs_color: Color32,

    /// Color of the active collapse tabs button.
    pub collapse_tabs_active_color: Color32,

    /// Color of the collapse tabs button's background.
    pub collapse_tabs_bg_fill: Color32,

    /// Color of the collapse tabs button's left border.
    pub collapse_tabs_border_color: Color32,

    /// Color of the minimize window button.
    pub minimize_window_color: Color32,

    /// Color of the active minimize window button.
    pub minimize_window_active_color: Color32,

    /// Color of the minimize window button's background.
    pub minimize_window_bg_fill: Color32,

    /// Color of the minimize window button's left border.
    pub minimize_window_border_color: Color32,
}

/// Specifies the look and feel of node separators.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SeparatorStyle {
    /// Width of the rectangle separator between nodes. By `Default` it's `1.0`.
    pub width: f32,

    /// Extra width added to the "logical thickness" of the rectangle so it's
    /// easier to grab. By `Default` it's `4.0`.
    pub extra_interact_width: f32,

    /// Limit for the allowed area for the separator offset. By `Default` it's `175.0`.
    /// `bigger value > less allowed offset` for the current window size.
    pub extra: f32,

    /// Idle color of the rectangle separator. By `Default` it's [`Color32::BLACK`].
    pub color_idle: Color32,

    /// Hovered color of the rectangle separator. By `Default` it's [`Color32::GRAY`].
    pub color_hovered: Color32,

    /// Dragged color of the rectangle separator. By `Default` it's [`Color32::WHITE`].
    pub color_dragged: Color32,
}

/// Specifies the look and feel of tab bars.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TabBarStyle {
    /// Background color of tab bar. By `Default` it's [`Color32::WHITE`].
    pub bg_fill: Color32,

    /// Height of the tab bar. By `Default` it's `24.0`.
    pub height: f32,

    /// Inner margin of tab bar. By `Default` it's `Margin::ZERO`.
    pub inner_margin: Margin,

    /// Show a scroll bar when tab bar overflows. By `Default` it's `true`.
    pub show_scroll_bar_on_overflow: bool,

    /// Tab corner_radius. By `Default` it's [`CornerRadius::default`].
    pub corner_radius: CornerRadius,

    /// Color of the line separating the tab name area from the tab content area.
    /// By `Default` it's [`Color32::BLACK`].
    pub hline_color: Color32,

    /// Whether tab titles expand to fill the width of their tab bars.
    /// By `Default` it's `false`.
    pub fill_tab_bar: bool,
}

/// Specifies the look and feel of an individual tab.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TabStyle {
    /// Style of the tab when it is active.
    pub active: TabInteractionStyle,

    /// Style of the tab when it is inactive.
    pub inactive: TabInteractionStyle,

    /// Style of the tab when it is focused.
    pub focused: TabInteractionStyle,

    /// Style of the tab when it is hovered.
    pub hovered: TabInteractionStyle,

    /// Style of the tab when it is inactive and has keyboard focus.
    pub inactive_with_kb_focus: TabInteractionStyle,

    /// Style of the tab when it is active and has keyboard focus.
    pub active_with_kb_focus: TabInteractionStyle,

    /// Style of the tab when it is focused and has keyboard focus.
    pub focused_with_kb_focus: TabInteractionStyle,

    /// Style for the tab body.
    pub tab_body: TabBodyStyle,

    /// If `true`, show the hline below the active tabs name.
    /// If `false`, show the active tab as merged with the tab ui area.
    /// By `Default` it's `false`.
    pub hline_below_active_tab_name: bool,

    /// Spacing between tabs.
    pub spacing: f32,

    /// The minimum width of the tab.
    ///
    /// The tab title or [`TabBarStyle::fill_tab_bar`] may make the tab
    /// wider than this but never shorter.
    pub minimum_width: Option<f32>,
}

/// Specifies the look and feel of individual tabs while they are being interacted with.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TabInteractionStyle {
    /// Color of the outline around tabs. By `Default` it's [`Color32::BLACK`].
    pub outline_color: Color32,

    /// Tab corner radius. By `Default` it's [`CornerRadius::default`].
    pub corner_radius: CornerRadius,

    /// Colour of the tab's background. By `Default` it's [`Color32::WHITE`].
    pub bg_fill: Color32,

    /// Color of the title text.
    pub text_color: Color32,
}

/// Specifies the look and feel of the tab body.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TabBodyStyle {
    /// Inner margin of tab body. By `Default` it's `Margin::same(4.0)`.
    pub inner_margin: Margin,

    /// The stroke of the tabs border. By `Default` it's ['Stroke::default'].
    pub stroke: Stroke,

    /// Tab corner radius. By `Default` it's [`CornerRadius::default`].
    pub corner_radius: CornerRadius,

    /// Colour of the tab's background. By `Default` it's [`Color32::WHITE`].
    pub bg_fill: Color32,
}

/// Specifies the look and feel of the tab drop overlay.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct OverlayStyle {
    /// Sets selection color for the placing area of the tab where this tab targeted on it.
    /// By `Default` it's `(0, 191, 255)` (light blue) with `0.5` capacity.
    pub selection_color: Color32,

    /// Width of stroke when a selection uses an outline instead of filled rectangle.
    pub selection_stroke_width: f32,

    /// Units of padding between each button.
    pub button_spacing: f32,

    /// Max side length of a button on the overlay.
    pub max_button_size: f32,

    /// Style of the additional highlighting rectangle drawn on the surface which you're attempting to drop a tab in.
    ///
    /// By default this value shows no highlighting.
    pub hovered_leaf_highlight: LeafHighlighting,

    /// Opacity which surfaces will fade to in a range of `0.0..=1.0`.
    pub surface_fade_opacity: f32,

    /// The color of the overlay buttons.
    pub button_color: Color32,

    /// The stroke of the button border.
    pub button_border_stroke: Stroke,

    /// The type of overlay used.
    pub overlay_type: OverlayType,

    /// The feel of the overlay, timings, detection, etc.
    pub feel: OverlayFeel,
}

/// Specifies the feel of the tab drop overlay, i.e anything non visual about the overlay.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct OverlayFeel {
    /// range is `0.0..=1.0`.
    pub window_drop_coverage: f32,

    /// range is `0.0..=1.0`.
    pub center_drop_coverage: f32,

    /// The amount of time windows should stay faded despite not needing to, prevents quick mouse movements from causing flashing.
    pub fade_hold_time: f32,

    /// Amount of time the overlay waits before dropping a preference it may have for a node.
    pub max_preference_time: f32,

    /// Units which the buttons interact area will be expanded by.
    pub interact_expansion: f32,
}

/// Specifies the type of overlay used.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum OverlayType {
    /// Shows highlighted areas predicting where a dropped tab would land were it to be dropped this frame.
    ///
    /// Always used when hovering over tabs and tab head.
    HighlightedAreas,

    /// Shows icons indicating the possible drop positions which the user may hover over to drop a tab at that given location.
    ///
    /// This is the default type of overlay for leaves.
    Widgets,
}

/// Highlighting on the currently hovered leaf.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct LeafHighlighting {
    /// Fill color.
    pub color: Color32,

    /// Rounding of the resulting rectangle.
    pub corner_radius: CornerRadius,

    /// Stroke.
    pub stroke: Stroke,

    /// Amount of egui units which each side should expand.
    pub expansion: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            dock_area_padding: None,
            main_surface_border_stroke: Stroke::new(f32::default(), Color32::BLACK),
            main_surface_border_rounding: CornerRadius::default(),
            buttons: ButtonsStyle::default(),
            separator: SeparatorStyle::default(),
            tab_bar: TabBarStyle::default(),
            tab: TabStyle::default(),
            overlay: OverlayStyle::default(),
        }
    }
}

impl Default for ButtonsStyle {
    fn default() -> Self {
        Self {
            close_tab_color: Color32::WHITE,
            close_tab_active_color: Color32::WHITE,
            close_tab_bg_fill: Color32::GRAY,

            add_tab_align: TabAddAlign::Right,
            add_tab_color: Color32::WHITE,
            add_tab_active_color: Color32::WHITE,
            add_tab_bg_fill: Color32::GRAY,
            add_tab_border_color: Color32::BLACK,

            close_all_tabs_color: Color32::WHITE,
            close_all_tabs_active_color: Color32::WHITE,
            close_all_tabs_bg_fill: Color32::GRAY,
            close_all_tabs_border_color: Color32::BLACK,
            close_all_tabs_disabled_color: Color32::LIGHT_GRAY,

            collapse_tabs_color: Color32::WHITE,
            collapse_tabs_active_color: Color32::WHITE,
            collapse_tabs_bg_fill: Color32::GRAY,
            collapse_tabs_border_color: Color32::BLACK,

            minimize_window_color: Color32::WHITE,
            minimize_window_active_color: Color32::WHITE,
            minimize_window_bg_fill: Color32::GRAY,
            minimize_window_border_color: Color32::BLACK,
        }
    }
}

impl Default for SeparatorStyle {
    fn default() -> Self {
        Self {
            width: 1.0,
            extra_interact_width: 2.0,
            extra: 175.0,
            color_idle: Color32::BLACK,
            color_hovered: Color32::GRAY,
            color_dragged: Color32::WHITE,
        }
    }
}

impl Default for TabBarStyle {
    fn default() -> Self {
        Self {
            bg_fill: Color32::WHITE,
            height: 24.0,
            inner_margin: Margin::ZERO,
            show_scroll_bar_on_overflow: true,
            corner_radius: CornerRadius::default(),
            hline_color: Color32::BLACK,
            fill_tab_bar: false,
        }
    }
}

impl Default for TabStyle {
    fn default() -> Self {
        Self {
            active: TabInteractionStyle::default(),
            inactive: TabInteractionStyle {
                text_color: Color32::DARK_GRAY,
                ..Default::default()
            },
            focused: TabInteractionStyle {
                text_color: Color32::BLACK,
                ..Default::default()
            },
            hovered: TabInteractionStyle {
                text_color: Color32::BLACK,
                ..Default::default()
            },
            active_with_kb_focus: TabInteractionStyle::default(),
            inactive_with_kb_focus: TabInteractionStyle {
                text_color: Color32::DARK_GRAY,
                ..Default::default()
            },
            focused_with_kb_focus: TabInteractionStyle {
                text_color: Color32::BLACK,
                ..Default::default()
            },
            spacing: 0.0,
            tab_body: TabBodyStyle::default(),
            hline_below_active_tab_name: false,
            minimum_width: None,
        }
    }
}

impl Default for TabInteractionStyle {
    fn default() -> Self {
        Self {
            bg_fill: Color32::WHITE,
            outline_color: Color32::BLACK,
            corner_radius: CornerRadius::default(),
            text_color: Color32::DARK_GRAY,
        }
    }
}

impl Default for TabBodyStyle {
    fn default() -> Self {
        Self {
            inner_margin: Margin::same(4),
            stroke: Stroke::default(),
            corner_radius: CornerRadius::default(),
            bg_fill: Color32::WHITE,
        }
    }
}

impl Default for OverlayStyle {
    fn default() -> Self {
        Self {
            selection_color: Color32::from_rgb(0, 191, 255).linear_multiply(0.5),
            selection_stroke_width: 1.0,
            button_spacing: 10.0,
            max_button_size: 100.0,

            surface_fade_opacity: 0.1,

            hovered_leaf_highlight: Default::default(),
            button_color: Color32::from_gray(140),
            button_border_stroke: Stroke::new(1.0, Color32::from_gray(60)),
            overlay_type: OverlayType::Widgets,
            feel: Default::default(),
        }
    }
}

impl Default for OverlayFeel {
    fn default() -> Self {
        Self {
            max_preference_time: 0.3,
            window_drop_coverage: 0.5,
            center_drop_coverage: 0.25,
            fade_hold_time: 0.2,
            interact_expansion: 20.0,
        }
    }
}

impl Default for LeafHighlighting {
    fn default() -> Self {
        Self {
            color: Color32::TRANSPARENT,
            corner_radius: CornerRadius::same(0),
            stroke: Stroke::NONE,
            expansion: 0.0,
        }
    }
}

impl Style {
    pub(crate) const TAB_ADD_BUTTON_SIZE: f32 = 24.0;
    pub(crate) const TAB_ADD_PLUS_SIZE: f32 = 12.0;
    pub(crate) const TAB_CLOSE_BUTTON_SIZE: f32 = 24.0;
    pub(crate) const TAB_CLOSE_X_SIZE: f32 = 9.0;
    pub(crate) const TAB_CLOSE_ALL_BUTTON_SIZE: f32 = 24.0;
    pub(crate) const TAB_CLOSE_ALL_SIZE: f32 = 10.0;
    pub(crate) const TAB_COLLAPSE_BUTTON_SIZE: f32 = 24.0;
    pub(crate) const TAB_COLLAPSE_ARROW_SIZE: f32 = 10.0;
    pub(crate) const TAB_EXPAND_BUTTON_SIZE: f32 = 24.0;
    pub(crate) const TAB_EXPAND_ARROW_SIZE: f32 = 10.0;
}

impl Style {
    /// Derives relevant fields from `egui::Style` and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`Style::main_surface_border_stroke`]
    ///
    /// See also: [`ButtonsStyle::from_egui`], [`SeparatorStyle::from_egui`], [`TabBarStyle::from_egui`],
    /// [`TabStyle::from_egui`]
    pub fn from_egui(style: &egui::Style) -> Self {
        Self {
            main_surface_border_stroke: Stroke::NONE,
            main_surface_border_rounding: CornerRadius::ZERO,
            buttons: ButtonsStyle::from_egui(style),
            separator: SeparatorStyle::from_egui(style),
            tab_bar: TabBarStyle::from_egui(style),
            tab: TabStyle::from_egui(style),
            overlay: OverlayStyle::from_egui(style),
            ..Self::default()
        }
    }
}

impl ButtonsStyle {
    /// Derives relevant fields from `egui::Style` and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`ButtonsStyle::close_tab_bg_fill`]
    /// - [`ButtonsStyle::close_tab_color`]
    /// - [`ButtonsStyle::close_tab_active_color`]
    /// - [`ButtonsStyle::add_tab_bg_fill`]
    /// - [`ButtonsStyle::add_tab_color`]
    /// - [`ButtonsStyle::add_tab_active_color`]
    /// - [`ButtonsStyle::add_tab_border_color`]
    /// - [`ButtonsStyle::close_all_tabs_bg_fill`]
    /// - [`ButtonsStyle::close_all_tabs_color`]
    /// - [`ButtonsStyle::close_all_tabs_active_color`]
    /// - [`ButtonsStyle::close_all_tabs_border_color`]
    /// - [`ButtonsStyle::collapse_tabs_bg_fill`]
    /// - [`ButtonsStyle::collapse_tabs_color`]
    /// - [`ButtonsStyle::collapse_tabs_active_color`]
    /// - [`ButtonsStyle::collapse_tabs_border_color`]
    pub fn from_egui(style: &egui::Style) -> Self {
        Self {
            close_tab_bg_fill: style.visuals.widgets.hovered.bg_fill,
            close_tab_color: style.visuals.text_color(),
            close_tab_active_color: style.visuals.strong_text_color(),
            add_tab_bg_fill: style.visuals.widgets.hovered.bg_fill,
            add_tab_color: style.visuals.text_color(),
            add_tab_active_color: style.visuals.strong_text_color(),
            add_tab_border_color: style.visuals.widgets.noninteractive.bg_fill,
            close_all_tabs_bg_fill: style.visuals.widgets.hovered.bg_fill,
            close_all_tabs_color: style.visuals.text_color(),
            close_all_tabs_active_color: style.visuals.strong_text_color(),
            close_all_tabs_border_color: style.visuals.widgets.noninteractive.bg_fill,
            close_all_tabs_disabled_color: style.visuals.widgets.inactive.bg_fill,
            collapse_tabs_bg_fill: style.visuals.widgets.hovered.bg_fill,
            collapse_tabs_color: style.visuals.text_color(),
            collapse_tabs_active_color: style.visuals.strong_text_color(),
            collapse_tabs_border_color: style.visuals.widgets.noninteractive.bg_fill,
            minimize_window_bg_fill: style.visuals.widgets.hovered.bg_fill,
            minimize_window_color: style.visuals.text_color(),
            minimize_window_active_color: style.visuals.strong_text_color(),
            minimize_window_border_color: style.visuals.widgets.noninteractive.bg_fill,
            ..ButtonsStyle::default()
        }
    }
}

impl SeparatorStyle {
    /// Derives relevant fields from `egui::Style` and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`SeparatorStyle::color_idle`]
    /// - [`SeparatorStyle::color_hovered`]
    /// - [`SeparatorStyle::color_dragged`]
    pub fn from_egui(style: &egui::Style) -> Self {
        Self {
            // Same as egui panel resize colors:
            color_idle: style.visuals.widgets.noninteractive.bg_stroke.color, // dim
            color_hovered: style.visuals.widgets.hovered.fg_stroke.color,     // bright
            color_dragged: style.visuals.widgets.active.fg_stroke.color,      // bright
            ..SeparatorStyle::default()
        }
    }
}

impl TabBarStyle {
    /// Derives relevant fields from `egui::Style` and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`TabBarStyle::bg_fill`]
    /// - [`TabBarStyle::hline_color`]
    pub fn from_egui(style: &egui::Style) -> Self {
        Self {
            bg_fill: style.visuals.extreme_bg_color,
            corner_radius: CornerRadius {
                nw: style.visuals.widgets.inactive.corner_radius.nw + 2,
                ne: style.visuals.widgets.inactive.corner_radius.ne + 2,
                sw: 0,
                se: 0,
            },
            hline_color: style.visuals.widgets.noninteractive.bg_stroke.color,
            ..TabBarStyle::default()
        }
    }
}

impl TabStyle {
    /// Derives tab styles from `egui::Style`.
    ///
    /// See also: [`TabInteractionStyle::from_egui_active`], [`TabInteractionStyle::from_egui_inactive`],
    /// [`TabInteractionStyle::from_egui_focused`], [`TabInteractionStyle::from_egui_hovered`], [`TabBodyStyle::from_egui`],
    pub fn from_egui(style: &egui::Style) -> TabStyle {
        Self {
            active: TabInteractionStyle::from_egui_active(style),
            inactive: TabInteractionStyle::from_egui_inactive(style),
            focused: TabInteractionStyle::from_egui_focused(style),
            hovered: TabInteractionStyle::from_egui_hovered(style),
            active_with_kb_focus: TabInteractionStyle::from_egui_active_with_kb_focus(style),
            inactive_with_kb_focus: TabInteractionStyle::from_egui_inactive_with_kb_focus(style),
            focused_with_kb_focus: TabInteractionStyle::from_egui_focused_with_kb_focus(style),
            tab_body: TabBodyStyle::from_egui(style),
            ..Default::default()
        }
    }
}

impl TabInteractionStyle {
    /// Derives relevant fields from `egui::Style` for an active tab and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`TabInteractionStyle::outline_color`]
    /// - [`TabInteractionStyle::bg_fill`]
    /// - [`TabInteractionStyle::text_color`]
    pub fn from_egui_active(style: &egui::Style) -> Self {
        Self {
            outline_color: style.visuals.widgets.noninteractive.bg_stroke.color,
            bg_fill: style.visuals.window_fill(),
            text_color: style.visuals.text_color(),
            corner_radius: CornerRadius {
                sw: 0,
                se: 0,
                ..style.visuals.widgets.active.corner_radius
            },
        }
    }

    /// Derives relevant fields from `egui::Style` for an inactive tab and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`TabInteractionStyle::outline_color`]
    /// - [`TabInteractionStyle::bg_fill`]
    /// - [`TabInteractionStyle::text_color`]
    pub fn from_egui_inactive(style: &egui::Style) -> Self {
        Self {
            text_color: style.visuals.text_color(),
            bg_fill: tint_color_towards(style.visuals.window_fill, style.visuals.extreme_bg_color),
            outline_color: tint_color_towards(
                style.visuals.widgets.noninteractive.bg_stroke.color,
                style.visuals.extreme_bg_color,
            ),
            ..TabInteractionStyle::from_egui_active(style)
        }
    }

    /// Derives relevant fields from `egui::Style` for a focused tab and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`TabInteractionStyle::outline_color`]
    /// - [`TabInteractionStyle::bg_fill`]
    /// - [`TabInteractionStyle::text_color`]
    pub fn from_egui_focused(style: &egui::Style) -> Self {
        Self {
            text_color: style.visuals.strong_text_color(),
            ..TabInteractionStyle::from_egui_active(style)
        }
    }

    /// Derives relevant fields from `egui::Style` for a hovered tab and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`TabInteractionStyle::outline_color`]
    /// - [`TabInteractionStyle::bg_fill`]
    /// - [`TabInteractionStyle::text_color`]
    pub fn from_egui_hovered(style: &egui::Style) -> Self {
        Self {
            text_color: style.visuals.strong_text_color(),
            outline_color: style.visuals.widgets.hovered.bg_stroke.color,
            ..TabInteractionStyle::from_egui_inactive(style)
        }
    }

    /// Derives relevant fields from `egui::Style` for an active tab with keyboard focus and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`TabInteractionStyle::outline_color`]
    /// - [`TabInteractionStyle::bg_fill`]
    /// - [`TabInteractionStyle::text_color`]
    pub fn from_egui_active_with_kb_focus(style: &egui::Style) -> Self {
        Self {
            text_color: style.visuals.strong_text_color(),
            outline_color: style.visuals.widgets.hovered.bg_stroke.color,
            ..TabInteractionStyle::from_egui_active(style)
        }
    }

    /// Derives relevant fields from `egui::Style` for an inactive tab with keyboard focus and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`TabInteractionStyle::outline_color`]
    /// - [`TabInteractionStyle::bg_fill`]
    /// - [`TabInteractionStyle::text_color`]
    pub fn from_egui_inactive_with_kb_focus(style: &egui::Style) -> Self {
        Self {
            text_color: style.visuals.strong_text_color(),
            outline_color: style.visuals.widgets.hovered.bg_stroke.color,
            ..TabInteractionStyle::from_egui_inactive(style)
        }
    }

    /// Derives relevant fields from `egui::Style` for a focused tab with keyboard focus and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`TabInteractionStyle::outline_color`]
    /// - [`TabInteractionStyle::bg_fill`]
    /// - [`TabInteractionStyle::text_color`]
    pub fn from_egui_focused_with_kb_focus(style: &egui::Style) -> Self {
        Self {
            text_color: style.visuals.strong_text_color(),
            outline_color: style.visuals.widgets.hovered.bg_stroke.color,
            ..TabInteractionStyle::from_egui_focused(style)
        }
    }
}

impl TabBodyStyle {
    /// Derives relevant fields from `egui::Style` and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`TabBodyStyle::inner_margin`]
    /// - [`TabBodyStyle::stroke]
    /// - [`TabBodyStyle::bg_fill`]
    pub fn from_egui(style: &egui::Style) -> Self {
        Self {
            inner_margin: style.spacing.window_margin,
            stroke: style.visuals.widgets.noninteractive.bg_stroke,
            corner_radius: style.visuals.widgets.active.corner_radius,
            bg_fill: style.visuals.window_fill(),
        }
    }
}

impl OverlayStyle {
    /// Derives relevant fields from `egui::Style` and sets the remaining fields to their default values.
    ///
    /// Fields overwritten by [`egui::Style`] are:
    /// - [`OverlayStyle::selection_color`]
    /// - [`OverlayStyle::button_spacing]
    /// - [`OverlayStyle::button_color`]
    /// - [`OverlayStyle::button_border_stroke`]
    pub fn from_egui(style: &egui::Style) -> Self {
        Self {
            selection_color: style.visuals.selection.bg_fill.linear_multiply(0.5),
            button_spacing: style.spacing.icon_spacing,
            button_color: style.visuals.widgets.noninteractive.fg_stroke.color,
            button_border_stroke: style.visuals.widgets.noninteractive.bg_stroke,
            ..Default::default()
        }
    }
}
