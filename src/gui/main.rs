//! OpenFetch GUI主程序
//! 基于egui的跨平台桌面应用

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::sync::{Arc, Mutex};

mod app;
mod download_panel;
mod settings_panel;
mod tray;

use app::{AppState, get_all_platforms};
use download_panel::{render_download_panel, render_url_input};

struct OpenFetchGUI {
    state: Arc<Mutex<AppState>>,
    url_input: String,
    platform_input: String,
    selected_tab: usize,
}

impl OpenFetchGUI {
    fn new(cc: &eframe::CreationContext<'_>, state: Arc<Mutex<AppState>>) -> Self {
        // 配置egui样式
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.set_pixels_per_point(1.5);
        
        Self {
            state,
            url_input: String::new(),
            platform_input: "auto".to_string(),
            selected_tab: 0,
        }
    }
}

impl eframe::App for OpenFetchGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 侧边栏 + 主内容区
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.add_space(20.0);
                
                // Logo
                ui.vertical_centered(|ui| {
                    ui.label("⬇️");
                    ui.label(egui::RichText::new("OpenFetch").heading().color(egui::Color32::from_rgb(0, 212, 255)));
                    ui.label(egui::RichText::new("v0.8.0").weak());
                });
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);
                
                // 导航
                let tabs = ["下载", "扩展", "设置"];
                for (i, tab) in tabs.iter().enumerate() {
                    let btn = egui::Button::new(*tab)
                        .selected(self.selected_tab == i)
                        .fill(if self.selected_tab == i { 
                            egui::Color32::from_gray(50) 
                        } else { 
                            egui::Color32::TRANSPARENT 
                        });
                    
                    if ui.add(btn).clicked() {
                        self.selected_tab = i;
                    }
                }
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(20.0);
                    ui.separator();
                    let mut state = self.state.lock().unwrap();
                    ui.label(egui::RichText::new(format!("服务: {}", 
                        if state.is_server_running { "🟢 运行中" } else { "⚪ 已停止" }
                    )).weak());
                });
            });
        
        // 主内容区
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                match self.selected_tab {
                    0 => {
                        // 下载面板
                        ui.add_space(10.0);
                        
                        // URL输入
                        egui::Frame::default()
                            .fill(egui::Color32::from_gray(35))
                            .rounding(8.0)
                            .show(ui, |ui| {
                                ui.add_space(10.0);
                                render_url_input(ui, &mut self.url_input, &mut self.platform_input);
                                ui.add_space(10.0);
                            });
                        
                        ui.add_space(15.0);
                        
                        // 下载列表
                        render_download_panel(ui, &self.state);
                    }
                    1 => {
                        // 扩展面板
                        ui.heading("📦 已安装扩展");
                        ui.add_space(15.0);
                        
                        egui::Grid::new("extensions_grid")
                            .spacing([15.0, 10.0])
                            .show(ui, |ui| {
                                for (i, platform) in get_all_platforms().iter().enumerate() {
                                    ui.label(format!("{} {}", platform.icon, platform.name));
                                    
                                    let state = self.state.lock().unwrap();
                                    let enabled = true; // 从状态获取
                                    ui.checkbox(egui::widgets::Checkbox::without_text(&enabled), "");
                                    
                                    if i % 3 == 2 {
                                        ui.end_row();
                                    }
                                }
                            });
                    }
                    2 => {
                        // 设置面板
                        let mut state = self.state.lock().unwrap();
                        settings_panel::render_settings_panel(ui, &mut state.settings);
                    }
                    _ => {}
                }
            });
        
        // 定时刷新
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}

/// 启动GUI应用
pub fn run_gui(state: Arc<Mutex<AppState>>) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("OpenFetch - 开源全能下载器"),
        ..Default::default()
    };
    
    eframe::run_native(
        "OpenFetch",
        options,
        Box::new(|cc| Ok(Box::new(OpenFetchGUI::new(cc, state)))),
    ).ok();
}
