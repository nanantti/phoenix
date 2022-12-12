use super::engine;
use super::obstacle;
use super::projection;
use super::rectangle;

// TODO: use MapPoint instead of (f32, f32)
// TODO: reference to projection as struct member, instead of passing reference around

pub struct MapPoint {
    x: f32,
    z: f32,
}

impl MapPoint {
    pub fn new(x: f32, z: f32) -> MapPoint {
        MapPoint { x, z }
    }
}

pub struct Map {
    camera_height: f32,
    map_width: f32,
    map_length: f32,
    obstacles: Vec<obstacle::Obstacle>,
    tile_size: f32,
    best_distance_z: f32,
    best_time_seconds: f32,
    finish_line_z: f32,
}

impl Map {
    const TILE_SIZE_PX: f32 = 100.0;
    const FENCE_WIDTH_PX: f32 = 40.0;
    const FENCE_HEIGHT_PX: f32 = 100.0;
    const OBSTACLE_SIDE_MIN_PX: f32 = 50.0;
    const OBSTACLE_SIDE_MAX_PX: f32 = 150.0;
    const OBSTACLE_SIDE_AVG_PX: f32 =
        (Map::OBSTACLE_SIDE_MIN_PX + Map::OBSTACLE_SIDE_MAX_PX) * 0.50;
    const AVERAGE_OBSTACLE_AREA: f32 = Map::OBSTACLE_SIDE_AVG_PX * Map::OBSTACLE_SIDE_AVG_PX;

    pub fn new(camera_height: f32, map_width: f32, map_length: f32) -> Map {
        let tile_size = Map::TILE_SIZE_PX;
        let mut map = Map {
            camera_height,
            map_width,
            map_length,
            obstacles: Vec::new(),
            tile_size,
            best_distance_z: -1.0,
            best_time_seconds: -1.0,
            finish_line_z: map_length,
        };
        map.add_fences();
        //map.add_endgoal();
        map.roll_map(0.10);
        map
    }

    pub fn check_game_over(&self, player_shape: &rectangle::Rectangle) -> bool {
        self.check_collision(player_shape) || self.check_game_win(player_shape)
    }

    pub fn check_game_win(&self, player_shape: &rectangle::Rectangle) -> bool {
        player_shape.get_center().1 >= self.finish_line_z
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

    fn check_collision(&self, player_shape: &rectangle::Rectangle) -> bool {
        for obstacle in &self.obstacles {
            if obstacle.check_collision(player_shape) {
                return true;
            }
        }
        false
    }

    pub fn check_collision_obst(&self, other: &obstacle::Obstacle) -> bool {
        for obstacle in &self.obstacles {
            if obstacle.check_collision_obst(other) {
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
        self.draw_best_distance_line(projection);
    }

    pub fn draw_grid(&self, projection: &projection::Projection) {
        self.draw_horizontal_grid_lines(self.tile_size, projection);
    }

    fn draw_horizontal_grid_lines(&self, tile_size: f32, projection: &projection::Projection) {
        let range = projection.get_view_zone_z_range();
        let z_offset = self.z_grid_offset(tile_size, projection);
        let mut z = range.0;
        while z < range.1 {
            if z <= self.map_length {
                self.draw_horizontal_line(z - z_offset, projection, engine::GRID_LINE);
            }
            z += tile_size;
        }
    }

    fn z_grid_offset(&self, tile_size: f32, projection: &projection::Projection) -> f32 {
        let viewport_anchor = projection.get_view_zone_z_range().0;
        ((viewport_anchor as i32) % (tile_size as i32)) as f32
    }

    fn draw_horizontal_line(
        &self,
        z: f32,
        projection: &projection::Projection,
        draw_params: engine::DrawParameters,
    ) {
        let x = 0.50 * self.map_width;
        let left = projection::Point3D::new(x, -self.camera_height, z);
        let right = projection::Point3D::new(-x, -self.camera_height, z);
        engine::draw_line_personalized(
            projection.to_screen(&left),
            projection.to_screen(&right),
            draw_params,
        );
    }

    // Roll obstacles.
    fn roll_map(&mut self, area_ratio: f32) {
        for _ in 0..self.get_n_tries(area_ratio) {
            let random_obstacle = self.roll_random_obstacle();
            if !self.check_collision_obst(&random_obstacle) {
                self.add_obstacle(random_obstacle);
            }
        }
    }

    fn get_n_tries(&self, area_ratio: f32) -> i32 {
        (self.map_area() * area_ratio / Map::AVERAGE_OBSTACLE_AREA).round() as i32
    }

    fn map_area(&self) -> f32 {
        self.map_width * self.map_length
    }

    fn roll_random_obstacle(&self) -> obstacle::Obstacle {
        let height: f32 = self.random_height();
        let size: (f32, f32) = Map::random_size();
        let center: (f32, f32) = self.random_map_location(size);
        obstacle::Obstacle::new(center, size, height)
    }

    fn random_map_location(&self, size: (f32, f32)) -> (f32, f32) {
        let x_max: f32 = (self.map_width - size.0) * 0.50;
        let x_min: f32 = -x_max;
        let z_max: f32 = self.map_length - size.1 * 0.50;
        let z_min: f32 = size.1 * 5.00;
        let x: f32 = engine::gen_range(x_min, x_max);
        let z: f32 = engine::gen_range(z_min, z_max);
        (x, z)
    }

    fn random_height(&self) -> f32 {
        let h_min = 100.0;
        let h_max = self.camera_height * 2.0;
        engine::gen_range(h_min, h_max)
    }

    fn random_size() -> (f32, f32) {
        let x: f32 = engine::gen_range(Map::OBSTACLE_SIDE_MIN_PX, Map::OBSTACLE_SIDE_MAX_PX);
        let z: f32 = engine::gen_range(Map::OBSTACLE_SIDE_MIN_PX, Map::OBSTACLE_SIDE_MAX_PX);
        (x, z)
    }

    // Draw holographic hud
    pub fn log_endrun_distance(&mut self, best_dist: f32) {
        self.best_distance_z = self.best_distance_z.max(best_dist);
    }

    pub fn log_endrun_time(&mut self, time_interval: f64) {
        if self.best_time_seconds < 0.0 {
            self.best_time_seconds = time_interval as f32;
        } else {
            self.best_time_seconds = (time_interval as f32).min(self.best_time_seconds);
        }
    }

    fn draw_best_distance_line(&self, projection: &projection::Projection) {
        let line_location = (0.0, self.best_distance_z);
        let pole_location = MapPoint::new(self.map_width * 0.50, self.best_distance_z);
        if projection.is_point_in_view_zone(&line_location) {
            self.draw_horizontal_line(self.best_distance_z, projection, engine::HUD_LINE);
            self.draw_pole(projection, &pole_location);
        }
    }

    fn draw_pole(&self, projection: &projection::Projection, pole_location: &MapPoint) {
        let pole_height: f32 = 200.0;
        let bot = self.to_3d(pole_location, 0.0);
        let top = self.to_3d(pole_location, pole_height);
        engine::draw_line_personalized(
            projection.to_screen(&bot),
            projection.to_screen(&top),
            engine::HUD_LINE,
        );
        self.draw_best_line_text(projection, &top);
    }

    fn draw_best_line_text(
        &self,
        projection: &projection::Projection,
        anchor: &projection::Point3D,
    ) {
        let message: &str = &self.get_best_line_message();
        engine::draw_text(message, projection.to_screen(anchor), engine::TEXT_DEFAULT);
    }

    fn get_best_line_message(&self) -> String {
        if self.best_time_seconds > 0.0 {
            format! {"best time: {:.prec$}", self.best_time_seconds, prec = 3}
        } else {
            format! {"best distance: {:.prec$}", self.best_distance_z, prec = 0}
        }
    }

    fn to_3d(&self, point: &MapPoint, height: f32) -> projection::Point3D {
        projection::Point3D {
            x: point.x,
            y: height - self.camera_height,
            z: point.z,
        }
    }
}
