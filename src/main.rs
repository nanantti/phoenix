mod engine;

pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {}
    }
    pub fn update(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {}
    pub fn draw(&self) {}
    pub fn check_game_over(&self) -> bool {
        return false;
    }
    pub fn reset(&self) {}
}

#[macroquad::main("Phoenix")]
async fn main() {
    let mut game = Game::new();
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
