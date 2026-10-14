//! Minimal geometry model for the four-corner polygon experiment.
//! This is deliberately independent of egui. It is not the final document model.

/// A four-corner polygon with resettable coordinates.
/// Points are stored as x/y pairs: [x0, y0, x1, y1, x2, y2, x3, y3].
#[derive(Debug, Clone)]
pub struct Polygon {
    points: [f32; 8],
    original_points: [f32; 8],
}

impl Default for Polygon {
    fn default() -> Self {
        Self::new()
    }
}

impl Polygon {
    pub fn new() -> Self {
        let points = [100.0, 100.0, 300.0, 100.0, 350.0, 300.0, 150.0, 300.0];
        Self {
            points,
            original_points: points,
        }
    }

    pub fn get_points(&self) -> &[f32; 8] {
        &self.points
    }

    pub fn set_point(&mut self, index: usize, x: f32, y: f32) {
        if index < 4 {
            let offset = index * 2;
            self.points[offset] = x;
            self.points[offset + 1] = y;
        }
    }

    pub fn get_closest_handle_index(&self, x: f32, y: f32, radius: f32) -> Option<usize> {
        if radius < 0.0 {
            return None;
        }

        let radius_squared = radius * radius;
        for (index, pair) in self.points.chunks_exact(2).enumerate() {
            let dx = pair[0] - x;
            let dy = pair[1] - y;
            if dx * dx + dy * dy <= radius_squared {
                return Some(index);
            }
        }
        None
    }

    pub fn reset(&mut self) {
        self.points = self.original_points;
    }
}

#[cfg(test)]
mod tests {
    use super::Polygon;

    #[test]
    fn new_polygon_has_expected_corners() {
        assert_eq!(
            *Polygon::new().get_points(),
            [100.0, 100.0, 300.0, 100.0, 350.0, 300.0, 150.0, 300.0]
        );
    }

    #[test]
    fn moving_one_corner_does_not_change_the_others() {
        let mut polygon = Polygon::new();
        polygon.set_point(2, 275.0, 425.0);
        assert_eq!(
            *polygon.get_points(),
            [100.0, 100.0, 300.0, 100.0, 275.0, 425.0, 150.0, 300.0]
        );
    }

    #[test]
    fn reset_restores_the_original_corners() {
        let mut polygon = Polygon::new();
        polygon.set_point(0, 50.0, 75.0);
        polygon.set_point(3, 425.0, 325.0);
        polygon.reset();
        assert_eq!(*polygon.get_points(), *Polygon::new().get_points());
    }

    #[test]
    fn hit_testing_finds_a_nearby_handle() {
        let polygon = Polygon::new();
        assert_eq!(polygon.get_closest_handle_index(106.0, 108.0, 10.0), Some(0));
        assert_eq!(polygon.get_closest_handle_index(300.0, 100.0, 0.0), Some(1));
        assert_eq!(polygon.get_closest_handle_index(500.0, 500.0, 15.0), None);
    }

    #[test]
    fn hit_testing_rejects_a_negative_radius() {
        assert_eq!(
            Polygon::new().get_closest_handle_index(100.0, 100.0, -1.0),
            None
        );
    }
}
