pub struct PointScreen {
    pub x: f32,
    pub y: f32,
}

// Graphics
pub const BACKGROUND_COLOR: macroquad::prelude::Color = macroquad::prelude::BLACK;

#[derive(Copy, Clone)]
pub struct DrawParameters {
    color: macroquad::prelude::Color,
    line_width: f32,
}

pub struct TextParameters {
    color: macroquad::prelude::Color,
    font_size: f32,
}

pub const DEFAULT_LINE: DrawParameters = DrawParameters {
    color: macroquad::prelude::WHITE,
    line_width: 1.0,
};

pub const PHOENIX_LINE: DrawParameters = DrawParameters {
    color: macroquad::prelude::RED,
    line_width: 2.0,
};

pub const GRID_LINE: DrawParameters = DrawParameters {
    color: macroquad::prelude::GRAY,
    line_width: 0.25,
};

pub const HUD_LINE: DrawParameters = DrawParameters {
    color: macroquad::prelude::YELLOW,
    line_width: 2.0,
};

pub const TEXT_DEFAULT: TextParameters = TextParameters {
    color: macroquad::prelude::YELLOW,
    font_size: 20.0,
};

pub const TEXT_TITLE: TextParameters = TextParameters {
    color: macroquad::prelude::RED,
    font_size: 80.0,
};

pub const TEXT_INSTRUCTIONS: TextParameters = TextParameters {
    color: macroquad::prelude::WHITE,
    font_size: 20.0,
};

pub fn clear_background() {
    macroquad::prelude::clear_background(BACKGROUND_COLOR);
}

pub fn draw_line(p1: PointScreen, p2: PointScreen) {
    draw_line_personalized(p1, p2, DEFAULT_LINE);
}

pub fn draw_line_personalized(p1: PointScreen, p2: PointScreen, draw_params: DrawParameters) {
    let pt1 = transform(p1);
    let pt2 = transform(p2);
    macroquad::shapes::draw_line(
        pt1.x,
        pt1.y,
        pt2.x,
        pt2.y,
        draw_params.line_width,
        draw_params.color,
    );
}

pub fn transform(p: PointScreen) -> PointScreen {
    PointScreen {
        x: p.x + get_screen_width() * 0.50,
        y: -p.y + get_screen_height() * 0.50,
    }
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
//pub fn set_rand_seed(seed: u64) {
//    macroquad::rand::srand(seed);
//}

pub fn gen_range<T: macroquad::rand::RandomRange>(low: T, high: T) -> T {
    macroquad::rand::gen_range::<T>(low, high)
}

// text
pub fn draw_text(message: &str, p: PointScreen, params: TextParameters) {
    let pt = transform(p);
    macroquad::prelude::draw_text(message, pt.x, pt.y, params.font_size, params.color);
}
