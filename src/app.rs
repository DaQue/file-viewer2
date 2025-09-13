use eframe::egui;
use crate::highlight;
use crate::search;
use crate::themes::CodeTheme;
use egui::{text::LayoutJob, RichText, TextureHandle};
use std::fs;
use rfd::FileDialog;
use std::path::PathBuf;

const MAX_FILE_SIZE_BYTES: u64 = 10_000_000; // 10MB
const MAX_RECENT_FILES: usize = 10;
const BIG_TEXT_CHAR_THRESHOLD: usize = 500_000; // Disable heavy features beyond this
pub(crate) const HIGHLIGHT_CHAR_THRESHOLD: usize = 200_000; // Disable syntax/mark highlights beyond this

pub enum Content {
    Text(String),
    Image(TextureHandle),
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct FileViewerApp {
    #[serde(skip)]
    pub(crate) content: Option<Content>,
    #[serde(skip)]
    pub(crate) current_path: Option<PathBuf>,
    #[serde(skip)]
    pub(crate) error_message: Option<String>,
    pub(crate) dark_mode: bool,
    pub(crate) code_theme: CodeTheme,
    pub(crate) recent_files: Vec<PathBuf>,
    pub(crate) show_line_numbers: bool,
    pub(crate) word_wrap: bool,
    pub(crate) text_zoom: f32,
    pub(crate) image_zoom: f32,
    #[serde(skip)]
    pub(crate) show_about: bool,
    pub(crate) image_fit: bool,
    // Derived/runtime-only state for text rendering
    #[serde(skip)]
    pub(crate) text_is_big: bool,
    #[serde(skip)]
    pub(crate) text_line_count: usize,
    #[serde(skip)]
    pub(crate) text_is_lossy: bool,
    // Simple find state
    #[serde(skip)]
    pub(crate) search_query: String,
    #[serde(skip)]
    pub(crate) search_active: bool,
    #[serde(skip)]
    pub(crate) search_count: usize,
    #[serde(skip)]
    pub(crate) search_current: usize,
}

impl FileViewerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage
            && let Some(s) = storage.get_string(eframe::APP_KEY)
            && let Ok(mut app) = serde_json::from_str::<FileViewerApp>(&s)
        {
            // ensure runtime-only fields are initialized
            app.text_is_big = false;
            app.text_line_count = 0;
            app.text_is_lossy = false;
            app.search_query = String::new();
            app.search_active = false;
            app.search_count = 0;
            return app;
        }
        if let Some(mut app) = crate::settings::load_settings_from_disk() {
            app.text_is_big = false;
            app.text_line_count = 0;
            app.text_is_lossy = false;
            app.search_query = String::new();
            app.search_active = false;
            app.search_count = 0;
            return app;
        }
        Default::default()
    }

    pub(crate) fn apply_theme(&self, ctx: &egui::Context) {
        let mut visuals = if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };

        // Vibrant color scheme
        if self.dark_mode {
            // Dark theme - vibrant dark palette
            visuals.window_fill = egui::Color32::from_rgb(15, 15, 20);        // Deep dark background
            visuals.panel_fill = egui::Color32::from_rgb(22, 22, 28);         // Slightly lighter panels
            visuals.faint_bg_color = egui::Color32::from_rgb(32, 32, 40);     // Subtle backgrounds
            visuals.extreme_bg_color = egui::Color32::from_rgb(42, 42, 50);   // Hover states
            
            // Vibrant accent colors
            visuals.selection.bg_fill = egui::Color32::from_rgb(59, 130, 246); // Bright blue
            visuals.hyperlink_color = egui::Color32::from_rgb(99, 102, 241);  // Purple-blue
            
            // Interactive elements
            visuals.button_frame = true;
            visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(55, 65, 81));
        } else {
            // Light theme - vibrant light palette
            visuals.window_fill = egui::Color32::from_rgb(255, 255, 255);     // Pure white
            visuals.panel_fill = egui::Color32::from_rgb(248, 250, 252);      // Very light gray
            visuals.faint_bg_color = egui::Color32::from_rgb(241, 245, 249);  // Light backgrounds
            visuals.extreme_bg_color = egui::Color32::from_rgb(226, 232, 240); // Hover states
            
            // Vibrant accent colors
            visuals.selection.bg_fill = egui::Color32::from_rgb(59, 130, 246); // Bright blue
            visuals.hyperlink_color = egui::Color32::from_rgb(99, 102, 241);  // Purple-blue
            
            // Interactive elements
            visuals.button_frame = true;
            visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(226, 232, 240));
        }

        // Modern spacing and styling
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(12.0, 8.0);           // More generous spacing
        style.spacing.button_padding = egui::vec2(16.0, 10.0);        // Larger button padding
        style.spacing.menu_margin = egui::Margin::same(8);            // Menu margins
        style.spacing.window_margin = egui::Margin::same(16);         // Window margins
        
        // Modern typography
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(24.0, egui::FontFamily::Proportional)
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(14.0, egui::FontFamily::Proportional)
        );
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            egui::FontId::new(13.0, egui::FontFamily::Monospace)
        );
        
        // Modern styling
        style.visuals.button_frame = true;
        style.visuals.collapsing_header_frame = true;
        
        style.visuals = visuals;
        ctx.set_style(style);
    }

    // io helpers moved to crate::io

    pub fn load_file(&mut self, path: PathBuf, ctx: &egui::Context) {
        self.content = None;
        self.error_message = None;
        self.current_path = None;

        if let Ok(metadata) = fs::metadata(&path)
            && metadata.len() > MAX_FILE_SIZE_BYTES
        {
            self.error_message = Some(format!(
                "File is too large (> {:.1}MB)",
                MAX_FILE_SIZE_BYTES as f64 / 1_000_000.0
            ));
            return;
        }

        let loaded = if crate::io::is_supported_image(&path) {
            match crate::io::load_image(&path) {
                Ok(color_image) => {
                    let texture = ctx.load_texture(
                        path.to_string_lossy(),
                        color_image,
                        egui::TextureOptions::LINEAR,
                    );
                    Ok(Content::Image(texture))
                }
                Err(e) => Err(e),
            }
        } else {
            match crate::io::load_text(&path) {
                Ok((text, lossy, lines)) => {
                    self.text_is_big = text.len() >= BIG_TEXT_CHAR_THRESHOLD || lines >= 50_000;
                    self.text_line_count = lines;
                    self.text_is_lossy = lossy;
                    Ok(Content::Text(text))
                }
                Err(e) => Err(e),
            }
        };

        match loaded {
            Ok(content) => {
                self.content = Some(content);
                self.current_path = Some(path.clone());
                // Deduplicate and push to recents
                self.recent_files.retain(|p| p != &path);
                self.recent_files.push(path);
                if self.recent_files.len() > MAX_RECENT_FILES {
                    let overflow = self.recent_files.len() - MAX_RECENT_FILES;
                    self.recent_files.drain(0..overflow);
                }
                // Persist updated recents immediately
                crate::settings::save_settings_to_disk(self);
            }
            Err(e) => self.error_message = Some(e),
        }
    }

    // settings helpers moved to crate::settings
}

impl Default for FileViewerApp {
    fn default() -> Self {
        Self {
            content: None,
            current_path: None,
            error_message: None,
            dark_mode: true,
            code_theme: CodeTheme::default(),
            recent_files: Vec::new(),
            show_line_numbers: true,
            word_wrap: true,
            text_zoom: 1.0,
            image_zoom: 1.0,
            show_about: false,
            image_fit: false,
            text_is_big: false,
            text_line_count: 0,
            text_is_lossy: false,
            search_query: String::new(),
            search_active: false,
            search_count: 0,
            search_current: 0,
        }
    }
}

impl eframe::App for FileViewerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(s) = serde_json::to_string(self) {
            storage.set_string(eframe::APP_KEY, s);
        }
        crate::settings::save_settings_to_disk(self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply visuals each frame
        self.apply_theme(ctx);

        let mut file_to_load: Option<PathBuf> = None;

        // Keyboard shortcuts
        let mut toggle_dark = false;
        ctx.input(|i| {
            if i.modifiers.command && i.key_pressed(egui::Key::O) {
                if let Some(path) = FileDialog::new()
                    .add_filter("All Supported", &["txt","rs","py","toml","md","json","js","html","css","png","jpg","jpeg","gif","bmp","webp"])
                    .add_filter("Images", &["png","jpg","jpeg","gif","bmp","webp"])
                    .add_filter("Text/Source", &["txt","rs","py","toml","md","json","js","html","css"])
                    .pick_file()
                {
                    file_to_load = Some(path);
                }
            }
            if i.modifiers.command && i.key_pressed(egui::Key::D) {
                toggle_dark = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::F) {
                self.search_active = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::L) {
                self.show_line_numbers = !self.show_line_numbers;
                crate::settings::save_settings_to_disk(self);
            }
            if i.modifiers.command && i.key_pressed(egui::Key::W) {
                self.word_wrap = !self.word_wrap;
                crate::settings::save_settings_to_disk(self);
            }

            // Ctrl + Mouse wheel zoom for content
            if i.modifiers.command && i.raw_scroll_delta.y != 0.0 {
                let dir = i.raw_scroll_delta.y.signum();
                match &self.content {
                    Some(Content::Text(_)) => {
                        let factor = if dir > 0.0 { 1.05 } else { 1.0 / 1.05 };
                        self.text_zoom = (self.text_zoom * factor).clamp(0.6, 3.0);
                    }
                    Some(Content::Image(_)) => {
                        self.image_fit = false;
                        let factor = if dir > 0.0 { 1.10 } else { 1.0 / 1.10 };
                        self.image_zoom = (self.image_zoom * factor).clamp(0.1, 6.0);
                    }
                    _ => {}
                }
            }

            // Reset and keyboard zoom shortcuts
            if i.modifiers.command && i.key_pressed(egui::Key::Num0) {
                match &self.content {
                    Some(Content::Text(_)) => self.text_zoom = 1.0,
                    Some(Content::Image(_)) => { self.image_fit = false; self.image_zoom = 1.0; },
                    _ => {}
                }
            }
            if i.modifiers.command && i.key_pressed(egui::Key::Equals) {
                match &self.content {
                    Some(Content::Text(_)) => self.text_zoom = (self.text_zoom * 1.05).clamp(0.6, 3.0),
                    Some(Content::Image(_)) => { self.image_fit = false; self.image_zoom = (self.image_zoom * 1.10).clamp(0.1, 6.0); },
                    _ => {}
                }
            }
            if i.modifiers.command && i.key_pressed(egui::Key::Minus) {
                match &self.content {
                    Some(Content::Text(_)) => self.text_zoom = (self.text_zoom / 1.05).clamp(0.6, 3.0),
                    Some(Content::Image(_)) => { self.image_fit = false; self.image_zoom = (self.image_zoom / 1.10).clamp(0.1, 6.0); },
                    _ => {}
                }
            }

            // Navigation with arrow keys for current content type
            if i.key_pressed(egui::Key::ArrowRight) {
                if let Some(cur) = self.current_path.clone() {
                    match self.content {
                        Some(Content::Image(_)) => {
                            if let Some(next) = crate::io::neighbor_image(&cur, true) { file_to_load = Some(next); }
                        }
                        Some(Content::Text(_)) => {
                            if let Some(next) = crate::io::neighbor_text(&cur, true) { file_to_load = Some(next); }
                        }
                        _ => {}
                    }
                }
            }
            if i.key_pressed(egui::Key::ArrowLeft) {
                if let Some(cur) = self.current_path.clone() {
                    match self.content {
                        Some(Content::Image(_)) => {
                            if let Some(prev) = crate::io::neighbor_image(&cur, false) { file_to_load = Some(prev); }
                        }
                        Some(Content::Text(_)) => {
                            if let Some(prev) = crate::io::neighbor_text(&cur, false) { file_to_load = Some(prev); }
                        }
                        _ => {}
                    }
                }
            }
            // Support '<' and '>' typed keys for both images and text
            for ev in &i.events {
                if let egui::Event::Text(t) = ev {
                    if t == ">" {
                        if let Some(cur) = self.current_path.clone() {
                            match self.content {
                                Some(Content::Image(_)) => { if let Some(next) = crate::io::neighbor_image(&cur, true) { file_to_load = Some(next); } }
                                Some(Content::Text(_)) => { if let Some(next) = crate::io::neighbor_text(&cur, true) { file_to_load = Some(next); } }
                                _ => {}
                            }
                        }
                    } else if t == "<" {
                        if let Some(cur) = self.current_path.clone() {
                            match self.content {
                                Some(Content::Image(_)) => { if let Some(prev) = crate::io::neighbor_image(&cur, false) { file_to_load = Some(prev); } }
                                Some(Content::Text(_)) => { if let Some(prev) = crate::io::neighbor_text(&cur, false) { file_to_load = Some(prev); } }
                                _ => {}
                            }
                        }
                    }
                }
            }
        });

        // Modern About dialog
        if self.show_about {
            egui::Window::new("About Gemini File Viewer")
                .collapsible(false)
                .resizable(false)
                .open(&mut self.show_about)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("📁").size(48.0));
                        ui.add_space(12.0);
                        ui.label(RichText::new("Gemini File Viewer").heading().strong());
                        ui.label(RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION"))).weak());
                        ui.add_space(16.0);
                        
                        ui.separator();
                        ui.add_space(12.0);
                        
                        ui.label(RichText::new("⌨️ Keyboard Shortcuts").strong());
                        ui.add_space(8.0);
                        
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.monospace(RichText::new("Ctrl+O").strong());
                                ui.monospace(RichText::new("Ctrl+D").strong());
                                ui.monospace(RichText::new("Ctrl+L").strong());
                                ui.monospace(RichText::new("Ctrl+W").strong());
                                ui.monospace(RichText::new("Ctrl+F").strong());
                            });
                            ui.add_space(16.0);
                            ui.vertical(|ui| {
                                ui.label("Open file");
                                ui.label("Toggle dark mode");
                                ui.label("Toggle line numbers");
                                ui.label("Toggle word wrap");
                                ui.label("Find in text");
                            });
                        });
                        
                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.monospace(RichText::new("Ctrl+Wheel").strong());
                                ui.monospace(RichText::new("Ctrl+= / Ctrl+-").strong());
                                ui.monospace(RichText::new("Ctrl+0").strong());
                            });
                            ui.add_space(16.0);
                            ui.vertical(|ui| {
                                ui.label("Zoom text/image");
                                ui.label("Zoom in/out");
                                ui.label("Reset zoom");
                            });
                        });
                    });
                });
        }
        if toggle_dark {
            self.dark_mode = !self.dark_mode;
            self.apply_theme(ctx);
            crate::settings::save_settings_to_disk(self);
        }

        // Top Toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                crate::ui::toolbar(ui, self, ctx, &mut file_to_load);
            });
        });

        // Search Bar (for text files and images with navigation)
        if matches!(self.content, Some(Content::Text(_))) || matches!(self.content, Some(Content::Image(_))) {
            egui::TopBottomPanel::top("searchbar").show(ctx, |ui| {
                crate::ui::search_bar(ui, self, &mut file_to_load);
            });
        }

        // Status Bar
        egui::TopBottomPanel::bottom("statusbar").show(ctx, |ui| {
            crate::ui::status_bar(ui, self);
        });

        // Extra status information
        egui::TopBottomPanel::bottom("status-extra").show(ctx, |ui| {
            crate::ui::status_extra(ui, self);
        });

        // Main Content
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(err) = &self.error_message {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", err));
            }

            if let Some(content) = &self.content {
                match content {
                    Content::Text(text) => {
                        let mut frame = egui::Frame::group(ui.style());
                        frame.fill = self.code_theme.background();
                        frame.show(ui, |ui| {
                            // Wrap preference
                            ui.style_mut().wrap_mode = Some(if self.word_wrap { egui::TextWrapMode::Wrap } else { egui::TextWrapMode::Extend });
                            egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                                let text_style = egui::TextStyle::Monospace;
                                let mut font_id = text_style.resolve(ui.style());
                                font_id.size = (font_id.size * self.text_zoom).clamp(8.0, 48.0);
                                let text_color = self.code_theme.foreground();

                                let do_line_numbers = self.show_line_numbers && !self.text_is_big;
                                let do_highlight = !self.text_is_big && text.len() <= HIGHLIGHT_CHAR_THRESHOLD;
                                if do_line_numbers || do_highlight || !self.search_query.is_empty() {
                                    let mut bracket_depth: i32 = 0;
                                    let mut in_block_comment = false;
                                    let ext = self
                                        .current_path
                                        .as_ref()
                                        .and_then(|p| p.extension().and_then(|s| s.to_str()))
                                        .unwrap_or("")
                                        .to_lowercase();
                                    // Determine target line for current match
                                    let target_line = if !self.search_query.is_empty() && self.search_count > 0 {
                                        search::find_target_line(text, &self.search_query, self.search_current)
                                    } else { None };
                                    // Render per line and capture rect
                                    let mut counter: usize = 0;
                                    let mut target_rect: Option<egui::Rect> = None;
                                    for (i, line) in text.lines().enumerate() {
                                        let mut line_job = LayoutJob::default();
                                        if do_line_numbers {
                                            line_job.append(&format!("{:>4} ", i + 1), 0.0, egui::TextFormat { font_id: font_id.clone(), color: self.code_theme.comment(), ..Default::default() });
                                        }
                                        highlight::append_highlighted(&mut line_job, line, &ext, &self.search_query, font_id.clone(), text_color, do_highlight, &mut bracket_depth, self.search_current, &mut counter, &mut in_block_comment, self.code_theme);
                                        let resp = ui.label(line_job);
                                        if target_line == Some(i) { target_rect = Some(resp.rect); }
                                    }
                                    if let Some(rect) = target_rect { ui.scroll_to_rect(rect, Some(egui::Align::Center)); }
                                } else {
                                    ui.label(RichText::new(text).monospace().size(font_id.size));
                                }
                            });
                        });
                    }
                    Content::Image(texture) => {
                        let viewport = ui.available_size();
                        egui::ScrollArea::both().show(ui, |ui| {
                            ui.centered_and_justified(|ui| {
                                let size = texture.size();
                                let mut effective_zoom = self.image_zoom;
                                if self.image_fit {
                                    // Use the outer viewport size captured before the ScrollArea
                                    let sx = if size[0] > 0 { viewport.x / size[0] as f32 } else { 1.0 };
                                    let sy = if size[1] > 0 { viewport.y / size[1] as f32 } else { 1.0 };
                                    let fit = sx.min(sy);
                                    if fit.is_finite() && fit > 0.0 {
                                        effective_zoom = fit.clamp(0.1, 6.0);
                                    }
                                }
                                let desired = egui::vec2(size[0] as f32 * effective_zoom, size[1] as f32 * effective_zoom);
                                let image = egui::Image::new(texture).fit_to_exact_size(desired);
                                let resp = ui.add(image);
                                if resp.hovered() {
                                    let scroll = ui.input(|i| i.raw_scroll_delta.y);
                                    if scroll != 0.0 {
                                        self.image_fit = false;
                                        let factor = if scroll > 0.0 { 1.10 } else { 1.0 / 1.10 };
                                        self.image_zoom = (self.image_zoom * factor).clamp(0.1, 6.0);
                                    }
                                }
                            });
                        });
                    }
                }
            } else if self.error_message.is_none() {
                // Modern welcome screen
                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() * 0.2);
                    
                    // App icon and title
                    ui.label(RichText::new("📁").size(64.0));
                    ui.add_space(16.0);
                    ui.label(RichText::new("Gemini File Viewer").heading().strong());
                    ui.add_space(8.0);
                    ui.label(RichText::new("A modern file viewer for text and images").weak());
                    
                    ui.add_space(32.0);
                    
                    // Centered Open File button
                    ui.vertical_centered(|ui| {
                        let mut open_button = egui::Button::new(RichText::new("📂 Open File").strong());
                        open_button = open_button.fill(egui::Color32::from_rgb(34, 197, 94)); // Green
                        if ui.add(open_button).clicked() {
                            if let Some(path) = FileDialog::new()
                                .add_filter("All Supported", &["txt","rs","py","toml","md","json","js","html","css","png","jpg","jpeg","gif","bmp","webp"])
                                .add_filter("Images", &["png","jpg","jpeg","gif","bmp","webp"])
                                .add_filter("Text/Source", &["txt","rs","py","toml","md","json","js","html","css"])
                                .pick_file()
                            {
                                file_to_load = Some(path);
                            }
                        }
                    });
                    
                });
            }
        });

        // Deferred file loading to avoid borrow issues
        if let Some(path) = file_to_load {
            self.load_file(path, ctx);
        }
    }
}
