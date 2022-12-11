// Graphics
const LINE_WIDTH: f32 = 1.0;
pub const BACKGROUND_COLOR: macroquad::prelude::Color = macroquad::prelude::BLACK;
pub const BACK_COLOR: macroquad::color::Color = macroquad::color::Color {
    //#4F5458
    r: 79.0,
    g: 84.0,
    b: 88.0,
    a: 1.0,
};

pub enum MaterialColor {
    Map,
    Hud,
}

fn get_color(color: MaterialColor) -> macroquad::color::Color {
    match color {
        MaterialColor::Map => macroquad::prelude::WHITE,
        MaterialColor::Hud => macroquad::prelude::YELLOW,
    }
}

pub fn clear_background() {
    macroquad::prelude::clear_background(BACKGROUND_COLOR);
}

pub fn draw_line(p1: vector2d::Vector2D<f32>, p2: vector2d::Vector2D<f32>) {
    draw_line_width_color(p1, p2, LINE_WIDTH, MaterialColor::Map);
}

pub fn draw_line_width_color(
    p1: vector2d::Vector2D<f32>,
    p2: vector2d::Vector2D<f32>,
    line_width: f32,
    material_color: MaterialColor,
) {
    let color = get_color(material_color);
    let pt1 = transform(p1);
    let pt2 = transform(p2);
    macroquad::shapes::draw_line(pt1.x, pt1.y, pt2.x, pt2.y, line_width, color);
}

pub fn transform(p: vector2d::Vector2D<f32>) -> vector2d::Vector2D<f32> {
    vector2d::Vector2D {
        x: p.x + get_screen_width() * 0.50,
        y: -p.y + get_screen_height() * 0.50,
    }
}

pub fn get_screen_size() -> (f32, f32) {
    (get_screen_width(), get_screen_height())
}

pub fn get_screen_height() -> f32 {
    macroquad::prelude::screen_height()
}

pub fn get_screen_width() -> f32 {
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
