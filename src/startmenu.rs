use super::engine;

const TITLE_ANCHOR: engine::PointScreen = engine::PointScreen {
    x: -120.0,
    y: 210.0,
};

const INSTRUCTIONS_X: f32 = -80.0;
const INSTRUCTIONS_Y1: f32 = 180.0;
const INSTRUCTIONS_Y_STEP: f32 = -20.0;

const INSTRUCTIONS_ANCHOR: engine::PointScreen = engine::PointScreen {
    x: INSTRUCTIONS_X,
    y: INSTRUCTIONS_Y1,
};

const INSTRUCTIONS_ANCHOR2: engine::PointScreen = engine::PointScreen {
    x: INSTRUCTIONS_X,
    y: INSTRUCTIONS_Y1 + INSTRUCTIONS_Y_STEP,
};

const INSTRUCTIONS_ANCHOR3: engine::PointScreen = engine::PointScreen {
    x: INSTRUCTIONS_X,
    y: INSTRUCTIONS_Y1 + 2.0 * INSTRUCTIONS_Y_STEP,
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
        self.draw_instructions();
    }
    pub fn update(&mut self, active_keys: &engine::MoveKeys) {
        if active_keys.up {
            self.level_start = true;
        }
    }
    pub fn request_level_start(&self) -> bool {
        self.level_start
    }

    fn draw_instructions(&self) {
        engine::draw_text(
            " [W] : Accelerate",
            INSTRUCTIONS_ANCHOR,
            engine::TEXT_INSTRUCTIONS,
        );
        engine::draw_text(
            "[A|D]: Left|Right",
            INSTRUCTIONS_ANCHOR2,
            engine::TEXT_INSTRUCTIONS,
        );
        engine::draw_text(
            " [S] : Decelerate",
            INSTRUCTIONS_ANCHOR3,
            engine::TEXT_INSTRUCTIONS,
        );
    }
}
