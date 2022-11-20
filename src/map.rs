use super::engine;
use super::projection;

pub struct Map {
    camera_height: f32,
    z_max: f32,
    map_width: f32,
}

impl Map {
    pub fn new(camera_height: f32, z_max: f32, map_width: f32) -> Map {
        Map {
            camera_height,
            z_max,
            map_width,
        }
    }

    pub fn draw_grid(&self, fov_distance: f32) {
        let n_tiles_size_to_side = 14;
        let tile_size = self.map_width / (n_tiles_size_to_side as f32);
        let draw_distance = self.z_max;

        let mut x = 0.0;
        while x <= self.map_width * 0.50 {
            self.draw_vertical_line(x, fov_distance);
            self.draw_vertical_line(-x, fov_distance);
            x += tile_size;
        }

        let mut z = 0.0;
        while z < draw_distance {
            self.draw_horizontal_line(z, fov_distance);
            z += tile_size;
        }
    }

    fn draw_vertical_line(&self, x: f32, fov_distance: f32) {
        let bottom = vector3d::Vector3d::new(x, -self.camera_height, 0.0);
        let top = vector3d::Vector3d::new(x, -self.camera_height, self.z_max);
        engine::draw_line(
            projection::to_screen(bottom, fov_distance),
            projection::to_screen(top, fov_distance),
        );
    }

    fn draw_horizontal_line(&self, z: f32, fov_distance: f32) {
        let x = 0.50 * self.map_width;
        let left = vector3d::Vector3d::new(x, -self.camera_height, z);
        let right = vector3d::Vector3d::new(-x, -self.camera_height, z);
        engine::draw_line(
            projection::to_screen(left, fov_distance),
            projection::to_screen(right, fov_distance),
        );
    }
}
