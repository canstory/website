use egui::Ui;

pub struct IndustryContent;

impl Default for IndustryContent {
    fn default() -> Self {
        Self {
        }
    }
}

impl IndustryContent {
    pub fn ui(&mut self, ui: &mut Ui) {
        egui::Window::new("AI Industy")
            .auto_sized()
            .collapsible(false)
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // iPhone 12 Pro Size
            .show(ui.ctx(), |ui| {
                ui.heading("We build a database for the Global AI Industry.");
                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(">");
                    ui.hyperlink("Canstory.fasttable.cn");
                });
            });
    }
}