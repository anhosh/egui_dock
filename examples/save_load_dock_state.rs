#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs;

use eframe::{egui, NativeOptions};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

const DOCK_STATE_FILE: &str = "target/dock_state.json";

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct TabViewer {
    modified: bool,
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }

    fn on_rect_changed(&mut self, _tab: &mut Self::Tab) {
        self.modified = true
    }
}

struct MyApp {
    tree: DockState<String>,
}

impl MyApp {
    fn save_json(&self) {
        if let Ok(json) = serde_json::to_string(&self.tree) {
            let _ = fs::write(DOCK_STATE_FILE, json);
        }
    }

    fn load_json() -> Option<Self> {
        fs::read_to_string(DOCK_STATE_FILE)
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .map(|tree| Self { tree })
    }
}

impl Default for MyApp {
    fn default() -> Self {
        // Try loading from file, fallback to default layout
        Self::load_json().unwrap_or_else(|| {
            let mut tree = DockState::new(vec!["tab1".to_owned(), "tab2".to_owned()]);

            let [a, b] =
                tree.main_surface_mut()
                    .split_left(NodeIndex::root(), 0.3, vec!["tab3".to_owned()]);
            let [_, _] = tree
                .main_surface_mut()
                .split_below(a, 0.7, vec!["tab4".to_owned()]);
            let [_, _] = tree
                .main_surface_mut()
                .split_below(b, 0.5, vec!["tab5".to_owned()]);

            Self { tree }
        })
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut tab_viewer = TabViewer { modified: false };
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
        if tab_viewer.modified {
            self.save_json();
        }
    }
}
