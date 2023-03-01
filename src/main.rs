mod rotating_grille;
use rotating_grille::*;

mod vigener_progressive;
use vigener_progressive::*;

use egui_dock::Tree;
use itertools::Itertools;

#[derive(Debug)]
enum EncryptTab {
    Vigener {
        input_text: String,
        output_text: String,
        key: String,
    },
    Grille {
        input_text: String,
        output_text: String,
        key: CardboardMatrix,
    },
}

struct TabViewer {}
impl egui_dock::TabViewer for TabViewer {
    type Tab = EncryptTab;
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            EncryptTab::Vigener {
                input_text,
                output_text,
                key,
            } => {
                ui.horizontal(|ui| {
                    ui.label("Ключ: ");
                    ui.text_edit_singleline(key);
                });

                ui.columns(2, |column| {
                    column[0].text_edit_multiline(input_text);
                    if column[0].button("Получить открытый текст").clicked() {
                        let vig = VigenerProgressive::new(key);
                        if let Some(vig) = vig {
                            *input_text = vig.decrypt(output_text);
                        }
                    }

                    column[1].text_edit_multiline(output_text);
                    if column[1].button("Получить зашифрованный текст").clicked()
                    {
                        let vig = VigenerProgressive::new(key);
                        if let Some(vig) = vig {
                            *output_text = vig.encrypt(input_text);
                        }
                    }
                })
            }
            EncryptTab::Grille {
                input_text,
                output_text,
                key,
            } => {
                //.chunks(4).map(|row| row.collect());
                if ui.button("Получить открытый текст").clicked() {
                    let grille = Grille::new(*key);
                    let grid = output_text
                        .chars()
                        .filter(|c| c.is_ascii_alphabetic())
                        .map(|c| c.to_ascii_uppercase())
                        .chunks(4)
                        .into_iter()
                        .map(|row| row.collect::<Vec<_>>())
                        .collect::<Vec<Vec<_>>>();

                    if let Ok(rows) = dbg!(grid)
                        .into_iter()
                        .map(|row| row.try_into())
                        .collect::<Result<Vec<[char; 4]>, _>>()
                    {
                        if let Ok(grid) = (rows).try_into() {
                            *input_text = grille.decrypt(grid);
                        }
                    }
                }
                ui.label("Открытый текст");
                ui.text_edit_singleline(input_text);
                ui.columns(2, |column| {
                    column[0].label("Ключ");
                    egui::Grid::new("key").show(&mut column[0], |ui| {
                        for i in 0..4 {
                            for j in 0..4 {
                                ui.add(egui::Checkbox::without_text(&mut key[i][j]));
                            }
                            ui.end_row();
                        }
                    });

                    if column[1].button("Получить зашифрованный текст").clicked()
                    {
                        let grille = Grille::new(*key);
                        let encrypted = grille.encrypt(input_text);
                        *output_text = encrypted
                            .map(|line| {
                                line.iter()
                                    .map(|c| c.to_string())
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            })
                            .join("\n");
                    }
                    column[1].label("Зашифрованный текст");
                    column[1].text_edit_multiline(output_text);
                });
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            EncryptTab::Vigener { .. } => "Vigener".into(),
            EncryptTab::Grille { .. } => "Grille".into(),
        }
    }
}

#[derive(Default)]
struct MyTabs {
    tree: Tree<EncryptTab>,
}

impl MyTabs {
    pub fn new() -> Self {
        let tab1 = EncryptTab::Grille {
            input_text: String::new(),
            output_text: String::new(),
            key: [[false; 4]; 4],
        };
        let tab2 = EncryptTab::Vigener {
            input_text: String::new(),
            output_text: String::new(),
            key: String::new(),
        };

        let tree = Tree::new(vec![tab1, tab2]);
        Self { tree }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let style = egui_dock::Style::from_egui(ui.style().as_ref());
        egui_dock::DockArea::new(&mut self.tree)
            .style(style)
            .show_inside(ui, &mut TabViewer {});
    }
}

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    tabs: MyTabs,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            tabs: MyTabs::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.tabs.ui(ui);
        });
    }
}
