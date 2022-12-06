use super::engine;
use super::obstacle;
use super::projection;
use super::rectangle;

pub struct Map {
    camera_height: f32,
    map_width: f32,
    map_length: f32,
    obstacles: Vec<obstacle::Obstacle>,
    tile_size: f32,
}

impl Map {
    const FENCE_WIDTH_PX: f32 = 40.0;
    const FENCE_HEIGHT_PX: f32 = 100.0;
    const ENDGOAL_DEPTH_PX: f32 = 100.0;

    pub fn new(camera_height: f32, map_width: f32, map_length: f32) -> Map {
        let tile_size = super::PLAYER_WIDTH;
        let mut map = Map {
            camera_height,
            map_width,
            map_length,
            obstacles: Vec::new(),
            tile_size,
        };
        map.add_fences();
        map.add_endgoal();
        map
    }

    fn add_endgoal(&mut self) {
        self.add_obstacle(obstacle::Obstacle::new(
            (0.0, self.map_length + 0.50 * Map::ENDGOAL_DEPTH_PX),
            (
                self.map_width + 2.0 * Map::FENCE_WIDTH_PX,
                Map::ENDGOAL_DEPTH_PX,
            ),
            Map::FENCE_HEIGHT_PX,
        ));
    }

    fn add_fences(&mut self) {
        let x_fence = (self.map_width + Map::FENCE_WIDTH_PX) * 0.50;
        self.add_fence(x_fence);
        self.add_fence(-x_fence);
    }

    fn add_fence(&mut self, x: f32) {
        let mut z: f32 = self.tile_size * 0.50;
        while z < self.map_length {
            self.add_obstacle(obstacle::Obstacle::new(
                (x, z),
                (Map::FENCE_WIDTH_PX, self.tile_size),
                Map::FENCE_HEIGHT_PX,
            ));
            z += self.tile_size;
        }
    }

    pub fn add_obstacle(&mut self, obstacle: obstacle::Obstacle) {
        self.obstacles.push(obstacle);
    }

    pub fn check_collision(&self, player_shape: &rectangle::Rectangle) -> bool {
        for obstacle in &self.obstacles {
            if obstacle.check_collision(player_shape) {
                return true;
            }
        }
        false
    }

    pub fn draw(&self, projection: &projection::Projection) {
        self.draw_grid(projection);
        for obstacle in &self.obstacles {
            obstacle.draw(projection, -self.camera_height);
        }
    }

    pub fn draw_grid(&self, projection: &projection::Projection) {
        self.draw_vertical_grid_lines(self.tile_size, projection);
        self.draw_horizontal_grid_lines(self.tile_size, projection);
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
            if z <= self.map_length {
                self.draw_horizontal_line(z - z_offset, projection);
            }
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
        let top = vector3d::Vector3d::new(x, -self.camera_height, self.map_length);
        engine::draw_line(projection.to_screen(bottom), projection.to_screen(top));
    }

    fn draw_horizontal_line(&self, z: f32, projection: &projection::Projection) {
        let x = 0.50 * self.map_width;
        let left = vector3d::Vector3d::new(x, -self.camera_height, z);
        let right = vector3d::Vector3d::new(-x, -self.camera_height, z);
        engine::draw_line(projection.to_screen(left), projection.to_screen(right));
    }
}
