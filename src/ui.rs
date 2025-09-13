use std::path::PathBuf;
use eframe::egui;
use egui::RichText;
use crate::themes::CodeTheme;

pub(crate) fn toolbar(ui: &mut egui::Ui, app: &mut crate::app::FileViewerApp, ctx: &egui::Context, file_to_load: &mut Option<PathBuf>) {
    
    use rfd::FileDialog;

    // Modern app branding
    ui.horizontal(|ui| {
        ui.add_space(4.0);
        ui.label(RichText::new("📁").size(20.0));
        ui.add_space(8.0);
        ui.label(RichText::new("Gemini File Viewer").heading().strong());
        ui.add_space(8.0);
        ui.label(RichText::new("Pre-beta").weak().small());
    });
    
    ui.add_space(12.0);
    ui.separator();
    ui.add_space(8.0);

    // All toolbar buttons in single horizontal layout for perfect alignment
    ui.horizontal(|ui| {
        // Open File button
        let mut open_button = egui::Button::new(RichText::new("📂 Open File").strong());
        open_button = open_button.fill(egui::Color32::from_rgb(34, 197, 94)); // Green
        if ui.add(open_button).clicked()
            && let Some(path) = FileDialog::new()
                .add_filter("All Supported", &["txt","rs","py","toml","md","json","js","html","css","png","jpg","jpeg","gif","bmp","webp"])
                .add_filter("Images", &["png","jpg","jpeg","gif","bmp","webp"])
                .add_filter("Text/Source", &["txt","rs","py","toml","md","json","js","html","css"])
                .pick_file()
        {
            *file_to_load = Some(path);
        }

        // Recent Files button
        ui.menu_button(RichText::new("📋 Recent Files").strong(), |ui| {
            ui.set_min_width(480.0);
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            if app.recent_files.is_empty() {
                ui.label(RichText::new("No recent files").weak());
            }
            for file in app.recent_files.clone().into_iter().rev() {
                let display = file.to_string_lossy();
                if ui
                    .button(egui::RichText::new(display.clone()).monospace())
                    .on_hover_text(display)
                    .clicked()
                {
                    *file_to_load = Some(file);
                    ui.close_menu();
                }
            }
            ui.separator();
            let mut clear_button = egui::Button::new(RichText::new("🗑️ Clear Recent Files"));
            clear_button = clear_button.fill(egui::Color32::from_rgb(239, 68, 68)); // Red
            if ui.add(clear_button).clicked() {
                app.recent_files.clear();
                ui.close_menu();
            }
        });

        // Themes button
        ui.menu_button(RichText::new("🎨 Themes").strong(), |ui| {
            ui.set_min_width(300.0);
            
            let prev_theme = app.code_theme;
            
            ui.label(RichText::new("🎨 Code Themes").strong());
            ui.add_space(8.0);
            
            for theme in CodeTheme::all() {
                let is_selected = app.code_theme == *theme;
                let mut button_text = RichText::new(theme.name());
                if is_selected {
                    button_text = button_text.strong();
                }
                
                if ui.selectable_label(is_selected, button_text).clicked() {
                    app.code_theme = *theme;
                    ui.close_menu();
                }
            }
            
            if app.code_theme != prev_theme {
                crate::settings::save_settings_to_disk(app);
            }
        });

        // Settings button
        ui.menu_button(RichText::new("⚙️ Settings").strong(), |ui| {
            ui.set_min_width(200.0);
            
            let prev_dark = app.dark_mode;
            let prev_lines = app.show_line_numbers;
            
            ui.label(RichText::new("🎨 Display Settings").strong());
            ui.add_space(8.0);
            
            ui.checkbox(&mut app.dark_mode, RichText::new("🌙 Dark Mode").strong());
            ui.checkbox(&mut app.show_line_numbers, RichText::new("📊 Line Numbers").strong());
            
            if app.dark_mode != prev_dark {
                app.apply_theme(ctx);
            }
            if app.dark_mode != prev_dark || app.show_line_numbers != prev_lines {
                crate::settings::save_settings_to_disk(app);
            }
            
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);
            
            ui.label(RichText::new("ℹ️ About").strong());
            ui.add_space(8.0);
            ui.label(RichText::new("Gemini File Viewer").weak());
            ui.label(RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION"))).weak());
        });

        // Clear button
        let mut clear_button = egui::Button::new(RichText::new("🗑️ Clear").strong());
        clear_button = clear_button.fill(egui::Color32::from_rgb(239, 68, 68)); // Red
        if ui.add(clear_button).clicked() {
            app.content = None;
            app.current_path = None;
            app.error_message = None;
        }
    });

    // Image controls (zoom and fit)
    if matches!(app.content, Some(crate::app::Content::Image(_))) {
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);
        
        let prev_fit = app.image_fit;
        ui.horizontal(|ui| {
            ui.checkbox(&mut app.image_fit, RichText::new("📐 Fit to Window").strong());
            if app.image_fit != prev_fit { crate::settings::save_settings_to_disk(app); }
        });
        
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            let mut zoom_out_button = egui::Button::new(RichText::new("🔍-").strong());
            zoom_out_button = zoom_out_button.fill(egui::Color32::from_rgb(245, 158, 11)); // Orange
            if ui.add(zoom_out_button).clicked() { 
                app.image_fit = false; 
                app.image_zoom = (app.image_zoom / 1.10).clamp(0.1, 6.0); 
            }
            let mut zoom_in_button = egui::Button::new(RichText::new("🔍+").strong());
            zoom_in_button = zoom_in_button.fill(egui::Color32::from_rgb(245, 158, 11)); // Orange
            if ui.add(zoom_in_button).clicked() { 
                app.image_fit = false; 
                app.image_zoom = (app.image_zoom * 1.10).clamp(0.1, 6.0); 
            }
            let mut reset_button = egui::Button::new(RichText::new("100%").strong());
            reset_button = reset_button.fill(egui::Color32::from_rgb(34, 197, 94)); // Green
            if ui.add(reset_button).clicked() { 
                app.image_fit = false; 
                app.image_zoom = 1.0; 
            }
        });
    }
}

pub(crate) fn search_bar(ui: &mut egui::Ui, app: &mut crate::app::FileViewerApp, file_to_load: &mut Option<PathBuf>) {
    // Modern search bar with better styling
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            // Search input (only for text files)
            if matches!(app.content, Some(crate::app::Content::Text(_))) {
                ui.label(RichText::new("🔍 Find:").strong());
                ui.add_space(8.0);
                
                let prev = app.search_query.clone();
                let resp = ui.text_edit_singleline(&mut app.search_query);
                if app.search_active {
                    resp.request_focus();
                    app.search_active = false;
                }
                
                // Enter / Shift+Enter navigate matches
                let (enter, shift) = ui.input(|i| (i.key_pressed(egui::Key::Enter), i.modifiers.shift));
                if enter && app.search_count > 0 {
                    if shift {
                        if app.search_current == 0 { app.search_current = app.search_count.saturating_sub(1); } else { app.search_current -= 1; }
                    } else {
                        app.search_current = (app.search_current + 1) % app.search_count;
                    }
                }

                if resp.changed() || (prev.is_empty() && !app.search_query.is_empty()) {
                    app.search_count = 0;
                    app.search_current = 0;
                    if let Some(crate::app::Content::Text(ref text)) = app.content {
                        if !app.search_query.is_empty() && text.len() <= crate::app::HIGHLIGHT_CHAR_THRESHOLD {
                            app.search_count = crate::search::recompute_count(&app.search_query, text);
                        }
                    }
                }
            }
            
            // File navigation buttons (Prev/Next) - compact with just arrows
            if let Some(cur) = app.current_path.clone() {
                ui.add_space(12.0);
                match app.content {
                    Some(crate::app::Content::Image(_)) => {
                        if ui.small_button(RichText::new("⬅️").size(10.0)).on_hover_text("Previous file").clicked() {
                            if let Some(prev) = crate::io::neighbor_image(&cur, false) {
                                *file_to_load = Some(prev);
                            }
                        }
                        if ui.small_button(RichText::new("➡️").size(10.0)).on_hover_text("Next file").clicked() {
                            if let Some(next) = crate::io::neighbor_image(&cur, true) {
                                *file_to_load = Some(next);
                            }
                        }
                    }
                    Some(crate::app::Content::Text(_)) => {
                        if ui.small_button(RichText::new("⬅️").size(10.0)).on_hover_text("Previous file").clicked() {
                            if let Some(prev) = crate::io::neighbor_text(&cur, false) {
                                *file_to_load = Some(prev);
                            }
                        }
                        if ui.small_button(RichText::new("➡️").size(10.0)).on_hover_text("Next file").clicked() {
                            if let Some(next) = crate::io::neighbor_text(&cur, true) {
                                *file_to_load = Some(next);
                            }
                        }
                    }
                    _ => {}
                }
            }
            
            // Search result navigation (only for text files with search query)
            if matches!(app.content, Some(crate::app::Content::Text(_))) && !app.search_query.is_empty() {
                ui.add_space(12.0);
                ui.label(RichText::new(format!("{} match(es)", app.search_count)).weak());
                ui.add_space(8.0);
                
                if ui.small_button(RichText::new("⬅️").size(10.0)).on_hover_text("Previous match").clicked() && app.search_count > 0 {
                    if app.search_current == 0 { app.search_current = app.search_count.saturating_sub(1); } else { app.search_current -= 1; }
                }
                if ui.small_button(RichText::new("➡️").size(10.0)).on_hover_text("Next match").clicked() && app.search_count > 0 {
                    app.search_current = (app.search_current + 1) % app.search_count;
                }
                
                if app.search_count > 0 {
                    ui.add_space(8.0);
                    ui.label(RichText::new(format!("{}/{}", app.search_current + 1, app.search_count)).strong());
                }
            }
        });
    });
}

pub(crate) fn status_bar(ui: &mut egui::Ui, app: &mut crate::app::FileViewerApp) {
    use std::fs;
    
    // Modern status bar with better visual hierarchy
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            if let Some(path) = &app.current_path {
                ui.label(RichText::new("📄").size(16.0));
                ui.add_space(8.0);
                ui.monospace(RichText::new(path.to_string_lossy()).strong());
                
                if let Ok(metadata) = fs::metadata(path) {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("({:.1} KB)", metadata.len() as f64 / 1024.0)).weak());
                    });
                }
                
                ui.add_space(12.0);
                let mut copy_button = egui::Button::new(RichText::new("📋 Copy Path").strong());
                copy_button = copy_button.fill(egui::Color32::from_rgb(34, 197, 94)); // Green
                if ui.add(copy_button).on_hover_text("Copy path to clipboard").clicked() {
                    ui.ctx().copy_text(path.to_string_lossy().into());
                }
                let mut folder_button = egui::Button::new(RichText::new("📁 Open Folder").strong());
                folder_button = folder_button.fill(egui::Color32::from_rgb(59, 130, 246)); // Blue
                if ui.add(folder_button).clicked() {
                    #[cfg(target_os = "windows")]
                    { let _ = std::process::Command::new("explorer").arg(path).spawn(); }
                    #[cfg(target_os = "macos")]
                    { let _ = std::process::Command::new("open").arg("-R").arg(path).spawn(); }
                    #[cfg(all(unix, not(target_os = "macos")))]
                    { if let Some(parent) = path.parent() { let _ = std::process::Command::new("xdg-open").arg(parent).spawn(); } }
                }
            } else {
                ui.label(RichText::new("📄 No file selected").weak());
            }
        });
    });
}

pub(crate) fn status_extra(ui: &mut egui::Ui, app: &mut crate::app::FileViewerApp) {
    // Modern status extra with icons and better formatting
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            match &app.content {
                Some(crate::app::Content::Image(texture)) => {
                    let size = texture.size();
                    ui.colored_label(egui::Color32::from_rgb(34, 197, 94), RichText::new("🖼️").size(16.0)); // Green
                    ui.add_space(8.0);
                    ui.label(RichText::new(format!("{}x{} px", size[0], size[1])).strong());
                    
                    let eff = if app.image_fit { None } else { Some(app.image_zoom) };
                    if let Some(z) = eff { 
                        ui.add_space(12.0);
                        ui.colored_label(egui::Color32::from_rgb(245, 158, 11), RichText::new(format!("🔍 {:.0}%", z * 100.0))); // Orange
                    }
                    
                    let est = (size[0] as usize).saturating_mul(size[1] as usize).saturating_mul(4);
                    ui.add_space(12.0);
                    ui.colored_label(egui::Color32::from_rgb(59, 130, 246), RichText::new(format!("💾 ~{:.1} MB", est as f64 / (1024.0 * 1024.0)))); // Blue
                    
                    if app.image_fit { 
                        ui.add_space(12.0);
                        ui.colored_label(egui::Color32::from_rgb(168, 85, 247), RichText::new("📐 Fit: on")); // Purple
                    }
                }
                Some(crate::app::Content::Text(_)) => {
                    ui.colored_label(egui::Color32::from_rgb(34, 197, 94), RichText::new("📝").size(16.0)); // Green
                    ui.add_space(8.0);
                    ui.label(RichText::new(format!("{} lines", app.text_line_count)).strong());
                    ui.add_space(12.0);
                    ui.colored_label(egui::Color32::from_rgb(245, 158, 11), RichText::new(format!("🔍 {:.0}%", app.text_zoom * 100.0))); // Orange
                    
                    if app.text_is_big { 
                        ui.add_space(12.0);
                        ui.colored_label(egui::Color32::from_rgb(239, 68, 68), RichText::new("⚠️ Large file: reduced features")); // Red
                    }
                    if app.text_is_lossy { 
                        ui.add_space(12.0);
                        ui.colored_label(egui::Color32::from_rgb(239, 68, 68), RichText::new("⚠️ UTF-8 (lossy)")); // Red
                    }
                }
                _ => {}
            }
        });
    });
}

