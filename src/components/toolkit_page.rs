use egui::Ui;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct EasyMarkApp {
    editor: crate::components::easy_mark::EasyMarkEditor,
}

impl eframe::App for EasyMarkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.editor.panels(ctx);
    }
}

pub struct ToolkitBox {
    show_docs: bool
}

impl Default for ToolkitBox {
    fn default() -> Self {
        Self {
            show_docs: true,
        }
    }
}

impl ToolkitBox {
    pub fn ui(&mut self, ui: &mut Ui) {
        
        ui.horizontal_wrapped(|ui| {
            egui::SidePanel::left("toolkit_panel")
                .max_width(200.0)
                .show_separator_line(true)
                .show(ui.ctx(), |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.add_space(10.0);
                        ui.label("🌟 Toolkit for Creator");
                        ui.separator();
                        if ui.button("Can Storyboard").clicked() {
                            self.show_docs = !self.show_docs;
                        };

                        ui.add_space(20.0);
                        ui.label("Apps - Powered by OpenAI");
                        ui.separator();
                        ui.hyperlink_to("Chatext Servers", "http://chatext.canstory.ai/");
                    });
               
            });
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                let mut editor_app = EasyMarkApp::default();
                if self.show_docs {
                    editor_app.editor.ui(ui);
                }
            });
        });
        
        
    }
}