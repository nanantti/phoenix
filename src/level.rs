use super::engine;
use super::map;
use super::player;
use super::projection;

pub struct Level {
    game_map: map::Map,
    phoenix: player::Player,
    projection: projection::Projection,
    last_reset_timeframe: f64,
    camera_height: f32,
}

impl Level {
    pub fn new(camera_height: f32, map_width: f32, map_length: f32) -> Level {
        Level {
            game_map: map::Map::new(camera_height, map_width, map_length),
            phoenix: player::Player::new(-camera_height),
            projection: projection::Projection::new(camera_height),
            last_reset_timeframe: 0.0,
            camera_height,
        }
    }
    pub fn update(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {
        self.phoenix
            .update(current_time - self.last_reset_timeframe, active_keys);
        let player_pos = self.phoenix.get_position();
        self.projection.set_offset(player_pos.0, player_pos.1);
        let player_speed = self.phoenix.get_speed_pu();
        self.projection.set_fov(player_speed);
    }
    pub fn draw(&self) {
        self.game_map.draw(&self.projection);
        self.phoenix.draw(&self.projection);
    }
    pub fn check_game_over(&self) -> bool {
        return self.game_map.check_game_over(self.phoenix.get_shape());
    }
    pub fn reset(&mut self, time: f64) {
        let delta_t = time - self.last_reset_timeframe;
        self.game_map.reset_run(delta_t, self.phoenix.get_shape());
        self.phoenix = player::Player::new(-self.camera_height);
        self.projection = projection::Projection::new(self.camera_height);
        self.last_reset_timeframe = time;
    }
}
