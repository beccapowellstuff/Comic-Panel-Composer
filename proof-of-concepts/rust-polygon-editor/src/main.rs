//! Small Windows desktop experiment: draw and edit one four-corner polygon.
use eframe::egui;
use egui::{Color32, Pos2, Stroke};
use polygon_poc::Polygon;

struct PolygonApp {
    polygon: Polygon,
    dragging_handle: Option<usize>,
}

impl PolygonApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            polygon: Polygon::new(),
            dragging_handle: None,
        }
    }
}

impl eframe::App for PolygonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let canvas = ui.available_rect_before_wrap();

            // The geometry model owns the coordinates; egui positions are
            // constructed only for display and pointer interaction.
            let corners: Vec<Pos2> = self
                .polygon
                .get_points()
                .chunks_exact(2)
                .map(|pair| Pos2::new(pair[0], pair[1]))
                .collect();

            let painter = ui.painter();
            painter.add(egui::Shape::convex_polygon(
                corners.clone(),
                Color32::from_rgba_unmultiplied(100, 150, 255, 100),
                Stroke::new(2.0_f32, Color32::from_rgb(0, 0, 255)),
            ));

            for (index, corner) in corners.iter().enumerate() {
                let colour = if self.dragging_handle == Some(index) {
                    Color32::RED
                } else {
                    Color32::YELLOW
                };
                painter.circle_filled(*corner, 8.0, colour);
            }

            let response = ui.interact(
                canvas,
                egui::Id::new("polygon_canvas"),
                egui::Sense::click_and_drag(),
            );

            if response.drag_started() {
                if let Some(position) = response.interact_pointer_pos() {
                    self.dragging_handle = self
                        .polygon
                        .get_closest_handle_index(position.x, position.y, 15.0);
                }
            }

            if response.dragged() {
                if let (Some(index), Some(position)) =
                    (self.dragging_handle, response.interact_pointer_pos())
                {
                    self.polygon.set_point(index, position.x, position.y);
                }
            }

            if response.drag_stopped() {
                self.dragging_handle = None;
            }

            if ui.button("Reset").clicked() {
                self.polygon.reset();
                self.dragging_handle = None;
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Rust polygon proof of concept"),
        ..Default::default()
    };

    eframe::run_native(
        "Rust polygon proof of concept",
        options,
        Box::new(|cc| Box::new(PolygonApp::new(cc))),
    )
}
