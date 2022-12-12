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

    pub fn get_corners(&self, y: f32) -> [projection::Point3D; 4] {
        let x_left = self.center.0 - 0.50 * self.size.0;
        let x_righ = self.center.0 + 0.50 * self.size.0;
        let z_near = self.center.1 - 0.50 * self.size.1;
        let z_far = self.center.1 + 0.50 * self.size.1;

        let p1 = projection::Point3D::new(x_left, y, z_far);
        let p2 = projection::Point3D::new(x_righ, y, z_far);
        let p3 = projection::Point3D::new(x_righ, y, z_near);
        let p4 = projection::Point3D::new(x_left, y, z_near);

        return [p1, p2, p3, p4];
    }

    fn get_corners_xz(&self) -> [(f32, f32); 4] {
        let corners = self.get_corners(0.0);
        let p1 = (corners[0].x, corners[0].z);
        let p2 = (corners[1].x, corners[1].z);
        let p3 = (corners[2].x, corners[2].z);
        let p4 = (corners[3].x, corners[3].z);
        return [p1, p2, p3, p4];
    }

    pub fn draw(&self, y: f32, projection: &projection::Projection) {
        let corners = self.get_corners(y);

        engine::draw_line(
            projection.to_screen(&corners[0]),
            projection.to_screen(&corners[1]),
        );
        engine::draw_line(
            projection.to_screen(&corners[1]),
            projection.to_screen(&corners[2]),
        );
        engine::draw_line(
            projection.to_screen(&corners[2]),
            projection.to_screen(&corners[3]),
        );
        engine::draw_line(
            projection.to_screen(&corners[3]),
            projection.to_screen(&corners[0]),
        );
    }

    pub fn get_center(&self) -> (f32, f32) {
        self.center
    }

    pub fn move_x(&mut self, delta_x: f32) {
        self.center.0 += delta_x;
    }

    pub fn move_y(&mut self, delta_y: f32) {
        self.center.1 += delta_y;
    }

    pub fn is_rectangle_in_view_range(&self, projection: &projection::Projection) -> bool {
        let mut in_view: bool = false;
        for corner in self.get_corners_xz() {
            if projection.is_point_in_view_zone(&corner) {
                in_view = true;
            }
        }
        in_view
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

    #[test]
    fn move_center_x() {
        let mut rec1 = Rectangle::new((0.0, 0.0), (1.0, 1.0));
        rec1.move_x(10.0);
        assert_eq! { rec1.get_center(), (10.0, 0.0) }
    }

    #[test]
    fn move_center_x_neg() {
        let mut rec1 = Rectangle::new((0.0, 0.0), (1.0, 1.0));
        assert_eq! { rec1.get_center(), (0.0, 0.0) }
        rec1.move_x(-20.0);
        assert_eq! { rec1.get_center(), (-20.0, 0.0) }
    }

    #[test]
    fn move_center_y() {
        let mut rec1 = Rectangle::new((0.0, 0.0), (1.0, 1.0));
        rec1.move_y(10.0);
        assert_eq! { rec1.get_center(), (0.0, 10.0) }
    }

    #[test]
    fn move_center_y_neg() {
        let mut rec1 = Rectangle::new((0.0, 0.0), (1.0, 1.0));
        assert_eq! { rec1.get_center(), (0.0, 0.0) }
        rec1.move_y(-20.0);
        assert_eq! { rec1.get_center(), (0.0, -20.0) }
    }
}
