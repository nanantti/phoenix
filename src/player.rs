use super::engine;
use super::projection;
use super::rectangle;

const PLAYER_DEPTH: f32 = 10.0;
const PLAYER_Z: f32 = 25.0;
const FLOAT_HEIGHT: f32 = 25.0;
const FRAME_UPDATE_SECONDS: f64 = 1.0 / 50.0;
const INITAL_FWD_SPEED: f32 = 400.0;
const FWD_ACELERATION: f32 = 400.0;
const MIN_SPEED: f32 = 200.0;
const MAX_SPEED: f32 = 1000.0;

pub struct Player {
    shape: rectangle::Rectangle,
    y: f32,
    last_update_time: f64,
    fwd_speed: f32,
}

impl Player {
    pub fn new(map_y: f32) -> Player {
        Player {
            shape: rectangle::Rectangle::new((0.0, PLAYER_Z), (super::PLAYER_WIDTH, PLAYER_DEPTH)),
            y: map_y + FLOAT_HEIGHT,
            last_update_time: 0.0,
            fwd_speed: INITAL_FWD_SPEED,
        }
    }

    pub fn get_speed_pu(&self) -> f32 {
        (self.fwd_speed - MIN_SPEED) / (MAX_SPEED - MIN_SPEED)
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn draw(&self, projection: &projection::Projection) {
        let compensated_projection = projection::Projection::make_compensated_projection(
            projection,
            self.shape.get_center(),
        );
        self.shape.draw(self.get_y(), &compensated_projection);
        self.shape
            .draw(self.get_y() - FLOAT_HEIGHT, &compensated_projection);
    }

    pub fn update(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {
        if self.skip_frame(current_time) {
            return;
        }
        let delta_t = self.time_since_last_update(current_time);
        self.update_fwd_speed(active_keys, delta_t);
        self.update_size_position(active_keys, delta_t);
        self.update_forward_position(delta_t);
        self.last_update_time = current_time;
    }

    fn update_fwd_speed(&mut self, active_keys: &engine::MoveKeys, delta_t: f32) {
        let delta_v = delta_t * FWD_ACELERATION;
        if active_keys.up {
            self.fwd_speed += delta_v;
        }
        if active_keys.down {
            self.fwd_speed -= delta_v;
        }
        if self.fwd_speed < MIN_SPEED {
            self.fwd_speed = MIN_SPEED;
        }
        if self.fwd_speed > MAX_SPEED {
            self.fwd_speed = MAX_SPEED;
        }
    }

    fn update_size_position(&mut self, active_keys: &engine::MoveKeys, delta_t: f32) {
        let move_dist = delta_t * self.fwd_speed;
        if active_keys.left && !active_keys.right {
            self.shape.move_x(-move_dist);
        }
        if !active_keys.left && active_keys.right {
            self.shape.move_x(move_dist);
        }
    }

    fn time_since_last_update(&self, current_time: f64) -> f32 {
        (current_time - self.last_update_time) as f32
    }

    fn update_forward_position(&mut self, delta_t: f32) {
        self.shape.move_y(self.fwd_speed * delta_t);
    }

    fn skip_frame(&mut self, current_time: f64) -> bool {
        if current_time - self.last_update_time >= FRAME_UPDATE_SECONDS {
            return false;
        }
        true
    }

    pub fn get_position(&self) -> (f32, f32) {
        self.shape.get_center()
    }

    pub fn get_shape(&self) -> &rectangle::Rectangle {
        &self.shape
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NO_PRESS: engine::MoveKeys = engine::MoveKeys {
        up: false,
        down: false,
        left: false,
        right: false,
    };

    const LEFT_PRESS: engine::MoveKeys = engine::MoveKeys {
        up: false,
        down: false,
        left: true,
        right: false,
    };

    const RIGHT_PRESS: engine::MoveKeys = engine::MoveKeys {
        up: false,
        down: false,
        left: false,
        right: true,
    };

    const LEFT_RIGHT_PRESS: engine::MoveKeys = engine::MoveKeys {
        up: false,
        down: false,
        left: true,
        right: true,
    };

    const UP_PRESS: engine::MoveKeys = engine::MoveKeys {
        up: true,
        down: false,
        left: false,
        right: false,
    };

    const NEXT_FRAME: f64 = FRAME_UPDATE_SECONDS * (1.05 as f64);

    #[test]
    fn player_floats_above_ground() {
        let map_y_coord = -400.0;
        let player = Player::new(map_y_coord);
        assert_eq! { player.get_y(), map_y_coord + FLOAT_HEIGHT}
    }

    #[test]
    fn skip_update_if_delta_t_is_too_short() {
        let mut player = Player::new(0.0);
        assert_eq! {player.skip_frame((FRAME_UPDATE_SECONDS * 0.90).into()), true}
    }

    #[test]
    fn do_not_skip_if_time_is_long_enough() {
        let mut player = Player::new(0.0);
        assert_eq! {player.skip_frame((FRAME_UPDATE_SECONDS * 1.10).into()), false}
    }

    #[test]
    fn skip_update_twice_if_delta_t_is_too_short() {
        let mut player = Player::new(0.0);
        assert_eq! {player.skip_frame((FRAME_UPDATE_SECONDS * 0.45).into()), true}
        assert_eq! {player.skip_frame((FRAME_UPDATE_SECONDS * 0.90).into()), true}
        assert_eq! {player.skip_frame((FRAME_UPDATE_SECONDS * 1.35).into()), false}
    }

    #[test]
    fn player_initial_position() {
        let player = Player::new(0.0);
        assert_eq! {player.get_position(), (0.0, PLAYER_Z)}
    }

    #[test]
    fn player_moves_left() {
        let mut player = Player::new(0.0);
        player.update(NEXT_FRAME, &LEFT_PRESS);
        let player_pos = player.get_position();
        assert! {player_pos.0 < 0.0}
    }

    #[test]
    fn player_moves_right() {
        let mut player = Player::new(0.0);
        player.update(NEXT_FRAME, &RIGHT_PRESS);
        let player_pos = player.get_position();
        assert! {player_pos.0 > 0.0}
    }

    #[test]
    fn player_does_not_move_sideways() {
        let mut player = Player::new(0.0);
        player.update(NEXT_FRAME, &LEFT_RIGHT_PRESS);
        let player_pos = player.get_position();
        assert! {player_pos.0 == 0.0}
    }

    #[test]
    fn player_moves_forward() {
        let mut player = Player::new(0.0);
        player.update(NEXT_FRAME, &UP_PRESS);
        let player_pos = player.get_position();
        assert! {player_pos.1 > PLAYER_Z}
    }

    #[test]
    fn player_moves_forward_with_inital_speed() {
        let mut player = Player::new(0.0);
        player.update(NEXT_FRAME, &NO_PRESS);
        let player_pos = player.get_position();
        let new_z = PLAYER_Z + INITAL_FWD_SPEED * (NEXT_FRAME as f32);
        assert_eq! {player_pos.1, new_z}
    }

    #[test]
    fn player_accelerates_on_up_press() {
        let mut player = Player::new(0.0);
        player.update(NEXT_FRAME, &UP_PRESS);
        let player_pos = player.get_position();
        let new_z = PLAYER_Z + INITAL_FWD_SPEED * (NEXT_FRAME as f32);
        // new_z is the position if no acceleration was present
        assert! {player_pos.1 > new_z}
    }
}
