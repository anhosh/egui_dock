#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(not(feature = "taffy"))]
fn main() {
    #[cfg(not(feature = "taffy"))]
    println!("This example requires the `taffy` feature");
}

#[cfg(feature = "taffy")]
fn main() -> eframe::Result<()> {
    with_taffy::run()
}

#[cfg(feature = "taffy")]
mod with_taffy {
    use eframe::NativeOptions;
    use egui::{TextEdit, WidgetText};
    use egui_dock::{DockArea, DockState, NodeIndex};
    use egui_taffy::taffy::prelude::{auto, fit_content, fr, length, percent, span};
    use egui_taffy::taffy::{
        AlignContent, AlignItems, AlignSelf, Display, FlexDirection, Size, Style,
    };
    use egui_taffy::TuiBuilderLogic;

    pub fn run() -> eframe::Result<()> {
        let options = NativeOptions::default();
        eframe::run_native(
            "My egui App with taffy",
            options,
            Box::new(|cc| {
                // Set solid scrollbars for the entire app
                cc.egui_ctx.style_mut(|style| {
                    // FIXME this frequently causes taffy to flash an orange line and an 'unaligned' message where the scrollbars appear.
                    //       is the bug in egui_taffy or in egui_dock?
                    style.spacing.scroll = egui::style::ScrollStyle::solid();
                });

                Ok(Box::<MyApp>::default())
            }),
        )
    }

    #[derive(Default)]
    struct TabState {
        title: String,
        text: String,
    }

    impl From<String> for TabState {
        fn from(value: String) -> Self {
            TabState {
                title: value,
                text: Default::default(),
            }
        }
    }

    struct TabViewer {}

    impl egui_dock::TabViewer for TabViewer {
        type Tab = TabState;

        fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
            WidgetText::from(&tab.title)
        }

        fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
            egui_taffy::tui(ui, ui.id().with("taffy_tab"))
                .reserve_available_space()
                .style(Style {
                    flex_direction: FlexDirection::Row,
                    align_self: Some(AlignSelf::Stretch),
                    size: Size {
                        width: percent(1.),
                        height: auto(),
                    },
                    min_size: Size {
                        width: length(200.0),
                        height: length(50.0),
                    },
                    ..Style::default()
                })
                .show(|tui| {
                    //
                    // grid container
                    //
                    tui.style(Style {
                        flex_grow: 1.0,
                        display: Display::Grid,
                        grid_template_columns: vec![fit_content(percent(1.)), fr(1.)],
                        grid_template_rows: vec![fr(1.), fr(1.)],

                        // ensure items are centered vertically on rows
                        align_items: Some(AlignItems::Center),
                        ..Style::default()
                    })
                    .add_with_border(|tui| {
                        let row_style = || Style {
                            padding: length(2.),
                            gap: length(2.),
                            ..Default::default()
                        };

                        //
                        // row 1
                        //

                        // cell 1
                        tui.style(Style { ..row_style() }).add_with_border(|tui| {
                            tui.label("Label");
                        });

                        // cell 1
                        tui.style(Style {
                            flex_grow: 1.0,
                            min_size: Size {
                                width: length(100.0),
                                height: auto(),
                            },
                            ..row_style()
                        })
                        .add_with_border(|tui| {
                            // container
                            tui.style(Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                flex_grow: 1.0,
                                ..Style::default()
                            })
                            .add_with_border(|tui| {
                                // left
                                tui.style(Style {
                                    flex_grow: 1.0,
                                    min_size: Size {
                                        width: length(100.0),
                                        height: auto(),
                                    },
                                    ..Style::default()
                                })
                                .ui(|ui| {
                                    // FIXME this exapands, but does not contract, instead egui_dock shows a scrollbar in the tab.
                                    TextEdit::singleline(&mut tab.text)
                                        .hint_text("try typing...")
                                        .desired_width(ui.available_width())
                                        .show(ui);
                                });

                                // right
                                if tui
                                    .style(Style {
                                        flex_grow: 0.0,
                                        ..Style::default()
                                    })
                                    .button(|tui| tui.label("Add"))
                                    .clicked()
                                {
                                    println!("Add button clicked");
                                }
                            });
                        });

                        //
                        // row 2
                        //

                        // cell 1 and 2 (spanned)
                        tui.style(Style {
                            grid_column: span(2),
                            ..row_style()
                        })
                        .add_with_border(|tui| {
                            tui.label("Spanned");
                        });

                        //
                        // row 3
                        //

                        // cell 1
                        tui.style(Style { ..row_style() }).add_with_border(|tui| {
                            tui.label("A button");
                        });

                        // cell 2
                        tui.style(Style { ..row_style() }).add_with_border(|tui| {
                            if tui
                                .style(Style {
                                    flex_grow: 1.0,
                                    justify_content: Some(AlignContent::Center),
                                    ..Style::default()
                                })
                                .button(|tui| {
                                    tui.label("Click me!");
                                })
                                .clicked()
                            {
                                println!("Button clicked");
                            }
                        });
                    })
                });
        }
    }

    struct MyApp {
        tree: DockState<TabState>,
    }

    impl Default for MyApp {
        fn default() -> Self {
            let mut tree = DockState::new(vec!["tab1".to_owned().into(), "tab2".to_owned().into()]);

            // You can modify the tree before constructing the dock
            let [a, b] = tree.main_surface_mut().split_left(
                NodeIndex::root(),
                0.3,
                vec!["tab3".to_owned().into()],
            );
            let [_, _] =
                tree.main_surface_mut()
                    .split_below(a, 0.7, vec!["tab4".to_owned().into()]);
            let [_, _] =
                tree.main_surface_mut()
                    .split_below(b, 0.5, vec!["tab5".to_owned().into()]);

            Self { tree }
        }
    }

    impl eframe::App for MyApp {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            // Disable text wrapping
            //
            // egui text layouting tries to utilize minimal width possible
            ctx.style_mut(|style| {
                style.wrap_mode = Some(egui::TextWrapMode::Extend);
            });

            DockArea::new(&mut self.tree)
                .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
                .show(ctx, &mut TabViewer {});
        }
    }
}
