#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{NativeOptions, egui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Hidden Tabs",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "Visible" => {
                ui.label("The tab bar is showing.");
                ui.label(
                    "Right click a tab and press 'Hide Tab Bar' to collapse the tab bar region.",
                );
            }
            "Hidden" => {
                ui.label("The tab bar is hidden.");
                ui.label("Try dragging by the top margin of the tab when the tab bar is hidden.");
            }
            _ => {}
        }
    }
}

struct MyApp {
    tree: DockState<String>,
    style: Option<Style>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec!["Visible".to_owned()]);

        let [_, b] =
            tree.main_surface_mut()
                .split_right(NodeIndex::root(), 0.5, vec!["Hidden".to_owned()]);

        // Hide the tab bar on the right panel
        tree.main_surface_mut()[b].set_tab_bar_hidden(true);

        Self { tree, style: None }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let style = self
            .style
            .get_or_insert_with(|| {
                let mut s = Style::from_egui(ui.style());
                // Set a larger drag area for hidden tab bars
                s.tab.tab_body.hidden_tab_bar_drag_height = Some(14.0);
                s
            })
            .clone();

        DockArea::new(&mut self.tree)
            .style(style)
            .hidable_tab_bars(true)
            .draggable_tabs(true)
            .show_inside(ui, &mut TabViewer {});
    }
}
