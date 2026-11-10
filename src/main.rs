use eframe::egui;

struct LightFox {
    address: String,
    status: String,
    tabs: usize,
}

impl Default for LightFox {
    fn default() -> Self {
        Self {
            address: "https://example.com".to_owned(),
            status: "Preview build — Servo web rendering is not integrated yet.".to_owned(),
            tabs: 1,
        }
    }
}

impl eframe::App for LightFox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("◈ LightFox");
                ui.separator();
                ui.add_enabled(false, egui::Button::new("‹"));
                ui.add_enabled(false, egui::Button::new("›"));
                ui.add_enabled(false, egui::Button::new("⟳"));
                let address = ui.add(
                    egui::TextEdit::singleline(&mut self.address)
                        .hint_text("Search or enter a website address")
                        .desired_width(ui.available_width() - 90.0),
                );
                if address.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.status = "Address accepted. Page rendering requires the Servo backend.".into();
                }
                if ui.button("Go").clicked() {
                    self.status = "Navigation is waiting for Servo integration.".into();
                }
                if ui.button("+").clicked() {
                    self.tabs += 1;
                    self.status = format!("Prototype tab {} created.", self.tabs);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(90.0);
                ui.heading(egui::RichText::new("◈ LightFox").size(38.0).strong());
                ui.label("Fast by design. Private by default.");
                ui.add_space(28.0);
                ui.label("The lightweight browser project built with Rust.");
                ui.add_space(12.0);
                ui.label("Servo is the planned web engine. It is not connected in this prototype.");
                ui.add_space(30.0);
                ui.group(|ui| {
                    ui.set_min_width(420.0);
                    ui.label("PROJECT STATUS");
                    ui.colored_label(egui::Color32::from_rgb(230, 185, 112), "EARLY UI PROTOTYPE");
                    ui.label("Tabs and address bar are visual scaffolding.");
                    ui.label("Web pages, JavaScript, downloads and bookmarks are not available yet.");
                });
                ui.add_space(25.0);
                ui.label(&self.status);
            });
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{} tab(s)", self.tabs));
                ui.separator();
                ui.label("Servo: not integrated");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("LightFox · experimental");
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1120.0, 720.0])
            .with_min_inner_size([720.0, 480.0])
            .with_title("LightFox"),
        ..Default::default()
    };
    eframe::run_native(
        "LightFox",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::<LightFox>::default())
        }),
    )
}
