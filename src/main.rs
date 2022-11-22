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
}

impl Game {
    pub fn new(camera_height: f32, map_width: f32) -> Game {
        let horizon_delta = 10.0;
        let z_max = 4000.0;
        Game {
            game_map: map::Map::new(camera_height, z_max, map_width),
            phoenix: player::Player::new(-camera_height),
            projection: projection::Projection::new(camera_height, z_max, horizon_delta),
        }
    }
    pub fn update(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {
        self.phoenix.update(current_time, active_keys);
        let player_pos = self.phoenix.get_position();
        self.projection.set_offset(player_pos.0, player_pos.1);
    }
    pub fn draw(&self) {
        self.game_map.draw(&self.projection);
        self.phoenix.draw(&self.projection);
    }
    pub fn check_game_over(&self) -> bool {
        return false;
    }
    pub fn reset(&self) {}
}

#[macroquad::main("Phoenix")]
async fn main() {
    let mut game = Game::new(
        0.50 * engine::get_screen_height(),
        engine::get_screen_width(),
    );
    loop {
        engine::clear_background();
        //game.update_screen_size(engine::get_screen_size());
        game.update(engine::get_time(), &engine::get_active_move_keys());
        game.draw();
        if game.check_game_over() {
            game.reset();
        }
        engine::await_next_frame().await
    }
}
