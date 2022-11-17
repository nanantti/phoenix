// Graphics
pub const BACKGROUND_COLOR: macroquad::prelude::Color = macroquad::prelude::BLACK;
pub const LINE_COLOR: macroquad::prelude::Color = macroquad::prelude::GREEN;

pub fn clear_background() {
    macroquad::prelude::clear_background(BACKGROUND_COLOR);
}

pub fn get_screen_size() -> (f32, f32) {
    (get_screen_width(), get_screen_height())
}

fn get_screen_height() -> f32 {
    macroquad::prelude::screen_height()
}

fn get_screen_width() -> f32 {
    macroquad::prelude::screen_width()
}

// Time
pub async fn await_next_frame() {
    macroquad::prelude::next_frame().await
}

pub fn get_time() -> f64 {
    macroquad::prelude::get_time()
}

// Input
pub struct MoveKeys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

pub fn get_active_move_keys() -> MoveKeys {
    MoveKeys {
        up: macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::W),
        down: macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::S),
        left: macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::A),
        right: macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::D),
    }
}

// Random
pub fn set_rand_seed(seed: u64) {
    macroquad::rand::srand(seed);
}

pub fn gen_range<T: macroquad::rand::RandomRange>(low: T, high: T) -> T {
    macroquad::rand::gen_range::<T>(low, high)
}
