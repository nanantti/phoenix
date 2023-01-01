mod engine;
mod level;
mod map;
mod obstacle;
mod player;
mod projection;
mod rectangle;

const PLAYER_WIDTH: f32 = 25.0;

fn init_level() -> level::Level {
    let camera_drop: f32 = 0.350 * engine::get_screen_height();
    let map_width: f32 = 2.0 * engine::get_screen_width();
    let map_length: f32 = 10.0 * engine::get_screen_width();
    level::Level::new(camera_drop, map_width, map_length)
}

#[macroquad::main("Phoenix")]
async fn main() {
    let mut game = init_level();
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
