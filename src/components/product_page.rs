use egui::Ui;

pub struct ProductList;

impl Default for ProductList {
    fn default() -> Self {
        Self {
        }
    }
}

impl ProductList {
    pub fn ui(&mut self, ui: &mut Ui) {

        egui::Window::new("Ru Meng")
            .collapsible(false)
            .anchor(egui::Align2::LEFT_CENTER, [100.0, 0.0])
            .fixed_size([390.0, 844.0]) // iPhone 12 Pro Size
            .show(ui.ctx(), |ui| {
                ui.set_min_height(720.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(300.0);
                    ui.heading("Convet Dream to Story (image + text)");
                    ui.heading("with your dream architect.");
                    ui.label("[ AI Powered ]");
                    ui.add_space(44.0);
                    ui.button("Coming Soon...")
                });

            });
        
        egui::Window::new("View stories from Creative People")
            .collapsible(false)
            .anchor(egui::Align2::RIGHT_CENTER, [-100.0, 0.0])
            .fixed_size([390.0, 844.0]) // iPhone 12 Pro Size
            .show(ui.ctx(), |ui| {
                ui.set_min_height(720.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(300.0);
                    ui.heading("Creating and will be launched soon");
                });
            });
    }
}