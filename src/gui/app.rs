//! OpenFetch GUI 应用

use eframe::egui;

pub struct OpenFetchApp {
    pub url: String,
    pub platform: String,
    pub status: String,
}

impl Default for OpenFetchApp {
    fn default() -> Self {
        Self {
            url: String::new(),
            platform: "auto".to_string(),
            status: "就绪".to_string(),
        }
    }
}

impl eframe::App for OpenFetchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📥 OpenFetch - 开源全能下载器");
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label("视频链接:");
                ui.text_edit_singleline(&mut self.url);
            });
            
            ui.horizontal(|ui| {
                ui.label("平台:");
                egui::ComboBox::from_id_salt("platform")
                    .selected_text(&self.platform)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.platform, "auto".to_string(), "自动检测");
                        ui.selectable_value(&mut self.platform, "bilibili".to_string(), "哔哩哔哩");
                        ui.selectable_value(&mut self.platform, "youtube".to_string(), "YouTube");
                        ui.selectable_value(&mut self.platform, "douyin".to_string(), "抖音");
                        ui.selectable_value(&mut self.platform, "weibo".to_string(), "微博视频");
                    });
            });
            
            ui.add_space(10.0);
            
            if ui.button("⬇️ 开始下载").clicked() {
                if !self.url.is_empty() {
                    self.status = format!("正在下载: {} ...", self.url);
                } else {
                    self.status = "请输入视频链接".to_string();
                }
            }
            
            ui.add_space(20.0);
            ui.label(format!("状态: {}", self.status));
            
            ui.separator();
            ui.label("支持的平台: 哔哩哔哩 | YouTube | 抖音 | 微博视频 | 西瓜视频");
        });
    }
}

/// 启动 GUI
pub fn run_gui() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "OpenFetch",
        options,
        Box::new(|_cc| Ok(Box::new(OpenFetchApp::default()))),
    ).expect("Failed to run GUI");
}
