use super::engine;
use super::obstacle;
use super::projection;

pub struct Map {
    camera_height: f32,
    map_width: f32,
    obstacle: obstacle::Obstacle,
}

impl Map {
    const FENCE_WIDTH_PX: f32 = 40.0;
    pub fn new(camera_height: f32, map_width: f32) -> Map {
        Map {
            camera_height,
            map_width,
            obstacle: obstacle::Obstacle::new((200.0, 200.0), (100.0, 100.0), 400.0),
        }
    }

    pub fn draw(&self, projection: &projection::Projection) {
        self.draw_grid(projection);
        self.obstacle.draw(projection, -self.camera_height);
        self.draw_map_fence(projection);
    }

    pub fn draw_grid(&self, projection: &projection::Projection) {
        let n_tiles_size_to_side = 8;
        let tile_size = self.map_width / (n_tiles_size_to_side as f32);
    }

    fn draw_vertical_line(&self, x: f32, projection: &projection::Projection) {
        let z_limits = projection.get_view_zone_z_range();
        let bottom = vector3d::Vector3d::new(x, -self.camera_height, z_limits.0);
        let top = vector3d::Vector3d::new(x, -self.camera_height, z_limits.1);
        engine::draw_line(projection.to_screen(bottom), projection.to_screen(top));
    }

    fn draw_horizontal_line(&self, z: f32, projection: &projection::Projection) {
        let x = 0.50 * self.map_width;
        let left = vector3d::Vector3d::new(x, -self.camera_height, z);
        let right = vector3d::Vector3d::new(-x, -self.camera_height, z);
        engine::draw_line(projection.to_screen(left), projection.to_screen(right));
    }

    fn draw_map_fence(&self, projection: &projection::Projection) {
        let x_limit = 0.50 * self.map_width;
        let x_limit_outer = x_limit + Map::FENCE_WIDTH_PX;
        self.draw_vertical_line(x_limit, projection);
        self.draw_vertical_line(-x_limit, projection);
        self.draw_vertical_line(x_limit_outer, projection);
        self.draw_vertical_line(-x_limit_outer, projection);
    }
}
