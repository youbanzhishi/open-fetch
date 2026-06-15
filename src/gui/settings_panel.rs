//! 设置面板组件

use crate::gui::app::AppSettings;

/// 渲染设置面板
pub fn render_settings_panel(ui: &mut egui::Ui, settings: &mut AppSettings) {
    ui.heading("⚙️ 设置");
    ui.add_space(15.0);
    
    egui::Grid::new("settings_grid")
        .num_columns(2)
        .spacing([20.0, 10.0])
        .show(ui, |ui| {
            // 下载目录
            ui.label("下载目录:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut settings.download_path);
                if ui.button("📂").clicked() {
                    // 打开文件选择器
                }
            });
            ui.end_row();
            
            // 最大并发数
            ui.label("最大并发:");
            ui.add(egui::Slider::new(&mut settings.max_concurrent, 1..=10).text(format!("{}个", settings.max_concurrent)));
            ui.end_row();
            
            // 自动启动服务
            ui.label("自动启动服务:");
            ui.checkbox(&mut settings.auto_server, "启动时自动开启HTTP服务");
            ui.end_row();
            
            // 通知
            ui.label("通知:");
            ui.checkbox(&mut settings.notifications, "下载完成时发送通知");
            ui.end_row();
            
            // 主题
            ui.label("主题:");
            ui.horizontal(|ui| {
                ui.radio_value(&mut settings.theme, "dark".to_string(), "🌙 深色");
                ui.radio_value(&mut settings.theme, "light".to_string(), "☀️ 浅色");
            });
            ui.end_row();
        });
    
    ui.add_space(20.0);
    ui.separator();
    ui.add_space(10.0);
    
    // 服务器设置
    ui.heading("🌐 HTTP服务");
    ui.add_space(10.0);
    
    ui.horizontal(|ui| {
        ui.label("端口:");
        ui.add(egui::DragValue::new(&mut settings.max_concurrent).clamp_range(1024..=65535));
        
        if ui.button("▶️ 启动服务").clicked() {
            // 启动HTTP服务
        }
        
        if ui.button("⏹ 停止服务").clicked() {
            // 停止HTTP服务
        }
    });
    
    ui.add_space(10.0);
    ui.label(egui::RichText::new("服务地址: http://localhost:8080").weak());
    
    ui.add_space(20.0);
    ui.separator();
    ui.add_space(10.0);
    
    // 关于
    ui.heading("ℹ️ 关于");
    ui.add_space(10.0);
    ui.label("OpenFetch v0.8.0");
    ui.label(egui::RichText::new("开源全能下载器").weak());
    ui.label(egui::RichText::new("https://github.com/youbanzhishi/open-fetch").weak());
}
