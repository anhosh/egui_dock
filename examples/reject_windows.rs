#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, NativeOptions};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct TabViewer;

struct OpinionatedTab {
    can_become_window: Result<bool, bool>,
    title: String,
    content: String,
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = OpinionatedTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&tab.title).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(&tab.content);
        match &mut tab.can_become_window {
            Ok(changing_opinion) => {
                ui.add(egui::Checkbox::new(
                    changing_opinion,
                    "can be turned into window",
                ));
            }
            Err(fixed_opinion) => {
                if *fixed_opinion {
                    ui.small("this tab can exist in a window");
                } else {
                    ui.small("this tab cannot exist in a window");
                }
            }
        }
    }

    fn allowed_in_windows(&self, tab: &mut Self::Tab) -> bool {
        match tab.can_become_window {
            Ok(opinion) | Err(opinion) => opinion,
        }
    }
}

struct MyApp {
    tree: DockState<OpinionatedTab>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec![
            OpinionatedTab {
                can_become_window: Ok(false),
                title: "old tab".to_owned(),
                content: "since when could tabs become windows?".to_string(),
            },
            OpinionatedTab {
                can_become_window: Err(false),
                title: "grumpy tab".to_owned(),
                content: "I don't want to be a window!".to_string(),
            },
        ]);

        // You can modify the tree before constructing the dock
        let [a, _] = tree.main_surface_mut().split_right(
            NodeIndex::root(),
            0.6,
            vec![OpinionatedTab {
                can_become_window: Ok(true),
                title: "wise tab".to_owned(),
                content: "egui_dock 0.7!".to_string(),
            }],
        );
        let [_, _] = tree.main_surface_mut().split_below(
            a,
            0.4,
            vec![OpinionatedTab {
                can_become_window: Ok(true),
                title: "instructional tab".to_owned(),
                content: "This demo is meant to showcase the ability for tabs to become/be placed inside windows. 
                \nindividual tabs have the ability to accept/reject being put/turned into a window. 
                \nIn this demo some tabs have a fixed opinion on this, others can be swayed with the click of a checkbox. 
                \n\n In your app you yourself may decide how tabs behave, but for now try dragging some tabs into empty space to turn them into windows!"
                .to_string(),
            }],
        );
        let _ = tree.add_window(vec![OpinionatedTab {
            can_become_window: Err(true),
            title: "egotistical tab".to_owned(),
            content: "im above you all!".to_string(),
        }]);

        Self { tree }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut TabViewer {});
    }
}
