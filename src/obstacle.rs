use super::engine;
use super::projection;
use super::rectangle;

pub struct Obstacle {
    base: rectangle::Rectangle,
    height: f32,
}

impl Obstacle {
    pub fn new(center: (f32, f32), size: (f32, f32), height: f32) -> Obstacle {
        Obstacle {
            base: rectangle::Rectangle::new(center, size),
            height,
        }
    }

    pub fn draw(&self, fov_distance: f32, y_level: f32) {
        let y_low = y_level;
        let y_high = y_level + self.height;

        self.base.draw(y_low, fov_distance);
        self.base.draw(y_high, fov_distance);

        let corners_low = self.base.get_corners(y_low);
        let corners_high = self.base.get_corners(y_high);

        for i in 0..4 {
            engine::draw_line(
                projection::to_screen(corners_low[i], fov_distance),
                projection::to_screen(corners_high[i], fov_distance),
            );
        }
    }
}
