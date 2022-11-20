mod engine;
mod map;
mod projection;

pub struct Game {
    game_map: map::Map,
}

impl Game {
    pub fn new(camera_height: f32, map_width: f32) -> Game {
        let horizon_delta = 10.0;
        let z_max = 4000.0;
        Game {
            game_map: map::Map::new(camera_height, z_max, horizon_delta, map_width),
        }
    }
    pub fn update(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {}
    pub fn draw(&self) {
        self.game_map.draw_grid();
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
