#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, NativeOptions};
use egui::{Color32, RichText};
use egui_dock::{DockArea, DockState, NodeIndex, NodePath, Style, SurfaceIndex};

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

enum MyTabKind {
    Regular,
    Fancy,
}

struct MyTab {
    kind: MyTabKind,
    path: NodePath,
}

impl MyTab {
    fn regular(node: NodePath) -> Self {
        Self {
            kind: MyTabKind::Regular,
            path: node,
        }
    }

    fn fancy(node: NodePath) -> Self {
        Self {
            kind: MyTabKind::Fancy,
            path: node,
        }
    }

    fn title(&self) -> String {
        match self.kind {
            MyTabKind::Regular => format!("Regular Tab {}", self.path.node.0),
            MyTabKind::Fancy => format!("Fancy Tab {}", self.path.node.0),
        }
    }

    fn content(&self) -> RichText {
        match self.kind {
            MyTabKind::Regular => {
                RichText::new(format!("Content of {}. This tab is ho-hum.", self.title()))
            }
            MyTabKind::Fancy => RichText::new(format!(
                "Content of {}. This tab sure is fancy!",
                self.title()
            ))
            .italics()
            .size(20.0)
            .color(Color32::from_rgb(255, 128, 64)),
        }
    }
}

struct TabViewer<'a> {
    added_nodes: &'a mut Vec<MyTab>,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = MyTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(tab.content());
    }

    fn add_popup(&mut self, ui: &mut egui::Ui, path: NodePath) {
        ui.set_min_width(120.0);
        ui.style_mut().visuals.button_frame = false;

        if ui.button("Regular tab").clicked() {
            self.added_nodes.push(MyTab::regular(path));
        }

        if ui.button("Fancy tab").clicked() {
            self.added_nodes.push(MyTab::fancy(path));
        }
    }
}

struct MyApp {
    dock_state: DockState<MyTab>,
    counter: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec![
            MyTab::regular(NodePath {
                surface: SurfaceIndex::main(),
                node: NodeIndex(1),
            }),
            MyTab::fancy(NodePath {
                surface: SurfaceIndex::main(),
                node: NodeIndex(2),
            }),
        ]);

        // You can modify the tree before constructing the dock
        let [a, b] = tree.main_surface_mut().split_left(
            NodeIndex::root(),
            0.3,
            vec![MyTab::fancy(NodePath {
                surface: SurfaceIndex::main(),
                node: NodeIndex(3),
            })],
        );
        let [_, _] = tree.main_surface_mut().split_below(
            a,
            0.7,
            vec![MyTab::fancy(NodePath {
                surface: SurfaceIndex::main(),
                node: NodeIndex(4),
            })],
        );
        let [_, _] = tree.main_surface_mut().split_below(
            b,
            0.5,
            vec![MyTab::regular(NodePath {
                surface: SurfaceIndex::main(),
                node: NodeIndex(5),
            })],
        );

        Self {
            dock_state: tree,
            counter: 6,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut added_nodes = Vec::new();
        DockArea::new(&mut self.dock_state)
            .show_add_buttons(true)
            .show_add_popup(true)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(
                ctx,
                &mut TabViewer {
                    added_nodes: &mut added_nodes,
                },
            );

        added_nodes.drain(..).for_each(|node| {
            self.dock_state.set_focused_node_and_surface(node.path);
            self.dock_state.push_to_focused_leaf(MyTab {
                kind: node.kind,
                path: NodePath {
                    surface: node.path.surface,
                    node: NodeIndex(self.counter),
                },
            });
            self.counter += 1;
        });
    }
}
