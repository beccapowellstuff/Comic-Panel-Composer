//! Compares docked egui tabs, in-app floating egui windows,
//! and real OS-level native viewports. This is a test, not product code.

use std::{fs, path::PathBuf};

use docking_workspace_poc::{DocumentState, Panel};
use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, TabViewer};
use serde::{Deserialize, Serialize};

mod window_placement;
mod window_activation;

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
    // Each independent OS viewport hosts its own egui_dock tab hierarchy.
    // New field is optional so older saved PoC workspaces still load.
    #[serde(default)]
    native_docks: [Option<DockState<Panel>>; 3],
    native_bounds: [Option<WindowBounds>; 3],
    // Version 2: monitor-independent physical virtual-desktop coordinates.
    // Old saves omit this field. Their ambiguous logical positions are never
    // reused, preventing the previous off-screen restore regression.
    #[serde(default)]
    native_position_px: [Option<[i32; 2]>; 3],
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
            native_docks: [None, None, None],
            native_bounds: [None; 3],
            native_position_px: [None; 3],
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
    // Runtime-only: apply saved window placement on creation, never during movement.
    native_initialised: [bool; 3],
    main_was_focused: bool,
}

impl WorkspaceApp {
    fn new() -> Self {
        let path = state_path();
        let (mut workspace, status) = match fs::read_to_string(&path) {
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
        // Migrate old PoC layouts: previous releases saved one panel per
        // native OS viewport without a DockState of their own.
        for panel in Panel::FLOATABLE {
            let slot = panel.native_slot().unwrap();
            if workspace.native_open[slot] && workspace.native_docks[slot].is_none() {
                workspace.native_docks[slot] = Some(DockState::new(vec![panel]));
            }
        }
        Self {
            workspace,
            status,
            native_initialised: [false; 3],
            main_was_focused: false,
        }
    }

    fn save_workspace_to_file(&self) -> Result<(), String> {
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
                self.workspace.native_docks[slot] = Some(DockState::new(vec![panel]));
                self.native_initialised[slot] = false;
            }
        }
    }

    fn redock(&mut self, panel: Panel, ctx: &egui::Context) {
        let Some(slot) = panel.native_slot() else {
            return;
        };
        if !self.workspace.native_open[slot] {
            return;
        }
        self.workspace.native_open[slot] = false;
        self.native_initialised[slot] = false;
        if let Some(group) = self.workspace.native_docks[slot].take() {
            for tab in Panel::FLOATABLE {
                if group.find_tab(&tab).is_some() {
                    self.workspace.dock.push_to_focused_leaf(tab);
                }
            }
        }
        ctx.send_viewport_cmd_to(panel_viewport_id(panel), egui::ViewportCommand::Close);
    }

    fn merge_native_groups(&mut self, from: usize, to: usize, ctx: &egui::Context) {
        if from == to || !self.workspace.native_open[from] || !self.workspace.native_open[to] {
            return;
        }
        let Some(source) = self.workspace.native_docks[from].take() else {
            return;
        };
        let Some(target) = self.workspace.native_docks[to].as_mut() else {
            self.workspace.native_docks[from] = Some(source);
            return;
        };
        for tab in Panel::FLOATABLE {
            if source.find_tab(&tab).is_some() {
                target.push_to_focused_leaf(tab);
            }
        }
        self.workspace.native_open[from] = false;
        self.native_initialised[from] = false;
        // Closing the empty source viewport must not discard its former
        // screen placement; reopening another panel may reuse that position.
        ctx.send_viewport_cmd_to(
            panel_viewport_id(Panel::FLOATABLE[from]),
            egui::ViewportCommand::Close,
        );
    }

    fn recover_all_windows(&mut self, ctx: &egui::Context) {
        for panel in Panel::FLOATABLE {
            self.redock(panel, ctx);
        }
        // Keep the document and dock arrangement but discard all saved OS
        // placement so detaching again starts somewhere Windows can show it.
        self.workspace.native_bounds = [None; 3];
        self.workspace.native_position_px = [None; 3];
        self.status = "Recovered all floating tool panels to the main workspace. Save the workspace to make this persistent.".to_owned();
    }

    fn native_windows(&mut self, ctx: &egui::Context) {
        let mut redock_requests = Vec::new();
        let mut merge_requests = Vec::new();

        for panel in Panel::FLOATABLE {
            let slot = panel.native_slot().expect("tool panel has a native slot");
            if !self.workspace.native_open[slot] {
                continue;
            }

            // Let Windows create the viewport where it can be drawn safely.
            // In the first native-frame callback, position it ONCE using the
            // viewport's actual DPI. Applying saved egui point coordinates in
            // the builder on the primary monitor previously put windows offscreen.
            let mut builder = egui::ViewportBuilder::default()
                .with_title(format!("{} — native Windows window", panel.title()));
            if !self.native_initialised[slot] {
                let saved_size = self.workspace.native_bounds[slot]
                    .map(|bounds| bounds.inner_size)
                    .filter(|size| {
                        size.iter().all(|n| n.is_finite())
                            && (100.0..=2500.0).contains(&size[0])
                            && (100.0..=2500.0).contains(&size[1])
                    })
                    .unwrap_or([320.0, 330.0]);
                builder = builder.with_inner_size(saved_size);
            }
            let restore_position = if !self.native_initialised[slot] {
                self.workspace.native_position_px[slot]
                    .and_then(window_placement::safe_restore_position)
            } else {
                None
            };

            let available_merge_targets: Vec<_> = Panel::FLOATABLE
                .iter()
                .copied()
                .filter(|other| {
                    let other_slot = other.native_slot().unwrap();
                    other_slot != slot && self.workspace.native_open[other_slot]
                })
                .collect();
            let mut request_redock = false;
            let mut request_merge = None;
            let mut observed_bounds = None;
            let mut observed_position_px = None;
            ctx.show_viewport_immediate(
                panel_viewport_id(panel),
                builder,
                |window_ui, class| {
                    if window_ui.input(|input| input.viewport().close_requested()) {
                        request_redock = true;
                    }

                    // OuterPosition is measured in viewport points and
                    // converted by egui-winit to PHYSICAL pixels using this
                    // window's current pixels-per-point. This sends only once.
                    if let Some([x_px, y_px]) = restore_position {
                        let scale = window_ui.input(|input| input.pixels_per_point);
                        if scale.is_finite() && scale > 0.0 {
                            window_ui.send_viewport_cmd(
                                egui::ViewportCommand::OuterPosition(egui::pos2(
                                    x_px as f32 / scale,
                                    y_px as f32 / scale,
                                )),
                            );
                        }
                    }

                    egui::CentralPanel::default().show(window_ui, |ui| {
                        ui.heading(panel.title());
                        if class == egui::ViewportClass::EmbeddedWindow {
                            ui.colored_label(
                                egui::Color32::RED,
                                "NOT a native window: the backend embedded this viewport",
                            );
                        } else {
                            ui.label("Native viewport requested — verify independent window behaviour");
                        }
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("Dock all tabs in main workspace").clicked() {
                                request_redock = true;
                            }
                            ui.menu_button("Merge this window into...", |menu| {
                                if available_merge_targets.is_empty() {
                                    menu.label("Detach another native window first");
                                }
                                for destination in &available_merge_targets {
                                    if menu.button(destination.title()).clicked() {
                                        request_merge = destination.native_slot();
                                        menu.close();
                                    }
                                }
                            });
                        });
                        ui.small("Native window group: tabs below can be rearranged and stacked.");
                        ui.small("Cross-OS-window DRAG is not implemented; Merge uses a menu for this test.");
                        ui.separator();
                        let mut unused_detach = Vec::new();
                        if let Some(dock) = self.workspace.native_docks[slot].as_mut() {
                            let mut viewer = DockedTabViewer {
                                document: &mut self.workspace.document,
                                detach_requests: &mut unused_detach,
                                is_native: true,
                            };
                            DockArea::new(dock)
                                .id(egui::Id::new(("native-dock", slot)))
                                .show_inside(ui, &mut viewer);
                        }
                        ui.separator();
                        show_dpi_information(ui);
                    });

                    let (bounds, pixels) = window_ui.input(|input| {
                        let info = input.viewport();
                        let bounds = match (info.outer_rect, info.inner_rect) {
                            (Some(outer), Some(inner)) => Some(WindowBounds {
                                position: [outer.min.x, outer.min.y],
                                inner_size: [inner.width(), inner.height()],
                            }),
                            _ => None,
                        };
                        let pixels = info.outer_rect.and_then(|outer| {
                            let scale = input.pixels_per_point;
                            let x = outer.min.x * scale;
                            let y = outer.min.y * scale;
                            if x.is_finite() && y.is_finite()
                                && x.abs() < 100_000.0 && y.abs() < 100_000.0
                            {
                                Some([x.round() as i32, y.round() as i32])
                            } else {
                                None
                            }
                        });
                        (bounds, pixels)
                    });
                    observed_bounds = bounds;
                    observed_position_px = pixels;
                },
            );

            let was_initialised = self.native_initialised[slot];
            self.native_initialised[slot] = true;
            // The first callback still sees the WINDOW BEFORE a requested
            // restore movement has taken effect. Never overwrite the saved
            // physical location with that temporary initial position.
            if was_initialised || restore_position.is_none() {
                if let Some(bounds) = observed_bounds {
                    self.workspace.native_bounds[slot] = Some(bounds);
                }
                if let Some(position) = observed_position_px {
                    self.workspace.native_position_px[slot] = Some(position);
                }
            }
            if request_redock {
                redock_requests.push(panel);
            }
            if let Some(destination) = request_merge {
                merge_requests.push((slot, destination));
            }
        }

        // Process after painting; mutating docking trees mid-callback is unsafe.
        for (source, target) in merge_requests {
            self.merge_native_groups(source, target, ctx);
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
    is_native: bool,
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
        if !self.is_native && tab.native_slot().is_some() {
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
        let mut recover_requested = false;
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
                if ui.button("Recover floating panels").clicked() {
                    recover_requested = true;
                }
            });
            ui.label("Dragging a dock tab out creates an IN-APP egui window, not an OS window.");
            ui.label("To test REAL Windows windows, use 'Detach as a REAL Windows window' in a tool panel.");
            ui.small(self.status.as_str());
            ui.separator();

            let mut viewer = DockedTabViewer {
                document: &mut self.workspace.document,
                detach_requests: &mut detach_requests,
                is_native: false,
            };
            DockArea::new(&mut self.workspace.dock).show_inside(ui, &mut viewer);
        });

        for panel in detach_requests {
            self.detach_native(panel);
        }
        let ctx = ui.ctx().clone();
        self.native_windows(&ctx);

        // Only on a transition back to MAIN window focus; don't continuously
        // reorder windows and don't force focus onto an Inspector. Windows
        // handles the change in z-order without an always-on-top flag.
        let main_focused = ui.input(|input| input.viewport().focused.unwrap_or(false));
        if main_focused && !self.main_was_focused {
            let _raised = window_activation::raise_tool_windows_without_focus();
        }
        self.main_was_focused = main_focused;

        if recover_requested {
            self.recover_all_windows(&ctx);
        }

        if reset_requested {
            for panel in Panel::FLOATABLE {
                ctx.send_viewport_cmd_to(panel_viewport_id(panel), egui::ViewportCommand::Close);
            }
            self.workspace = SavedWorkspace::default();
            self.native_initialised = [false; 3];
            self.status = "Workspace reset in memory (saved layout is unchanged).".to_owned();
        }
        if save_requested {
            self.status = match self.save_workspace_to_file() {
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
