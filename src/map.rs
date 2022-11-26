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
        self.draw_vertical_grid_lines(tile_size, projection);
        self.draw_horizontal_grid_lines(tile_size, projection);
    }

    fn draw_vertical_grid_lines(&self, tile_size: f32, projection: &projection::Projection) {
        let mut x = 0.0;
        while x < self.map_width * 0.50 {
            self.draw_vertical_line(x, projection);
            self.draw_vertical_line(-x, projection);
            x += tile_size;
        }
    }

    fn draw_horizontal_grid_lines(&self, tile_size: f32, projection: &projection::Projection) {
        let range = projection.get_view_zone_z_range();
        let z_offset = self.z_grid_offset(tile_size, projection);
        let mut z = range.0;
        while z < range.1 {
            self.draw_horizontal_line(z - z_offset, projection);
            z += tile_size;
        }
    }

    fn z_grid_offset(&self, tile_size: f32, projection: &projection::Projection) -> f32 {
        let viewport_anchor = projection.get_view_zone_z_range().0;
        let anchor_grid_displacement = ((viewport_anchor as i32) % (tile_size as i32)) as f32;
        anchor_grid_displacement
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
