mod engine;
mod map;
mod obstacle;
mod player;
mod projection;
mod rectangle;

const PLAYER_WIDTH: f32 = 25.0;

pub struct Game {
    game_map: map::Map,
    phoenix: player::Player,
    projection: projection::Projection,
    last_reset_timeframe: f64,
    camera_height: f32,
    map_width: f32,
}

impl Game {
    pub fn new(camera_height: f32, map_width: f32, map_length: f32) -> Game {
        Game {
            game_map: map::Map::new(camera_height, map_width, map_length),
            phoenix: player::Player::new(-camera_height),
            projection: projection::Projection::new(camera_height),
            last_reset_timeframe: 0.0,
            camera_height,
            map_width,
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
    pub fn add_obstacle(&mut self, obstacle: obstacle::Obstacle) {
        self.game_map.add_obstacle(obstacle);
    }
    pub fn reset(&mut self, time: f64) {
        let player_z = self.phoenix.get_position().1;
        if self.game_map.check_game_win(self.phoenix.get_shape()) {
            self.game_map
                .log_endrun_time(time - self.last_reset_timeframe);
        }
        self.game_map.log_endrun_distance(player_z);
        self.phoenix = player::Player::new(-self.camera_height);
        self.projection = projection::Projection::new(self.camera_height);
        self.last_reset_timeframe = time;
    }
}

fn foobar() -> Game {
    let camera_drop: f32 = 0.50 * engine::get_screen_height();
    let map_width: f32 = 2.0 * engine::get_screen_width();
    let map_length: f32 = 2.0 * engine::get_screen_width();
    Game::new(camera_drop, map_width, map_length)
}

#[macroquad::main("Phoenix")]
async fn main() {
    let mut game = foobar();
    loop {
        engine::clear_background();
        game.update(engine::get_time(), &engine::get_active_move_keys());
        game.draw();
        if game.check_game_over() {
            game.reset(engine::get_time());
        }
        engine::await_next_frame().await
    }
}
