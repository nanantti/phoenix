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

    pub fn draw(&self, projection: &projection::Projection, y_level: f32) {
        if !self.base.is_rectangle_in_view_range(projection) {
            return;
        }

        let y_low = y_level;
        let y_high = y_level + self.height;

        self.base.draw(y_low, projection);
        self.base.draw(y_high, projection);

        let corners_low = self.base.get_corners(y_low);
        let corners_high = self.base.get_corners(y_high);

        for i in 0..4 {
            engine::draw_line(
                projection.to_screen(&corners_low[i]),
                projection.to_screen(&corners_high[i]),
            );
        }
    }

    pub fn check_collision(&self, other: &rectangle::Rectangle) -> bool {
        self.base.collision(other)
    }

    pub fn check_collision_obst(&self, other: &Obstacle) -> bool {
        self.base.collision(&other.base)
    }
}
