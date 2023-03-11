#[cfg(feature = "glow")]
use eframe::glow;

#[cfg(target_arch = "wasm32")]
use core::any::Any;

// ----------------------------------------------------------------------------

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FractalClockApp {
    fractal_clock: crate::components::FractalClock,
}

impl eframe::App for FractalClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.fractal_clock
                    .ui(ui, Some(crate::seconds_since_midnight()));
            });
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ProductPage {
    product: crate::components::ProductList,
}

impl  eframe::App for ProductPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.product.ui(ui);
            });
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ToolkitPage {
    toolkit: crate::components::ToolkitBox,
}

impl  eframe::App for ToolkitPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.toolkit.ui(ui);
            });
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IndustryPage {
    product: crate::components::IndustryContent,
}

impl  eframe::App for IndustryPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.product.ui(ui);
            });
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AboutPage {
    about: crate::components::AboutContent,
}

impl  eframe::App for AboutPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.about.ui(ui);
            });
    }
}

/// The state that we persist (serialize).
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    clock: FractalClockApp,
    product: ProductPage,
    toolkit: ToolkitPage,
    industry: IndustryPage,
    about: AboutPage,
    selected_anchor: String,
}

pub struct WrapApp {
    state: State,
}

impl WrapApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        #[allow(unused_mut)]
        let mut app = Self {
            state: State::default(),
        };

        #[cfg(feature = "persistence")]
        if let Some(storage) = _cc.storage {
            // 从持久化存储中恢复 App 状态
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                app.state = state;
            }
        }

        // Custom fonts
        setup_custom_fonts(&cc.egui_ctx);
        // configure_text_styles(&cc.egui_ctx);

        app
    }

    // Wrapp all apps into menu
    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, &str, &mut dyn eframe::App)> {
        let vec = vec![
            (
                "🚀 Home",
                "home",
                &mut self.state.clock as &mut dyn eframe::App,
            ),
            (
                "👻 Rumeng App",
                "product",
                &mut self.state.product as &mut dyn eframe::App,
            ),
            (
                "🌟 Toolkit for Creator",
                "toolkit",
                &mut self.state.toolkit as &mut dyn eframe::App,
            ),
            (
                "🔗 AI Industry",
                "industry",
                &mut self.state.industry as &mut dyn eframe::App,
            ),
            (
                "🙌 About",
                "about",
                &mut self.state.about as &mut dyn eframe::App,
            ),
        ];

        vec.into_iter()
    }
}

impl eframe::App for WrapApp {
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // 保存 App 状态
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        // 在网页端的时候匹配 url hashtag 的参数
        if let Some(anchor) = frame.info().web_info.location.hash.strip_prefix('#') {
            self.state.selected_anchor = anchor.to_owned();
        }

        if self.state.selected_anchor.is_empty() {
            // 如果没有 hashtag，默认拿第一个菜单项
            let selected_anchor = self.apps_iter_mut().next().unwrap().0.to_owned();
            self.state.selected_anchor = selected_anchor;
        }

        #[cfg(not(target_arch = "wasm32"))]
        // 网页全屏快捷键支持
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11)) {
            frame.set_fullscreen(!frame.info().window_info.fullscreen);
        }

        egui::TopBottomPanel::bottom("wrap_app_top_bar")
            .min_height(28.0)
            .show(ctx, |ui| {
            egui::trace!(ui);

            // 渲染菜单项
            ui.horizontal_centered(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame);
            });
        });


        self.show_selected_app(ctx, frame);
    }

    #[cfg(feature = "glow")]
    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(custom3d) = &mut self.custom3d {
            custom3d.on_exit(gl);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(&mut *self)
    }
}

impl WrapApp {
    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut found_anchor = false;
        let selected_anchor = self.state.selected_anchor.clone();
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory(|mem| mem.everything_is_visible()) {
                app.update(ctx, frame);
                found_anchor = true;
            }
        }
        if !found_anchor {
            self.state.selected_anchor = "home".into();
        }
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::widgets::global_dark_light_mode_switch(ui);

        let mut selected_anchor = self.state.selected_anchor.clone();
        for (name, anchor, _app) in self.apps_iter_mut() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor.to_owned();
                if frame.is_web() {
                    ui.output_mut(|o| o.open_url(format!("#{}", anchor)));
                }
            }
        }
        self.state.selected_anchor = selected_anchor;

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::warn_if_debug_build(ui);
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert("CascadiaCode".to_owned(), egui::FontData::from_static(
        include_bytes!("../assets/fonts/CascadiaCode.ttf")
    ));

    fonts.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "CascadiaCode".to_owned());

    ctx.set_fonts(fonts);
}