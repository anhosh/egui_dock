//! # `egui_dock`: docking support for `egui`
//!
//! Originally created by [@lain-dono](https://github.com/lain-dono), this library provides docking support for `egui`.
//! It lets you open and close tabs, freely move them around, resize them, and undock them into new egui windows which
//! can also have other tabs docked in them.
//!
//! ## Basic usage
//!
//! The library is centered around the [`DockState`].
//! It contains a series of [`Surface`]s which all have their own [`Tree`].
//! Each [`Tree`] stores a hierarchy of [`Node`]s which contain the splits and tabs.
//!
//! [`DockState`] is generic (`DockState<Tab>`) so you can use any data to represent a tab.
//! You show the tabs using [`DockArea`] and specify how they are shown by implementing [`TabViewer`].
//!
//! ```rust
//! use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};
//! use egui::{Ui, WidgetText};
//!
//! // First, let's pick a type that we'll use to attach some data to each tab.
//! // It can be any type.
//! type Tab = String;
//!
//! // To define the contents and properties of individual tabs, we implement the `TabViewer`
//! // trait. Only three things are mandatory: the `Tab` associated type, and the `ui` and
//! // `title` methods. There are more methods in `TabViewer` which you can also override.
//! struct MyTabViewer;
//!
//! impl TabViewer for MyTabViewer {
//!     // This associated type is used to attach some data to each tab.
//!     type Tab = Tab;
//!
//!     // Returns the current `tab`'s title.
//!     fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
//!         tab.as_str().into()
//!     }
//!
//!     // Defines the contents of a given `tab`.
//!     fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
//!         ui.label(format!("Content of {tab}"));
//!     }
//! }
//!
//! // Here is a simple example of how you can manage a `DockState` of your application.
//! struct MyTabs {
//!     dock_state: DockState<Tab>
//! }
//!
//! impl MyTabs {
//!     pub fn new() -> Self {
//!         // Create a `DockState` with an initial tab "tab1" in the main `Surface`'s root node.
//!         let tabs = ["tab1", "tab2", "tab3"].map(str::to_string).into_iter().collect();
//!         let dock_state = DockState::new(tabs);
//!         Self { dock_state }
//!     }
//!
//!     fn ui(&mut self, ui: &mut Ui) {
//!         // Here we just display the `DockState` using a `DockArea`.
//!         // This is where egui handles rendering and all the integrations.
//!         //
//!         // We can specify a custom `Style` for the `DockArea`, or just inherit
//!         // all of it from egui.
//!         DockArea::new(&mut self.dock_state)
//!             .style(Style::from_egui(ui.style().as_ref()))
//!             .show_inside(ui, &mut MyTabViewer);
//!     }
//! }
//!
//! # let mut my_tabs = MyTabs::new();
//! # egui::__run_test_ctx(|ctx| {
//! #     #[allow(deprecated)]
//! #     egui::CentralPanel::default().show(ctx, |ui| my_tabs.ui(ui));
//! # });
//! ```
//!
//! ## Look and feel customization
//!
//! `egui_dock` exposes the [`Style`] struct that lets you change how tabs and the [`DockArea`]
//! should look and feel. [`Style`] is divided into several, more specialized structs that handle
//! individual elements of the UI.
//!
//! Your [`Style`] can inherit all its properties from an [`egui::Style`] through the
//! [`Style::from_egui`] function.
//!
//! Example:
//!
//! ```rust
//! # use egui_dock::{DockArea, DockState, OverlayType, Style, TabAddAlign, TabViewer};
//! # use egui::{Ui, WidgetText};
//! # struct MyTabViewer;
//! # impl TabViewer for MyTabViewer {
//! #     type Tab = ();
//! #     fn title(&mut self, tab: &mut Self::Tab) -> WidgetText { WidgetText::default() }
//! #     fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {}
//! # }
//! # egui::__run_test_ctx(|ctx| {
//! # #[allow(deprecated)]
//! # egui::CentralPanel::default().show(ctx, |ui| {
//! # let mut dock_state = DockState::new(vec![]);
//! // Inherit the look and feel from egui.
//! let mut style = Style::from_egui(ui.style());
//!
//! // Modify a few fields.
//! style.overlay.overlay_type = OverlayType::HighlightedAreas;
//! style.buttons.add_tab_align = TabAddAlign::Left;
//!
//! // Use the style with the `DockArea`.
//! DockArea::new(&mut dock_state)
//!     .style(style)
//!     .show_inside(ui, &mut MyTabViewer);
//! # });
//! # });
//! #
//! ```
//!
//! ## Surfaces
//!
//! A [`Surface`] is an abstraction for any tab hierarchy. There are two kinds of
//! non-empty surfaces: `Main` and `Window`.
//!
//! There can only be one `Main` surface. It's the one surface that is rendered inside the
//! [`Ui`](egui::Ui) you've passed to [`DockArea::show_inside`], or inside the
//! [`egui::CentralPanel`] created by [`DockArea::show`].
//!
//! On the other hand, there can be multiple `Window` surfaces. Those represent surfaces that were
//! created by undocking tabs from the `Main` surface, and each of them is rendered inside
//! a [`egui::Window`] - hence their name.
//!
//! While most of surface management will be done by the user of your application, you can also do it
//! programatically using the [`DockState`] API.
//!
//! Example:
//!
//! ```rust
//! # use egui_dock::DockState;
//! # use egui::{Pos2, Vec2};
//! # let mut dock_state = DockState::new(vec![]);
//! // Create a new window `Surface` with one tab inside it.
//! let mut surface_index = dock_state.add_window(vec!["Window Tab".to_string()]);
//!
//! // Access the window state by its surface index and then move and resize it.
//! let window_state = dock_state.get_window_state_mut(surface_index).unwrap();
//! window_state.set_position(Pos2::ZERO);
//! window_state.set_size(Vec2::splat(100.0));
//! ```
//!
//! For more details, see: [`DockState`].
//!
//! ## Trees
//!
//! In each [`Surface`] there is a [`Tree`] which actually stores the tabs. As the name suggests,
//! tabs and splits are represented with a binary tree.
//!
//! The [`Tree`] API allows you to programatically manipulate the dock layout.
//!
//! Example:
//!
//! ```rust
//! # use egui_dock::{DockState, NodeIndex};
//! // Create a `DockState` with an initial tab "tab1" in the main `Surface`'s root node.
//! let mut dock_state = DockState::new(vec!["tab1".to_string()]);
//!
//! // Currently, the `DockState` only has one `Surface`: the main one.
//! // Let's get mutable access to add more nodes in it.
//! let surface = dock_state.main_surface_mut();
//!
//! // Insert "tab2" to the left of "tab1", where the width of "tab2"
//! // is 20% of root node's width.
//! let [_old_node, new_node] =
//!     surface.split_left(NodeIndex::root(), 0.20, vec!["tab2".to_string()]);
//!
//! // Insert "tab3" below "tab2" with both tabs having equal size.
//! surface.split_below(new_node, 0.5, vec!["tab3".to_string()]);
//!
//! // The layout will look similar to this:
//! // +--------+--------------------------------+
//! // |        |                                |
//! // |  tab2  |                                |
//! // |        |                                |
//! // +--------+              tab1              |
//! // |        |                                |
//! // |  tab3  |                                |
//! // |        |                                |
//! // +--------+--------------------------------+
//! ```
//!
//! ## Translations
//!
//! Some parts of the [`DockArea`] contain text that has nothing to do with tab content (currently it's just the
//! tab context menus, but that might change in the future). The [`translations`] module provides an API for defining
//! an alternative for each text element. This is especially useful when your application's interface is in any
//! language other than English, but can also be used in any other way, e.g. to add icons.
//!
//! Example usage:
//!
//! ```rust
//! # use egui_dock::{DockState, TabContextMenuTranslations, Translations, LeafTranslations};
//! # type Tab = ();
//! let translations_pl = Translations {
//!     tab_context_menu: TabContextMenuTranslations {
//!         close_button: "Zamknij zakładkę".to_string(),
//!         eject_button: "Przenieś zakładkę do nowego okna".to_string(),
//!     },
//!     leaf: LeafTranslations {
//!         close_button_disabled_tooltip: "Ten węzeł zawiera niezamykalne zakładki.".to_string(),
//!         close_all_button: "Zamknij okno".to_string(),
//!         close_all_button_menu_hint: "Kliknij prawym przyciskiem myszy, aby zamknąć to okno.".to_string(),
//!         close_all_button_modifier_hint: "Naciśnij klawisze modyfikujące (domyślnie Shift), aby zamknąć to okno.".to_string(),
//!         close_all_button_modifier_menu_hint: "Naciśnij klawisze modyfikujące (domyślnie Shift) lub kliknij prawym przyciskiem myszy, aby zamknąć to okno.".to_string(),
//!         close_all_button_disabled_tooltip: "To okno zawiera zakładki, których nie można zamknąć.".to_string(),
//!         minimize_button: "Zminimalizuj okno".to_string(),
//!         minimize_button_menu_hint: "Kliknij prawym przyciskiem myszy, aby zminimalizować to okno.".to_string(),
//!         minimize_button_modifier_hint: "Naciśnij klawisze modyfikujące (domyślnie Shift), aby zminimalizować to okno.".to_string(),
//!         minimize_button_modifier_menu_hint: "Naciśnij klawisze modyfikujące (domyślnie Shift) lub kliknij prawym przyciskiem myszy, aby zminimalizować to okno.".to_string(),
//!     }
//! };
//! let dock_state = DockState::<Tab>::new(vec![]).with_translations(translations_pl);
//!
//! // Alternatively:
//! let mut dock_state = DockState::<Tab>::new(vec![]);
//! dock_state.translations.tab_context_menu.close_button = "タブを閉じる".to_string();
//! dock_state.translations.tab_context_menu.eject_button = "タブを新しいウィンドウへ移動".to_string();
//! dock_state.translations.leaf.close_button_disabled_tooltip = "このノードは閉じられないタブがある".to_string();
//! dock_state.translations.leaf.close_all_button = "ウィンドウを閉じる".to_string();
//! dock_state.translations.leaf.close_all_button_menu_hint = "右クリックでこのウィンドウを閉じる".to_string();
//! dock_state.translations.leaf.close_all_button_modifier_hint = "修飾キー（デフォルトではShift）を押して、このウィンドウを閉じます".to_string();
//! dock_state.translations.leaf.close_all_button_modifier_menu_hint = "修飾キー（デフォルトではShift）を押すか、右クリックしてこのウィンドウを閉じます".to_string();
//! dock_state.translations.leaf.close_all_button_disabled_tooltip = "このウィンドウは閉じられないタブがある".to_string();
//! dock_state.translations.leaf.minimize_button = "ウィンドウを最小化する".to_string();
//! dock_state.translations.leaf.minimize_button_menu_hint = "右クリックでウィンドウを最小化する".to_string();
//! dock_state.translations.leaf.minimize_button_modifier_hint = "修飾キー（デフォルトではShift）を押すと、このウィンドウが最小化されます".to_string();
//! dock_state.translations.leaf.minimize_button_modifier_menu_hint = "修飾キー（デフォルトではShift）を押すか、右クリックしてこのウィンドウを最小化する".to_string();
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]

#[allow(deprecated)]
pub use dock_state::*;
pub use egui;
pub use style::*;
pub use translations::*;
pub use tree::*;
pub use widgets::*;

/// The main structure of the library.
pub mod dock_state;

/// Look and feel.
pub mod style;

/// Widgets provided by the library.
pub mod widgets;

mod utils;
