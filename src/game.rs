use crate::engine;
use crate::level;
use crate::startmenu;

const FRAME_UPDATE_SECONDS: f64 = 1.0 / 50.0;
const GAMEOVER_TIME_SECONDS: f64 = 1.0;

#[derive(PartialEq, Debug)]
pub enum GameMode {
    StartMenu,
    StartToLevel,
    Level,
    ToGameOver,
    GameOver,
}

pub struct Game {
    mode: GameMode,
    level: level::Level,
    menu: startmenu::StartMenu,
    last_update_time: f64,
    gameover_timestamp: f64,
}

impl Game {
    pub fn new(camera_drop: f32, map_width: f32, map_length: f32, time: f64) -> Game {
        Game {
            mode: GameMode::StartMenu,
            level: level::Level::new(camera_drop, map_width, map_length, time),
            menu: startmenu::StartMenu::new(),
            last_update_time: time,
            gameover_timestamp: 0.0,
        }
    }

    pub fn run(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {
        if self.skip_frame(current_time) {
            return;
        }
        match self.mode {
            GameMode::StartMenu => self.run_start_menu(active_keys),
            GameMode::StartToLevel => self.init_level(current_time),
            GameMode::Level => self.run_game(current_time, active_keys),
            GameMode::ToGameOver => self.init_gameover(current_time),
            GameMode::GameOver => self.run_gameover(current_time),
        }
    }

    pub fn draw(&self) {
        match self.mode {
            GameMode::StartMenu => {
                self.menu.draw();
                self.level.draw()
            }
            GameMode::StartToLevel => self.level.draw(),
            GameMode::Level => self.level.draw(),
            GameMode::ToGameOver => self.level.draw(),
            GameMode::GameOver => self.level.draw(),
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
            self.mode = GameMode::StartToLevel;
        }
    }

    fn init_level(&mut self, current_time: f64) {
        self.level.reset(current_time);
        self.mode = GameMode::Level;
    }

    fn run_game(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {
        self.level.update(current_time, active_keys);
        if self.level.check_game_over() {
            self.mode = GameMode::ToGameOver;
        }
    }

    fn init_gameover(&mut self, current_time: f64) {
        self.gameover_timestamp = current_time;
        self.mode = GameMode::GameOver;
    }

    fn run_gameover(&mut self, current_time: f64) {
        if current_time - self.gameover_timestamp > GAMEOVER_TIME_SECONDS {
            self.mode = GameMode::Level;
            self.level.reset(current_time);
        }
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

        assert_eq! { game.level.phoenix.get_position().0, 0.0}
        assert_eq! { game.level.phoenix.get_position().1, 25.0}

        assert_eq! { game.level.projection.offset_x, 0.0}
        assert_eq! { game.level.projection.offset_z, 25.0}
    }

    #[test]
    fn init_game_start() {
        let mut game = Game::new(CAMERA_DROP, MAP_WIDTH, MAP_LENGTH, INIT_TIME);
        game.run(0.1, &UP_PRESS);

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
