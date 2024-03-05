mod engine;
mod level;
mod map;
mod obstacle;
mod player;
mod projection;
mod rectangle;
mod startmenu;

const PLAYER_WIDTH: f32 = 25.0;
const FRAME_UPDATE_SECONDS: f64 = 1.0 / 50.0;

#[derive(PartialEq, Debug)]
pub enum GameMode {
    StartMenu,
    Level,
}

struct Game {
    mode: GameMode,
    level: level::Level,
    menu: startmenu::StartMenu,
    last_update_time: f64,
}

impl Game {
    pub fn new(camera_drop: f32, map_width: f32, map_length: f32, time: f64) -> Game {
        Game {
            mode: GameMode::StartMenu,
            level: level::Level::new(camera_drop, map_width, map_length, time),
            menu: startmenu::StartMenu::new(),
            last_update_time: time,
        }
    }

    pub fn run(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {
        if self.skip_frame(current_time) {
            return;
        }
        match self.mode {
            GameMode::StartMenu => self.run_start_menu(active_keys),
            GameMode::Level => self.run_game(current_time, active_keys),
        }
    }

    pub fn draw(&self) {
        match self.mode {
            GameMode::StartMenu => {
                self.menu.draw();
                self.level.draw()
            }
            GameMode::Level => self.level.draw(),
        }
    }

    fn skip_frame(&mut self, current_time: f64) -> bool {
        if current_time - self.last_update_time >= FRAME_UPDATE_SECONDS {
            return false;
        }
        true
    }

    fn run_start_menu(&mut self, active_keys: &engine::MoveKeys) {
        self.menu.update(active_keys);
        if self.menu.request_level_start() {
            self.mode = GameMode::Level;
        }
    }

    fn run_game(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {
        self.level.update(current_time, active_keys);
        if self.level.check_game_over() {
            self.level.reset(current_time);
        }
    }
}

#[macroquad::main("Phoenix")]
async fn main() {
    let camera_drop: f32 = 0.350 * engine::get_screen_height();
    let map_width: f32 = 2.0 * engine::get_screen_width();
    let map_length: f32 = 10.0 * engine::get_screen_width();

    let mut game = Game::new(camera_drop, map_width, map_length, engine::get_time());
    loop {
        engine::clear_background();
        game.run(engine::get_time(), &engine::get_active_move_keys());
        game.draw();
        engine::await_next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAMERA_DROP: f32 = 10.0;
    const MAP_LENGTH: f32 = 100.0;
    const MAP_WIDTH: f32 = 1000.0;
    const INIT_TIME: f64 = 0.0;

    const NO_PRESS: engine::MoveKeys = engine::MoveKeys {
        up: false,
        down: false,
        left: false,
        right: false,
    };

    const UP_PRESS: engine::MoveKeys = engine::MoveKeys {
        up: true,
        down: false,
        left: false,
        right: false,
    };

    #[test]
    fn init_game() {
        let game = Game::new(CAMERA_DROP, MAP_WIDTH, MAP_LENGTH, INIT_TIME);

        assert_eq! { game.mode, GameMode::StartMenu}

        assert_eq! { game.level.phoenix.get_position().0, 0.0}
        assert_eq! { game.level.phoenix.get_position().1, 25.0}

        assert_eq! { game.level.projection.offset_x, 0.0}
        assert_eq! { game.level.projection.offset_z, 25.0}
    }

    #[test]
    fn init_game_start() {
        let mut game = Game::new(CAMERA_DROP, MAP_WIDTH, MAP_LENGTH, INIT_TIME);
        game.run(0.1, &UP_PRESS);

        assert_eq! { game.mode, GameMode::Level}

        assert_eq! { game.level.phoenix.get_position().0, 0.0}
        assert_eq! { game.level.phoenix.get_position().1, 25.0}

        assert_eq! { game.level.projection.offset_x, 0.0}
        assert_eq! { game.level.projection.offset_z, 25.0}
    }

    #[test]
    fn init_game_start_reset() {
        let mut game = Game::new(CAMERA_DROP, MAP_WIDTH, MAP_LENGTH, INIT_TIME);
        game.run(0.1, &NO_PRESS);
        game.run(0.2, &UP_PRESS);
        game.run(0.3, &UP_PRESS);
        game.run(0.4, &UP_PRESS);

        assert_eq! { game.mode, GameMode::Level}

        assert_eq! { game.level.phoenix.get_position().0, 0.0}
        assert_eq! { game.level.phoenix.get_position().1, 25.0}

        assert_eq! { game.level.projection.offset_x, 0.0}
        assert_eq! { game.level.projection.offset_z, 25.0}
    }

    #[test]
    fn skip_update_if_delta_t_is_too_short() {
        let mut game = Game::new(CAMERA_DROP, MAP_WIDTH, MAP_LENGTH, INIT_TIME);
        assert_eq! {game.skip_frame((FRAME_UPDATE_SECONDS * 0.90).into()), true}
    }

    #[test]
    fn do_not_skip_if_time_is_long_enough() {
        let mut game = Game::new(CAMERA_DROP, MAP_WIDTH, MAP_LENGTH, INIT_TIME);
        assert_eq! {game.skip_frame((FRAME_UPDATE_SECONDS * 1.10).into()), false}
    }

    #[test]
    fn skip_update_twice_if_delta_t_is_too_short() {
        let mut game = Game::new(CAMERA_DROP, MAP_WIDTH, MAP_LENGTH, INIT_TIME);
        assert_eq! {game.skip_frame((FRAME_UPDATE_SECONDS * 0.45).into()), true}
        assert_eq! {game.skip_frame((FRAME_UPDATE_SECONDS * 0.90).into()), true}
        assert_eq! {game.skip_frame((FRAME_UPDATE_SECONDS * 1.35).into()), false}
    }
}
