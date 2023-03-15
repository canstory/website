use egui::Ui;

pub struct AboutContent {

}

impl Default for AboutContent {
    fn default() -> Self {
        Self {
        }
    }
}

impl AboutContent {
    pub fn ui(&mut self, ui: &mut Ui) {
        egui::Window::new("About Canstory")
            .anchor(egui::Align2::LEFT_BOTTOM, [100.0, -200.0])
            .auto_sized()
            .auto_sized()
            .show(ui.ctx(), |ui| {
                ui.heading("Why Canstory");
                ui.label(format!(
                    "Since 2017, the methodology in the field of artificial intelligence has changed, and artificial intelligence has begun to intervene in everyone's life and creative work. "
                ));
                ui.add_space(12.0); // ui.separator();
                ui.label("We also decided to enter the wave.");

                ui.add_space(12.0); // ui.separator();

                ui.heading("History");
                ui.label("> 2023.2, We raised initial funds to create a startup company >>> Canstory.ai");
                ui.label("> 2023.3 - Now, We are making every effort to create core product and seek angel round financing.");

                ui.add_space(12.0); // ui.separator();
                ui.heading("Links");
                links(ui);
            });
        
        egui::Window::new("Team")
            .anchor(egui::Align2::LEFT_TOP, [100.0, 100.0])
            .auto_sized()
            .show(ui.ctx(), |ui| {
                ui.heading("Xiaowen, Lone");
                ui.label(format!(
                    "Founder, a Rust engineer and entrepreneur, aged 30."
                ));
                ui.add_space(12.0); // ui.separator();
                ui.hyperlink_to("Send him an email", "mailto:lone@canstory.ai");
            });
    }
}

fn links(ui: &mut egui::Ui) {
    use egui::special_emojis::{GITHUB, TWITTER};
    ui.hyperlink_to(
        format!("{} Canstory on GitHub", GITHUB),
        "https://github.com/canstory",
    );
    ui.hyperlink_to(
        format!("{} @cantory", TWITTER),
        "https://twitter.com/canstory_ai",
    );
}