//! Compares docked egui tabs, in-app floating egui windows,
//! and real OS-level native viewports. This is a test, not product code.

use std::{fs, path::PathBuf};

use docking_workspace_poc::{DocumentState, Panel};
use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, TabViewer};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct WindowBounds {
    position: [f32; 2],
    inner_size: [f32; 2],
}

#[derive(Clone, Serialize, Deserialize)]
struct SavedWorkspace {
    dock: DockState<Panel>,
    document: DocumentState,
    native_open: [bool; 3],
    native_bounds: [Option<WindowBounds>; 3],
}

impl Default for SavedWorkspace {
    fn default() -> Self {
        let mut dock = DockState::new(vec![Panel::Canvas]);

        // Start with Pages and Layers on the left, Inspector on the right.
        let [canvas, left] = dock.main_surface_mut().split_left(
            NodeIndex::root(),
            0.24,
            vec![Panel::Pages],
        );
        dock.main_surface_mut()
            .split_below(left, 0.55, vec![Panel::Layers]);
        dock.main_surface_mut()
            .split_right(canvas, 0.72, vec![Panel::Inspector]);

        Self {
            dock,
            document: DocumentState::default(),
            native_open: [false; 3],
            native_bounds: [None; 3],
        }
    }
}

fn state_path() -> PathBuf {
    let root = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    root.join("ComicPanelComposerPoCs")
        .join("docking-workspace.json")
}

struct WorkspaceApp {
    workspace: SavedWorkspace,
    status: String,
}

impl WorkspaceApp {
    fn new() -> Self {
        let path = state_path();
        let (workspace, status) = match fs::read_to_string(&path) {
            Ok(contents) => match serde_json::from_str::<SavedWorkspace>(&contents) {
                Ok(saved) => (saved, format!("Loaded workspace from {}", path.display())),
                Err(error) => (
                    SavedWorkspace::default(),
                    format!("Invalid saved layout ({error}); using defaults"),
                ),
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => (
                SavedWorkspace::default(),
                "Default layout. Nothing has been saved yet.".to_owned(),
            ),
            Err(error) => (
                SavedWorkspace::default(),
                format!("Could not read saved layout ({error}); using defaults"),
            ),
        };
        Self { workspace, status }
    }

    fn save(&self) -> Result<(), String> {
        let path = state_path();
        let parent = path.parent().ok_or("Missing settings directory")?;
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        let contents = serde_json::to_string_pretty(&self.workspace)
            .map_err(|error| error.to_string())?;
        fs::write(&path, contents).map_err(|error| error.to_string())
    }

    fn detach_native(&mut self, panel: Panel) {
        let Some(slot) = panel.native_slot() else {
            return;
        };
        if self.workspace.native_open[slot] {
            return;
        }

        if let Some(path) = self.workspace.dock.find_tab(&panel) {
            if self.workspace.dock.remove_tab(path).is_some() {
                self.workspace.native_open[slot] = true;
            }
        }
    }

    fn redock(&mut self, panel: Panel, ctx: &egui::Context) {
        let Some(slot) = panel.native_slot() else {
            return;
        };
        if self.workspace.native_open[slot] {
            self.workspace.native_open[slot] = false;
            self.workspace.dock.push_to_focused_leaf(panel);
            ctx.send_viewport_cmd_to(panel_viewport_id(panel), egui::ViewportCommand::Close);
        }
    }

    fn native_windows(&mut self, ctx: &egui::Context) {
        let mut redock_requests = Vec::new();

        for panel in Panel::FLOATABLE {
            let slot = panel.native_slot().expect("tool panel has a native slot");
            if !self.workspace.native_open[slot] {
                continue;
            }

            let mut builder = egui::ViewportBuilder::default()
                .with_title(format!("{} — native Windows window", panel.title()))
                .with_inner_size([320.0, 330.0]);

            if let Some(bounds) = self.workspace.native_bounds[slot] {
                builder = builder
                    .with_position(egui::pos2(
                        bounds.position[0],
                        bounds.position[1],
                    ))
                    .with_inner_size(bounds.inner_size);
            }

            let mut request_redock = false;
            let mut observed_bounds = None;
            ctx.show_viewport_immediate(
                panel_viewport_id(panel),
                builder,
                |window_ui, class| {
                    if window_ui.input(|input| input.viewport().close_requested()) {
                        request_redock = true;
                    }

                    egui::CentralPanel::default().show(window_ui, |ui| {
                        ui.heading(panel.title());
                        if class == egui::ViewportClass::EmbeddedWindow {
                            ui.colored_label(
                                egui::Color32::RED,
                                "NOT a native window: the backend embedded this viewport",
                            );
                        } else {
                            ui.label(format!("Viewport class: {class:?}"));
                        }
                        if ui.button("Dock back into main workspace").clicked() {
                            request_redock = true;
                        }
                        ui.separator();
                        draw_panel_contents(ui, panel, &mut self.workspace.document);
                        ui.separator();
                        show_dpi_information(ui);
                    });

                    observed_bounds = window_ui.input(|input| {
                        match (input.viewport().outer_rect, input.viewport().inner_rect) {
                            (Some(outer), Some(inner)) => Some(WindowBounds {
                                position: [outer.min.x, outer.min.y],
                                inner_size: [inner.width(), inner.height()],
                            }),
                            _ => None,
                        }
                    });
                },
            );

            if let Some(bounds) = observed_bounds {
                self.workspace.native_bounds[slot] = Some(bounds);
            }
            if request_redock {
                redock_requests.push(panel);
            }
        }

        for panel in redock_requests {
            self.redock(panel, ctx);
        }
    }
}

fn panel_viewport_id(panel: Panel) -> egui::ViewportId {
    egui::ViewportId::from_hash_of(("docking-poc-native-panel", panel))
}

fn show_dpi_information(ui: &mut egui::Ui) {
    let native_ppp = ui.input(|input| input.viewport().native_pixels_per_point);
    ui.label(format!("UI pixels per point: {:.2}", ui.ctx().pixels_per_point()));
    ui.label(format!("OS pixels per point: {native_ppp:?}"));
    ui.small("Try moving this window between screens with different display scaling.");
}

fn draw_panel_contents(ui: &mut egui::Ui, panel: Panel, document: &mut DocumentState) {
    match panel {
        Panel::Canvas => {
            ui.heading("Sample comic canvas");
            ui.label("The opacity and visibility of this sample panel are shared with every tool window.");
            let size = egui::vec2(ui.available_width().max(100.0), ui.available_height().max(160.0));
            let (area, _) = ui.allocate_exact_size(size, egui::Sense::hover());
            let page = egui::Rect::from_center_size(
                area.center(),
                egui::vec2(area.width().min(360.0) * 0.85, area.height().min(420.0) * 0.85),
            );
            ui.painter().rect_filled(page, egui::CornerRadius::same(2), egui::Color32::WHITE);
            if document.layer_visible {
                let illustration = page.shrink(16.0);
                ui.painter().rect_filled(
                    illustration,
                    egui::CornerRadius::same(4),
                    egui::Color32::from_rgba_unmultiplied(80, 145, 225, document.opacity),
                );
            }
            ui.painter().text(
                page.center(),
                egui::Align2::CENTER_CENTER,
                format!("PAGE {}", document.selected_page),
                egui::FontId::proportional(18.0),
                egui::Color32::BLACK,
            );
        }
        Panel::Pages => {
            ui.label("Change the selected page:");
            for page in 1..=3 {
                if ui
                    .selectable_label(document.selected_page == page, format!("Page {page}"))
                    .clicked()
                {
                    document.selected_page = page;
                }
            }
        }
        Panel::Layers => {
            ui.checkbox(&mut document.layer_visible, "Show sample artwork layer");
            ui.label("Use this control while the Inspector is in another OS window.");
        }
        Panel::Inspector => {
            ui.label("Sample artwork opacity:");
            ui.add(egui::Slider::new(&mut document.opacity, 0..=255).text("Alpha"));
            ui.checkbox(&mut document.layer_visible, "Artwork visible");
            ui.label(format!("Current page: {}", document.selected_page));
        }
    }
}

struct DockedTabViewer<'a> {
    document: &'a mut DocumentState,
    detach_requests: &'a mut Vec<Panel>,
}

impl TabViewer for DockedTabViewer<'_> {
    type Tab = Panel;

    fn id(&mut self, tab: &mut Panel) -> egui::Id {
        egui::Id::new(*tab)
    }

    fn title(&mut self, tab: &mut Panel) -> egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Panel) {
        if tab.native_slot().is_some() {
            if ui.button("Detach as a REAL Windows window").clicked() {
                self.detach_requests.push(*tab);
            }
            ui.separator();
        }
        draw_panel_contents(ui, *tab, self.document);
    }
}

impl eframe::App for WorkspaceApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut save_requested = false;
        let mut reset_requested = false;
        let mut detach_requests = Vec::new();

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("Docking workspace — technology experiment");
                if ui.button("Save workspace").clicked() {
                    save_requested = true;
                }
                if ui.button("Reset workspace").clicked() {
                    reset_requested = true;
                }
            });
            ui.label("Dragging a dock tab out creates an IN-APP egui window, not an OS window.");
            ui.label("To test REAL Windows windows, use 'Detach as a REAL Windows window' in a tool panel.");
            ui.small(self.status.as_str());
            ui.separator();

            let mut viewer = DockedTabViewer {
                document: &mut self.workspace.document,
                detach_requests: &mut detach_requests,
            };
            DockArea::new(&mut self.workspace.dock).show_inside(ui, &mut viewer);
        });

        for panel in detach_requests {
            self.detach_native(panel);
        }
        let ctx = ui.ctx().clone();
        self.native_windows(&ctx);

        if reset_requested {
            for panel in Panel::FLOATABLE {
                ctx.send_viewport_cmd_to(panel_viewport_id(panel), egui::ViewportCommand::Close);
            }
            self.workspace = SavedWorkspace::default();
            self.status = "Workspace reset in memory (saved layout is unchanged).".to_owned();
        }
        if save_requested {
            self.status = match self.save() {
                Ok(()) => format!("Workspace saved to {}", state_path().display()),
                Err(error) => format!("Save failed: {error}"),
            };
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Comic Panel Composer — Docking PoC")
            .with_inner_size([1150.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Comic Panel Composer — Docking PoC",
        options,
        Box::new(|_cc| Ok(Box::new(WorkspaceApp::new()))),
    )
}
