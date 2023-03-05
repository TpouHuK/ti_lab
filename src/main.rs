mod rotating_grille;
use rotating_grille::*;

mod vigener_progressive;
use vigener_progressive::*;

use egui_dock::Tree;
use itertools::Itertools;

use im_native_dialog::ImNativeFileDialog;
use std::path::PathBuf;

enum EncryptTab {
    Vigener {
        input_file_read_path_dialog: ImNativeFileDialog<Option<PathBuf>>,
        input_file_write_path_dialog: ImNativeFileDialog<Option<PathBuf>>,
        output_file_read_path_dialog: ImNativeFileDialog<Option<PathBuf>>,
        output_file_write_path_dialog: ImNativeFileDialog<Option<PathBuf>>,

        input_text: String,
        output_text: String,
        key: String,
    },
    Grille {
        input_file_read_path_dialog: ImNativeFileDialog<Option<PathBuf>>,
        input_file_write_path_dialog: ImNativeFileDialog<Option<PathBuf>>,
        output_file_read_path_dialog: ImNativeFileDialog<Option<PathBuf>>,
        output_file_write_path_dialog: ImNativeFileDialog<Option<PathBuf>>,

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
                input_file_read_path_dialog,
                input_file_write_path_dialog,
                output_file_read_path_dialog,
                output_file_write_path_dialog,

                input_text,
                output_text,
                key,
            } => {
                ui.horizontal(|ui| {
                    ui.label("Ключ: ");
                    ui.text_edit_singleline(key);
                });

                if let Some(Ok(Some(path))) = input_file_read_path_dialog.check() {
                    if let Ok(str) = std::fs::read_to_string(path) {
                        *input_text = str;
                    }
                }

                if let Some(Ok(Some(path))) = input_file_write_path_dialog.check() {
                    std::fs::write(path, &input_text);
                }

                if let Some(Ok(Some(path))) = output_file_read_path_dialog.check() {
                    if let Ok(str) = std::fs::read_to_string(path) {
                        *output_text = str;
                    }
                }

                if let Some(Ok(Some(path))) = output_file_write_path_dialog.check() {
                    std::fs::write(path, &input_text);
                }

                let right_key = filter_russian(key.chars()).count() > 0;

                ui.columns(2, |column| {
                    column[0].text_edit_multiline(input_text);
                    if column[0]
                        .add_enabled(right_key, egui::Button::new("Получить открытый текст"))
                        .clicked()
                    {
                        let vig = VigenerProgressive::new(key);
                        if let Some(vig) = vig {
                            *input_text = vig.decrypt(output_text);
                        }
                    }

                    if column[0]
                        .button("Загрузить открытый текст из файла")
                        .clicked()
                    {
                        input_file_read_path_dialog
                            .open_single_file(None)
                            .expect("Unable to open file_path dialog");
                    }

                    if column[0]
                        .button("Сохранить открытый текст в файл")
                        .clicked()
                    {
                        input_file_write_path_dialog
                            .show_save_single_file(None)
                            .expect("Unable to open file_path dialog");
                    }

                    column[1].text_edit_multiline(output_text);
                    if column[1]
                        .add_enabled(right_key, egui::Button::new("Получить зашифрованный текст"))
                        .clicked()
                    {
                        let vig = VigenerProgressive::new(key);
                        if let Some(vig) = vig {
                            *output_text = vig.encrypt(input_text);
                        }
                    }

                    if column[1]
                        .button("Загрузить шифрованный текст из файла")
                        .clicked()
                    {
                        output_file_read_path_dialog
                            .open_single_file(None)
                            .expect("Unable to open file_path dialog");
                    }

                    if column[1]
                        .button("Сохранить шифрованный текст в файл")
                        .clicked()
                    {
                        output_file_write_path_dialog
                            .show_save_single_file(None)
                            .expect("Unable to open file_path dialog");
                    }
                })
            }
            EncryptTab::Grille {
                input_file_read_path_dialog,
                input_file_write_path_dialog,
                output_file_read_path_dialog,
                output_file_write_path_dialog,

                input_text,
                output_text,
                key,
            } => {
                if let Some(Ok(Some(path))) = input_file_read_path_dialog.check() {
                    if let Ok(str) = std::fs::read_to_string(path) {
                        *input_text = str;
                    }
                }

                if let Some(Ok(Some(path))) = input_file_write_path_dialog.check() {
                    std::fs::write(path, &input_text);
                }

                if let Some(Ok(Some(path))) = output_file_read_path_dialog.check() {
                    if let Ok(str) = std::fs::read_to_string(path) {
                        *output_text = str;
                    }
                }

                if let Some(Ok(Some(path))) = output_file_write_path_dialog.check() {
                    std::fs::write(path, &input_text);
                }

                let right_key = key.iter().flatten().filter(|t| **t).count() == 4;
                //.chunks(4).map(|row| row.collect());
                if ui
                    .add_enabled(right_key, egui::Button::new("Получить открытый текст"))
                    .clicked()
                {
                    let grille = Grille::new(*key);
                    let mut chars = output_text
                        .chars()
                        .filter(|c| c.is_ascii_alphabetic())
                        .map(|c| c.to_ascii_uppercase());

                    let mut output = String::new();
                    'outer: loop {
                        let mut char_matrix = [[' '; 4]; 4];
                        for row in &mut char_matrix {
                            for ch in row {
                                if let Some(char) = chars.next() {
                                    *ch = char;
                                } else {
                                    break 'outer;
                                }
                            }
                        }
                        output.push_str(&grille.decrypt(char_matrix));
                    }
                    *input_text = output;
                }
                ui.columns(2, |column| {
                    column[0].label("Открытый текст");
                    column[0].text_edit_singleline(input_text);

                    if column[0]
                        .button("Загрузить открытый текст из файла")
                        .clicked()
                    {
                        input_file_read_path_dialog
                            .open_single_file(None)
                            .expect("Unable to open file_path dialog");
                    }

                    if column[0]
                        .button("Сохранить открытый текст в файл")
                        .clicked()
                    {
                        input_file_write_path_dialog
                            .show_save_single_file(None)
                            .expect("Unable to open file_path dialog");
                    }

                    column[0].label("Ключ");
                    egui::Grid::new("key").show(&mut column[0], |ui| {
                        let a = rot_90(key.to_owned());
                        let b = rot_90(a);
                        let c = rot_90(b);
                        let d = rot_90(c);

                        for i in 0..4 {
                            for j in 0..4 {
                                let mut is_enabled = true;
                                if (a[i][j] || b[i][j] || c[i][j] || d[i][j]) && !(key[i][j]) {
                                    is_enabled = false;
                                }
                                ui.add_enabled(
                                    is_enabled,
                                    egui::Checkbox::without_text(&mut key[i][j]),
                                );
                            }
                            ui.end_row();
                        }
                    });

                    if column[1]
                        .add_enabled(right_key, egui::Button::new("Получить зашифрованный текст"))
                        .clicked()
                    {
                        let grille = Grille::new(*key);
                        let mut out = String::new();

                        for square in input_text
                            .chars()
                            .filter(|c| c.is_ascii_alphabetic())
                            .map(|c| c.to_ascii_uppercase())
                            .chunks(4 * 4)
                            .into_iter()
                        {
                            let encrypted = grille.encrypt(&square.collect::<String>());
                            out.push_str(
                                &encrypted
                                    .map(|line| {
                                        line.iter()
                                            .map(|c| c.to_string())
                                            .collect::<Vec<_>>()
                                            .join(" ")
                                    })
                                    .join("\n"),
                            );
                            out.push_str("\n\n");
                        }
                        *output_text = out;
                    }
                    column[1].label("Зашифрованный текст");
                    column[1].text_edit_multiline(output_text);

                    if column[1]
                        .button("Загрузить шифрованный текст из файла")
                        .clicked()
                    {
                        output_file_read_path_dialog
                            .open_single_file(None)
                            .expect("Unable to open file_path dialog");
                    }

                    if column[1]
                        .button("Сохранить шифрованный текст в файл")
                        .clicked()
                    {
                        output_file_write_path_dialog
                            .show_save_single_file(None)
                            .expect("Unable to open file_path dialog");
                    }
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
            input_file_read_path_dialog: Default::default(),
            input_file_write_path_dialog: Default::default(),
            output_file_read_path_dialog: Default::default(),
            output_file_write_path_dialog: Default::default(),

            input_text: String::new(),
            output_text: String::new(),
            key: [[false; 4]; 4],
        };
        let tab2 = EncryptTab::Vigener {
            input_file_read_path_dialog: Default::default(),
            input_file_write_path_dialog: Default::default(),
            output_file_read_path_dialog: Default::default(),
            output_file_write_path_dialog: Default::default(),

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
