#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, NativeOptions};
use egui_dock::{DockArea, DockState, NodeIndex, NodePath, Style};

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct TabViewer<'a> {
    added_nodes: &'a mut Vec<NodePath>,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = usize;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        format!("Tab {tab}").into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of tab {tab}"));
    }

    fn on_add(&mut self, path: NodePath) {
        self.added_nodes.push(path);
    }
}

struct MyApp {
    tree: DockState<usize>,
    counter: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec![1, 2]);

        // You can modify the tree before constructing the dock
        let [a, b] = tree
            .main_surface_mut()
            .split_left(NodeIndex::root(), 0.3, vec![3]);
        let [_, _] = tree.main_surface_mut().split_below(a, 0.7, vec![4]);
        let [_, _] = tree.main_surface_mut().split_below(b, 0.5, vec![5]);

        Self { tree, counter: 6 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut added_nodes = Vec::new();
        DockArea::new(&mut self.tree)
            .show_add_buttons(true)
            .style({
                let mut style = Style::from_egui(ctx.style().as_ref());
                style.tab_bar.fill_tab_bar = true;
                style
            })
            .show(
                ctx,
                &mut TabViewer {
                    added_nodes: &mut added_nodes,
                },
            );

        added_nodes.drain(..).for_each(|path| {
            self.tree.set_focused_node_and_surface(path);
            self.tree.push_to_focused_leaf(self.counter);
            self.counter += 1;
        });
    }
}
