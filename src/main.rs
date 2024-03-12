mod engine;
mod game;
mod level;
mod map;
mod obstacle;
mod player;
mod projection;
mod rectangle;
mod startmenu;

const PLAYER_WIDTH: f32 = 25.0;

#[macroquad::main("Phoenix")]
async fn main() {
    let camera_drop: f32 = 0.350 * engine::get_screen_height();
    let map_width: f32 = 2.0 * engine::get_screen_width();
    let map_length: f32 = 10.0 * engine::get_screen_width();

    let mut game = game::Game::new(camera_drop, map_width, map_length, engine::get_time());
    loop {
        engine::clear_background();
        game.run(engine::get_time(), &engine::get_active_move_keys());
        game.draw();
        engine::await_next_frame().await
    }
}
