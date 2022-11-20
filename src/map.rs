use super::engine;
use super::projection;

pub struct Map {
    camera_height: f32,
    z_max: f32,
    fov_distance: f32,
    map_width: f32,
}

impl Map {
    pub fn new(camera_height: f32, z_max: f32, horizon_delta: f32, map_width: f32) -> Map {
        let horizon_drop: f32 = horizon_delta / camera_height;
        let fov_distance = z_max / (1.0 + 1.0 / horizon_drop);
        Map {
            camera_height,
            z_max,
            fov_distance,
            map_width,
        }
    }

    pub fn draw_grid(&self) {
        let n_tiles_size_to_side = 14;
        let tile_size = self.map_width / (n_tiles_size_to_side as f32);
        let draw_distance = self.z_max;
        let camera_height = self.camera_height;

        let mut x = 0.0;
        while x <= self.map_width * 0.50 {
            self.draw_vertical_line(x, camera_height, draw_distance);
            self.draw_vertical_line(-x, camera_height, draw_distance);
            x += tile_size;
        }

        let mut z = 0.0;
        while z < draw_distance {
            self.draw_horizontal_line(z, camera_height);
            z += tile_size;
        }
    }

    fn draw_vertical_line(&self, x: f32, camera_height: f32, draw_distance: f32) {
        let bottom = vector3d::Vector3d::new(x, -camera_height, 0.0);
        let top = vector3d::Vector3d::new(x, -camera_height, draw_distance);
        engine::draw_line(
            projection::ToScreen(bottom, self.fov_distance),
            projection::ToScreen(top, self.fov_distance),
        );
    }

    fn draw_horizontal_line(&self, z: f32, camera_height: f32) {
        let x = 0.50 * self.map_width;
        let left = vector3d::Vector3d::new(x, -camera_height, z);
        let right = vector3d::Vector3d::new(-x, -camera_height, z);
        engine::draw_line(
            projection::ToScreen(left, self.fov_distance),
            projection::ToScreen(right, self.fov_distance),
        );
    }
}
