Egui → Slint Migration Checklist

Current Snapshot
- Branch: `rewrite/major-change`
- Baseline: tag `known-good-2025-09-13`, branch `known-good`
- Crates:
  - `file-viewer-core` (crates/core) — pure logic (text/image load, neighbors, search)
  - `gemini-file-viewer` (crates/app) — current egui app for reference
  - `file-viewer-slint-app` (crates/slint-app) — new Slint UI
- Run:
  - Egui: `cargo run -p gemini-file-viewer`
  - Slint: `cargo run -p file-viewer-slint-app`

Progress
- [x] Extract core (no egui types)
- [x] Stand up Slint MainWindow with status bar & menu buttons
- [x] Wire callbacks to core (open file, open folder, toggle theme)
- [x] Implement text viewer in Slint (option 1: spans per line) — basic, no syntax yet
- [x] Add image viewer path (PNG/JPEG) with RGBA → Slint Image
- [x] Image Fit-to-Screen (auto fit on open; zoom disables fit)
- [x] Recents list with persistence across runs
- [x] Prev/Next neighbors (text and images)
- [ ] Keyboard shortcuts (minimal set to re-add)
- [ ] Add theming tokens + toggle (dark/light palettes + code/search colors)
- [ ] Remove egui app after UI parity
- [ ] Polish: toolbar zoom controls, focus handling, accessibility labels, DPI scaling, error popup, button sizing

Immediate Next Steps
1) Keyboard shortcuts (minimal)
   - Re-add Ctrl+O, Ctrl+0/=/− and typed '<'/'>' using FocusScope.
2) Image Zoom controls
   - Add toolbar buttons: Fit / 100% / + / − and a zoom indicator.
3) Theming tokens
   - Dark/light palette tokens in `.slint` with consistent bindings.

Resumption Notes
- Ensure branch: `git checkout rewrite/major-change`
- Edit next:
  - `crates/slint-app/src/main.rs` (image viewer, recents, keyboard)
  - `crates/slint-app/ui/mainwindow.slint` (image area, ListView models, palettes)
  - `crates/core/src/lib.rs` (optional: add tokenizer API later)
- Large files: keep thresholds; skip heavy features for huge text.
- Linux runtime deps for Slint may require GL/X11/Wayland libs (`libgl1`, `libxkbcommon-x11-0`).
