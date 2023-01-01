use super::engine;

const TITLE_ANCHOR: engine::PointScreen = engine::PointScreen {
    x: -120.0,
    y: 200.0,
};

pub struct StartMenu {
    level_start: bool,
}

impl StartMenu {
    pub fn new() -> StartMenu {
        StartMenu { level_start: false }
    }
    pub fn draw(&self) {
        engine::draw_text("Phoenix", TITLE_ANCHOR, engine::TEXT_TITLE);
    }
    pub fn update(&mut self, active_keys: &engine::MoveKeys) {
        if active_keys.up {
            self.level_start = true;
        }
    }
    pub fn request_level_start(&self) -> bool {
        self.level_start
    }
}
