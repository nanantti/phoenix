mod engine;
mod level;
mod map;
mod obstacle;
mod player;
mod projection;
mod rectangle;
mod startmenu;

const PLAYER_WIDTH: f32 = 25.0;

pub enum GameMode {
    StartMenu,
    Level,
}

struct Game {
    mode: GameMode,
    level: level::Level,
    menu: startmenu::StartMenu,
}

impl Game {
    pub fn new() -> Game {
        Game {
            mode: GameMode::StartMenu,
            level: init_level(),
            menu: startmenu::StartMenu::new(),
        }
    }

    pub fn run(&mut self) {
        match self.mode {
            GameMode::StartMenu => self.run_start_menu(),
            GameMode::Level => self.run_game(),
        }
    }

    fn run_start_menu(&mut self) {
        self.menu.update(&engine::get_active_move_keys());
        self.menu.draw();
        if self.menu.request_level_start() {
            self.mode = GameMode::Level;
        }
    }

    fn run_game(&mut self) {
        self.level
            .update(engine::get_time(), &engine::get_active_move_keys());
        self.level.draw();
        if self.level.check_game_over() {
            self.level.reset(engine::get_time());
        }
    }
}

fn init_level() -> level::Level {
    let camera_drop: f32 = 0.350 * engine::get_screen_height();
    let map_width: f32 = 2.0 * engine::get_screen_width();
    let map_length: f32 = 10.0 * engine::get_screen_width();
    level::Level::new(camera_drop, map_width, map_length)
}

#[macroquad::main("Phoenix")]
async fn main() {
    let mut game = Game::new();
    loop {
        engine::clear_background();
        game.run();
        engine::await_next_frame().await
    }
}
