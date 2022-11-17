mod engine;
mod projection;

pub fn draw_grid(tile_size: f32) {
    let side_max = 20;
    let depth_max = 10;
    let draw_distance = 200.0;
    let camera_height = engine::get_screen_height();
    for i in -side_max..side_max {
        let x = tile_size * (i as f32);
        let bottom = vector3d::Vector3d::new(x, -camera_height, 0.0);
        let top = vector3d::Vector3d::new(x, -camera_height, draw_distance);
        engine::draw_line(projection::ToScreen(bottom), projection::ToScreen(top));
    }
    for i in 0..depth_max as i32 {
        let x = (side_max as f32) * tile_size;
        let z = (i as f32) * tile_size;
        let left = vector3d::Vector3d::new(x, -camera_height, z);
        let right = vector3d::Vector3d::new(-x, -camera_height, z);
        engine::draw_line(projection::ToScreen(left), projection::ToScreen(right));
    }
}

pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {}
    }
    pub fn update(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {}
    pub fn draw(&self) {
        draw_grid(50.0);
    }
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
