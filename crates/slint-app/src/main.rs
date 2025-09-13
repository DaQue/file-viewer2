use std::path::PathBuf;
use std::cell::RefCell;
use std::rc::Rc;
use file_viewer_core as core;
use slint::{Image as SlintImage, SharedPixelBuffer, Rgba8Pixel};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

slint::include_modules!();

#[derive(Default)]
struct AppState {
    current_path: Option<PathBuf>,
    recents: Vec<PathBuf>,
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    ui.set_status_text("Ready".into());
    let state: Rc<RefCell<AppState>> = Rc::new(RefCell::new(AppState::default()));
    // Load persisted recents at startup
    load_recents_and_update(&ui, state.clone());

    // Open File → use rfd + core loaders
    {
        let ui_weak = ui.as_weak();
        let state_rc = state.clone();
        ui.on_open_file(move || {
            let Some(path) = rfd::FileDialog::new()
                .add_filter(
                    "All Supported",
                    &["txt","rs","py","toml","md","json","js","html","css","png","jpg","jpeg","gif","bmp","webp"],
                )
                .add_filter("Images", &["png","jpg","jpeg","gif","bmp","webp"])
                .add_filter("Text/Source", &["txt","rs","py","toml","md","json","js","html","css"])
                .pick_file()
            else { return; };

            handle_open_path(ui_weak.clone(), state_rc.clone(), path);
        });
    }

    // Open Folder → dialog and status update
    {
        let ui_weak = ui.as_weak();
        ui.on_open_folder(move || {
            let Some(dir) = rfd::FileDialog::new().pick_folder() else { return; };
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_status_text(format!("Folder: {}", dir.display()).into());
            }
        });
    }

    // Toggle Theme → flip dark-mode and reflect in status
    {
        let ui_weak = ui.as_weak();
        ui.on_toggle_theme(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let dark = ui.get_dark_mode();
                ui.set_dark_mode(!dark);
                ui.set_status_text(if dark { "Theme: Light" } else { "Theme: Dark" }.into());
            }
        });
    }

    // Open recent
    {
        let ui_weak = ui.as_weak();
        let state_rc = state.clone();
        ui.on_open_recent(move |path: slint::SharedString| {
            handle_open_path(ui_weak.clone(), state_rc.clone(), PathBuf::from(path.as_str()));
        });
    }

    // Prev / Next neighbors
    {
        let ui_weak = ui.as_weak();
        let state_rc = state.clone();
        ui.on_prev_file(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let show_img = ui.get_show_image();
                drop(ui);
                let cur_opt = state_rc.borrow().current_path.clone();
                if let Some(cur) = cur_opt {
                    let next_path = if show_img {
                        core::neighbor_image(&cur, false)
                    } else {
                        core::neighbor_text(&cur, false)
                    };
                    if let Some(p) = next_path { handle_open_path(ui_weak.clone(), state_rc.clone(), p); }
                }
            }
        });
    }

    // Toggle Recents: always toggle; if opened with empty list, show hint
    {
        let ui_weak = ui.as_weak();
        ui.on_toggle_recents(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let new_state = !ui.get_show_recents();
                ui.set_show_recents(new_state);
                if new_state && ui.get_recents_count() == 0 {
                    ui.set_status_text("No recent files".into());
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state_rc = state.clone();
        ui.on_next_file(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let show_img = ui.get_show_image();
                drop(ui);
                let cur_opt = state_rc.borrow().current_path.clone();
                if let Some(cur) = cur_opt {
                    let next_path = if show_img {
                        core::neighbor_image(&cur, true)
                    } else {
                        core::neighbor_text(&cur, true)
                    };
                    if let Some(p) = next_path { handle_open_path(ui_weak.clone(), state_rc.clone(), p); }
                }
            }
        });
    }

    ui.run()
}

fn display_basename(path: &PathBuf) -> String {
    path.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| path.display().to_string())
}

const MAX_RECENTS: usize = 10;

fn push_recent(state: &mut AppState, path: &PathBuf) {
    state.recents.retain(|p| p != path);
    state.recents.push(path.clone());
    if state.recents.len() > MAX_RECENTS {
        let overflow = state.recents.len() - MAX_RECENTS;
        state.recents.drain(0..overflow);
    }
}

fn set_recents_model(ui: &MainWindow, state: &AppState) {
    let items: Vec<slint::SharedString> = state
        .recents
        .iter()
        .rev()
        .map(|p| slint::SharedString::from(p.to_string_lossy().to_string()))
        .collect();
    ui.set_recents(slint::ModelRc::new(slint::VecModel::from(items)));
    ui.set_recents_count(state.recents.len() as i32);
}

fn handle_open_path(ui_weak: slint::Weak<MainWindow>, state_rc: Rc<RefCell<AppState>>, path: PathBuf) {
    if let Some(ui) = ui_weak.upgrade() {
        let status = if core::is_supported_image(&path) {
            match core::load_image(&path) {
                Ok(img) => {
                    let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(img.width as u32, img.height as u32);
                    buf.make_mut_bytes().copy_from_slice(&img.pixels);
                    let slint_img = SlintImage::from_rgba8(buf);

                    ui.set_current_image(slint_img);
                    ui.set_image_w(img.width as i32);
                    ui.set_image_h(img.height as i32);
                    ui.set_image_zoom(1.0);
                    ui.set_image_fit(true);
                    ui.set_show_image(true);
                    ui.set_lines(slint::ModelRc::new(slint::VecModel::from(Vec::<Line>::new())));

                    let mut s = state_rc.borrow_mut();
                    s.current_path = Some(path.clone());
                    push_recent(&mut s, &path);
                    let snapshot = s.recents.clone();
                    drop(s);
                    set_recents_model(&ui, &state_rc.borrow());
                    save_recents(&snapshot);

                    ui.set_show_recents(false);
                    format!("Image: {} ({}x{})", display_basename(&path), img.width, img.height)
                }
                Err(e) => format!("Error loading image: {}", e),
            }
        } else if let Ok((text, lossy, lines)) = core::load_text(&path) {
            let mut s = state_rc.borrow_mut();
            s.current_path = Some(path.clone());
            push_recent(&mut s, &path);
            let snapshot = s.recents.clone();
            drop(s);
            set_recents_model(&ui, &state_rc.borrow());
            save_recents(&snapshot);

            let lossy_flag = if lossy { ", UTF-8 (lossy)" } else { "" };
            let mut line_items: Vec<Line> = Vec::with_capacity(lines);
            for (i, line) in text.lines().enumerate() {
                let span = Span { text: line.into(), match_kind: 0 };
                let spans_model = slint::VecModel::from(vec![span]);
                let line_item = Line { line_no: format!("{}", i + 1).into(), spans: slint::ModelRc::new(spans_model) };
                line_items.push(line_item);
            }
            ui.set_lines(slint::ModelRc::new(slint::VecModel::from(line_items)));
            ui.set_show_image(false);
            ui.set_show_recents(false);
            format!("Text: {} ({} lines{})", display_basename(&path), lines, lossy_flag)
        } else {
            "Unsupported file type".to_string()
        };
        ui.set_status_text(status.into());
    }
}

// --- Persistence for recents ---

#[derive(Serialize, Deserialize, Default)]
struct RecentsFile {
    recents: Vec<String>,
}

fn recents_path() -> Option<PathBuf> {
    ProjectDirs::from("", "", "gemini-file-viewer")
        .map(|dirs| dirs.config_dir().join("slint_recents.json"))
}

fn load_recents_and_update(ui: &MainWindow, state_rc: Rc<RefCell<AppState>>) {
    if let Some(path) = recents_path() {
        if let Ok(data) = std::fs::read(&path) {
            if let Ok(file) = serde_json::from_slice::<RecentsFile>(&data) {
                let mut s = state_rc.borrow_mut();
                s.recents = file.recents.into_iter().map(PathBuf::from).collect();
                drop(s);
                set_recents_model(ui, &state_rc.borrow());
            }
        }
    }
}

fn save_recents(recents: &Vec<PathBuf>) {
    if let Some(path) = recents_path() {
        if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
        let file = RecentsFile { recents: recents.iter().map(|p| p.to_string_lossy().to_string()).collect() };
        if let Ok(buf) = serde_json::to_vec_pretty(&file) {
            let _ = std::fs::write(path, buf);
        }
    }
}
