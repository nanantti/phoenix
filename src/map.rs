use super::engine;
use super::obstacle;
use super::projection;

pub struct Map {
    camera_height: f32,
    z_max: f32,
    map_width: f32,
    obstacle: obstacle::Obstacle,
}

impl Map {
    pub fn new(camera_height: f32, z_max: f32, map_width: f32) -> Map {
        Map {
            camera_height,
            z_max,
            map_width,
            obstacle: obstacle::Obstacle::new((200.0, 200.0), (100.0, 100.0), 400.0),
        }
    }

    pub fn draw(&self, projection: &projection::Projection) {
        self.draw_grid(projection);
        self.obstacle.draw(projection, -self.camera_height);
    }

    pub fn draw_grid(&self, projection: &projection::Projection) {
        let n_tiles_size_to_side = 8;
        let tile_size = self.map_width / (n_tiles_size_to_side as f32);
        let draw_distance = self.z_max;

        let mut x = 0.0;
        while x <= self.map_width * 0.50 {
            self.draw_vertical_line(x, projection);
            self.draw_vertical_line(-x, projection);
            x += tile_size;
        }

        let mut z = 0.0;
        while z < draw_distance {
            self.draw_horizontal_line(z, projection);
            z += tile_size;
        }
    }

    fn draw_vertical_line(&self, x: f32, projection: &projection::Projection) {
        let bottom = vector3d::Vector3d::new(x, -self.camera_height, 0.0);
        let top = vector3d::Vector3d::new(x, -self.camera_height, self.z_max);
        engine::draw_line(projection.to_screen(bottom), projection.to_screen(top));
    }

    fn draw_horizontal_line(&self, z: f32, projection: &projection::Projection) {
        let x = 0.50 * self.map_width;
        let left = vector3d::Vector3d::new(x, -self.camera_height, z);
        let right = vector3d::Vector3d::new(-x, -self.camera_height, z);
        engine::draw_line(projection.to_screen(left), projection.to_screen(right));
    }
}
