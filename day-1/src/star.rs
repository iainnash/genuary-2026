use nannou::{
    geom::{pt2, Point2},
    math::deg_to_rad,
};

pub struct Star {
    width: f32,
    height: f32,
}

impl Star {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    fn move_by_quadrant(&self, x_offset: f32, y_offset: f32, points: Vec<Point2>) -> Vec<Point2> {
        points
            .iter()
            .map(|p| pt2(p.x + x_offset * self.width, p.y + y_offset * self.height))
            .collect()
    }
    fn partial_circle_points(&self, start_angle: i32, end_angle: i32) -> Vec<Point2> {
        (start_angle..end_angle)
            .map(|angle| {
                let rad = deg_to_rad(angle as f32);
                let x = self.width * rad.cos();
                let y = self.height * rad.sin();
                pt2(x, y)
            })
            .collect()
    }
    pub fn points(&self) -> Vec<Point2> {
        vec![
            self.move_by_quadrant(1., -1., self.partial_circle_points(90, 180)),
            self.move_by_quadrant(-1., -1., self.partial_circle_points(0, 90)),
            self.move_by_quadrant(-1., 1., self.partial_circle_points(270, 360)),
            self.move_by_quadrant(1., 1., self.partial_circle_points(180, 270)),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}
