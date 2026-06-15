//! 下载面板组件

use crate::gui::app::{AppState, DownloadStatus, DownloadTask, get_all_platforms};

/// 渲染下载面板
pub fn render_download_panel(ui: &mut egui::Ui, state: &mut Arc<Mutex<AppState>>) {
    ui.heading("⬇️ 下载管理器");
    ui.add_space(10.0);
    
    let mut state_guard = state.lock().unwrap();
    
    // 顶部工具栏
    ui.horizontal(|ui| {
        if ui.button("➕ 新建下载").clicked() {
            // 触发新建下载弹窗
        }
        
        if ui.button("📂 打开目录").clicked() {
            if let Some(path) = directories::UserDirs::new()
                .and_then(|d| d.download_dir().map(|p| p.to_path_buf()))
            {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer")
                    .arg(&path)
                    .spawn()
                    .ok();
                #[cfg(target_os = "macos")]
                std::process::Command::new("open")
                    .arg(&path)
                    .spawn()
                    .ok();
                #[cfg(target_os = "linux")]
                std::process::Command::new("xdg-open")
                    .arg(&path)
                    .spawn()
                    .ok();
            }
        }
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(format!("并发: {}", state_guard.settings.max_concurrent));
        });
    });
    
    ui.separator();
    
    // 下载列表
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if state_guard.tasks.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.label("📭 暂无下载任务");
                    ui.labelegui::RichText::new("粘贴URL开始下载").weak();
                });
            } else {
                for (_, task) in state_guard.tasks.iter_mut() {
                    render_task_card(ui, task);
                }
            }
        });
}

/// 渲染单个任务卡片
fn render_task_card(ui: &mut egui::Ui, task: &mut DownloadTask) {
    ui.add_space(5.0);
    
    egui::Frame::default()
        .fill(egui::Color32::from_gray(30))
        .rounding(8.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // 平台图标
                let platforms = get_all_platforms();
                let icon = platforms.iter()
                    .find(|p| p.id == task.platform)
                    .map(|p| p.icon)
                    .unwrap_or("📁");
                
                ui.label(icon);
                
                // 标题
                ui.vertical(|ui| {
                    ui.label(if task.title.is_empty() { 
                        &task.url 
                    } else { 
                        &task.title 
                    });
                    
                    // 进度条
                    match &task.status {
                        DownloadStatus::Downloading { progress, speed } => {
                            ui.add(egui::ProgressBar::new(*progress / 100.0));
                            ui.label(egui::RichText::new(format!("{} - {}%", speed, *progress as i32)).weak());
                        }
                        DownloadStatus::Completed { path } => {
                            ui.label(egui::RichText::new("✅ 完成").color(egui::Color32::GREEN));
                        }
                        DownloadStatus::Failed { error } => {
                            ui.label(egui::RichText::new(format!("❌ {}", error)).color(egui::Color32::RED));
                        }
                        DownloadStatus::Pending => {
                            ui.label(egui::RichText::new("⏳ 等待中").weak());
                        }
                        DownloadStatus::Cancelled => {
                            ui.label(egui::RichText::new("🚫 已取消").weak());
                        }
                    }
                });
                
                // 操作按钮
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if matches!(task.status, DownloadStatus::Downloading { .. }) {
                        if ui.button("⏹").clicked() {
                            task.status = DownloadStatus::Cancelled;
                        }
                    } else {
                        if ui.button("🗑").clicked() {
                            // 删除任务
                        }
                    }
                });
            });
        });
}

/// 渲染URL输入框
pub fn render_url_input(ui: &mut egui::Ui, url: &mut String, platform: &mut String) {
    ui.horizontal(|ui| {
        // 平台选择
        egui::ComboBox::from_id_salt("platform_select")
            .selected_text(*platform)
            .show_ui(ui, |ui| {
                for p in get_all_platforms() {
                    ui.selectable_value(platform, p.id, format!("{} {}", p.icon, p.name));
                }
            });
        
        // URL输入
        ui.text_edit_singleline(url)
            .placeholder_text("粘贴视频URL...");
        
        // 下载按钮
        if ui.button("⬇️ 下载").clicked() && !url.is_empty() {
            // 添加下载任务
        }
    });
}
