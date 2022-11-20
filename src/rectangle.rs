use super::engine;
use super::projection;

pub struct Rectangle {
    center: (f32, f32),
    size: (f32, f32),
}

impl Rectangle {
    pub fn new(center: (f32, f32), size: (f32, f32)) -> Rectangle {
        Rectangle { center, size }
    }

    pub fn collision(&self, other: &Rectangle) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let hh = self.size.0 + other.size.0;

        let dy = (self.center.1 - other.center.1).abs();
        let vv = self.size.1 + other.size.1;
        if 2.0 * dy <= vv && 2.0 * dx <= hh {
            return true;
        }
        return false;
    }

    pub fn draw(&self, y: f32, fov_distance: f32) {
        let x_left = self.center.0 - 0.50 * self.size.0;
        let x_righ = self.center.0 + 0.50 * self.size.0;
        let z_near = self.center.1 - 0.50 * self.size.1;
        let z_far = self.center.1 + 0.50 * self.size.1;

        let p1 = vector3d::Vector3d::new(x_left, y, z_far);
        let p2 = vector3d::Vector3d::new(x_righ, y, z_far);
        let p3 = vector3d::Vector3d::new(x_righ, y, z_near);
        let p4 = vector3d::Vector3d::new(x_left, y, z_near);

        engine::draw_line(
            projection::to_screen(p1, fov_distance),
            projection::to_screen(p2, fov_distance),
        );
        engine::draw_line(
            projection::to_screen(p2, fov_distance),
            projection::to_screen(p3, fov_distance),
        );
        engine::draw_line(
            projection::to_screen(p3, fov_distance),
            projection::to_screen(p4, fov_distance),
        );
        engine::draw_line(
            projection::to_screen(p4, fov_distance),
            projection::to_screen(p1, fov_distance),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collision_true() {
        let rec1 = Rectangle::new((0.0, 0.0), (1.0, 1.0));
        let rec2 = Rectangle::new((1.0, 0.0), (1.0, 1.0));
        assert_eq! { rec1.collision(&rec2), true }
    }

    #[test]
    fn collision_false() {
        let rec1 = Rectangle::new((0.0, 0.0), (1.0, 1.0));
        let rec2 = Rectangle::new((3.0, 0.0), (1.0, 1.0));
        assert_eq! { rec1.collision(&rec2), false }
    }

    #[test]
    fn collision_true_y() {
        let rec1 = Rectangle::new((0.0, 0.0), (1.0, 1.0));
        let rec2 = Rectangle::new((0.0, 1.0), (1.0, 1.0));
        assert_eq! { rec1.collision(&rec2), true }
    }

    #[test]
    fn collision_false_y() {
        let rec1 = Rectangle::new((0.0, 0.0), (1.0, 1.0));
        let rec2 = Rectangle::new((0.0, 3.0), (1.0, 1.0));
        assert_eq! { rec1.collision(&rec2), false }
    }
}
