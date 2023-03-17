mod rotating_grille;
use egui::vec2;
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
                    std::fs::write(path, &input_text).unwrap();
                }

                if let Some(Ok(Some(path))) = output_file_read_path_dialog.check() {
                    if let Ok(str) = std::fs::read_to_string(path) {
                        *output_text = str;
                    }
                }

                if let Some(Ok(Some(path))) = output_file_write_path_dialog.check() {
                    std::fs::write(path, &output_text).unwrap();
                }

                let right_key = filter_russian(key.chars()).count() > 0;

                ui.columns(2, |column| {
                    column[0].group(|ui| {
                        ui.label("Открытый текст");
                        ui.text_edit_multiline(input_text);
                        if ui
                            .add_enabled(right_key, egui::Button::new("Получить (расшифровать)"))
                            .clicked()
                        {
                            let vig = VigenerProgressive::new(key);
                            if let Some(vig) = vig {
                                *input_text = vig.decrypt(output_text);
                            }
                        }

                        ui.horizontal(|ui| {
                            if ui.button("Загрузить...").clicked() {
                                input_file_read_path_dialog
                                    .open_single_file(None)
                                    .expect("Unable to open file_path dialog");
                            }

                            if ui.button("Сохранить...").clicked() {
                                input_file_write_path_dialog
                                    .show_save_single_file(None)
                                    .expect("Unable to open file_path dialog");
                            }
                        });
                    });

                    column[1].group(|ui| {
                        ui.label("Зашифрованный текст");
                        ui.text_edit_multiline(output_text);
                        if ui
                            .add_enabled(right_key, egui::Button::new("Получить (зашифровать)"))
                            .clicked()
                        {
                            let vig = VigenerProgressive::new(key);
                            if let Some(vig) = vig {
                                *output_text = vig.encrypt(input_text);
                            }
                        }

                        ui.horizontal(|ui| {
                            if ui.button("Загрузить...").clicked() {
                                output_file_read_path_dialog
                                    .open_single_file(None)
                                    .expect("Unable to open file_path dialog");
                            }

                            if ui.button("Сохранить...").clicked() {
                                output_file_write_path_dialog
                                    .show_save_single_file(None)
                                    .expect("Unable to open file_path dialog");
                            }
                        });
                    });
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
                    std::fs::write(path, &input_text).unwrap();
                }

                if let Some(Ok(Some(path))) = output_file_read_path_dialog.check() {
                    if let Ok(str) = std::fs::read_to_string(path) {
                        *output_text = str;
                    }
                }

                if let Some(Ok(Some(path))) = output_file_write_path_dialog.check() {
                    std::fs::write(path, &output_text).unwrap();
                }

                let right_key = key.iter().flatten().filter(|t| **t).count() == 4;


                ui.columns(2, |column| {
                    column[0].group(|ui| {
                ui.label("Ключ");
                {
                    let size = egui::vec2(10.0, 100.0);
                    let (response, painter) = ui.allocate_painter(size, egui::Sense::click());

                    let mut local_click = None;
                    let rect = response.rect;

                    if response.clicked() {
                        ui.input(|istate| {
                            let click_pos = istate.pointer.interact_pos().unwrap();
                            local_click = Some(click_pos - rect.min);
                        });
                    }

                    let side = rect.height().min(rect.width());
                    let cell_size = side / 4.0 - 7.0;
                    let cell_step = cell_size + 5.;

                    let a = rot_90(key.to_owned());
                    let b = rot_90(a);
                    let c = rot_90(b);
                    let d = rot_90(c);

                    if let Some(click_pos) = local_click {
                        let x = (click_pos.x / cell_step).floor() as usize;
                        let y = (click_pos.y / cell_step).floor() as usize;

                        if (0..4).contains(&x) && (0..4).contains(&y) {
                            let is_disabled =
                                (a[y][x] || b[y][x] || c[y][x] || d[y][x]) && !(key[y][x]);
                            if !is_disabled {
                                key[y][x] = !key[y][x];
                            }
                        }
                    }

                    let color = egui::Color32::from_gray(50);
                    let disabled_color = egui::Color32::from_gray(240);
                    let stroke = egui::Stroke::new(2.0, color);

                    let rect = rect.translate(vec2(1.0, 1.0));

                    for y in 0..4 {
                        for x in 0..4 {
                            let mut rect = rect;
                            rect.set_width(cell_size);
                            rect.set_height(cell_size);
                            let rect = rect
                                .translate(egui::vec2(cell_step * x as f32, cell_step * y as f32));
                            painter.rect_stroke(rect, egui::Rounding::default(), stroke);

                            let is_disabled =
                                (a[y][x] || b[y][x] || c[y][x] || d[y][x]) && !(key[y][x]);

                            if key[y][x] {
                                painter.rect_filled(rect, egui::Rounding::default(), color);
                            } else if is_disabled {
                                painter.rect_filled(
                                    rect,
                                    egui::Rounding::default(),
                                    disabled_color,
                                );
                            }
                        }
                    }
                }
                    });
                    column[0].group(|ui| {
                        ui.label("Открытый текст");
                        ui.text_edit_multiline(input_text);

                        if ui
                            .add_enabled(right_key, egui::Button::new("Получить (расшифровать)"))
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

                        if ui.button("Загрузить...").clicked() {
                            input_file_read_path_dialog
                                .open_single_file(None)
                                .expect("Unable to open file_path dialog");
                        }

                        if ui.button("Сохранить...").clicked() {
                            input_file_write_path_dialog
                                .show_save_single_file(None)
                                .expect("Unable to open file_path dialog");
                        }
                    });

                    column[1].group(|ui| {
                        ui.label("Зашифрованный текст");
                        ui.text_edit_multiline(output_text);

                        if ui
                            .add_enabled(right_key, egui::Button::new("Получить (зашифровать)"))
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

                        if ui.button("Загрузить...").clicked() {
                            output_file_read_path_dialog
                                .open_single_file(None)
                                .expect("Unable to open file_path dialog");
                        }

                        if ui.button("Сохранить...").clicked() {
                            output_file_write_path_dialog
                                .show_save_single_file(None)
                                .expect("Unable to open file_path dialog");
                        }
                    });
                });
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            EncryptTab::Vigener { .. } => "Виженер".into(),
            EncryptTab::Grille { .. } => "Вращающаяся решётка".into(),
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
        "Теория информации #1",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
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
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../AnonymousPro-Bold.ttf")),
        ); // .ttf and .otf supported
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());
        ctx.set_fonts(fonts);
        ctx.set_pixels_per_point(2.5);

        ctx.set_visuals(eframe::egui::Visuals::light());
        egui::CentralPanel::default().show(ctx, |ui| {
            self.tabs.ui(ui);
        });
    }
}
