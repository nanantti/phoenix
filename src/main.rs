mod engine;
mod map;
mod obstacle;
mod player;
mod projection;
mod rectangle;

pub struct Game {
    game_map: map::Map,
    phoenix: player::Player,
    projection: projection::Projection,
    last_reset_timeframe: f64,
}

impl Game {
    pub fn new(camera_height: f32, map_width: f32) -> Game {
        let horizon_delta = 10.0;
        let z_max = 4000.0;
        Game {
            game_map: map::Map::new(camera_height, map_width),
            phoenix: player::Player::new(-camera_height),
            projection: projection::Projection::new(camera_height, z_max, horizon_delta),
            last_reset_timeframe: 0.0,
        }
    }
    pub fn update(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {
        self.phoenix
            .update(current_time - self.last_reset_timeframe, active_keys);
        let player_pos = self.phoenix.get_position();
        self.projection.set_offset(player_pos.0, player_pos.1);
    }
    pub fn draw(&self) {
        self.game_map.draw(&self.projection);
        self.phoenix.draw(&self.projection);
    }
    pub fn check_game_over(&self) -> bool {
        return self.game_map.check_collision(self.phoenix.get_shape());
    }
    pub fn add_obstacle(&mut self, obstacle: obstacle::Obstacle) {
        self.game_map.add_obstacle(obstacle);
    }
    pub fn reset(&mut self, time: f64) {
        self.last_reset_timeframe = time;
    }
}

fn foobar() -> Game {
    let mut game = Game::new(
        0.50 * engine::get_screen_height(),
        engine::get_screen_width(),
    );
    game.add_obstacle(obstacle::Obstacle::new(
        (200.0, 200.0),
        (100.0, 100.0),
        400.0,
    ));
    game.add_obstacle(obstacle::Obstacle::new(
        (-200.0, 600.0),
        (100.0, 100.0),
        200.0,
    ));
    game.reset(engine::get_time());
    game
}

#[macroquad::main("Phoenix")]
async fn main() {
    let mut game = foobar();
    loop {
        engine::clear_background();
        //game.update_screen_size(engine::get_screen_size());
        game.update(engine::get_time(), &engine::get_active_move_keys());
        game.draw();
        if game.check_game_over() {
            game = foobar();
        }
        engine::await_next_frame().await
    }
}
