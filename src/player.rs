use super::engine;
use super::projection;
use super::rectangle;

const PLAYER_DEPTH: f32 = 10.0;
const PLAYER_Z: f32 = 25.0;
const FLOAT_HEIGHT: f32 = 15.0 + 0.50 * super::PLAYER_WIDTH * 0.70;
const INITAL_FWD_SPEED: f32 = 200.0;
const FWD_ACELERATION: f32 = 1000.0;
const MIN_SPEED: f32 = 200.0;
const MAX_SPEED: f32 = 2000.0;
const TILT_ANGLE_DEG: f32 = 45.0;
const TILT_ANGLE_RAD: f32 = std::f32::consts::PI * TILT_ANGLE_DEG / 180.0;

pub struct Player {
    shape: rectangle::Rectangle,
    y: f32,
    last_update_time: f64,
    fwd_speed: f32,
    roll_position: RollPosition,
}

enum RollPosition {
    Level,
    Left,
    Right,
}

impl Player {
    pub fn new(map_y: f32, time: f64) -> Player {
        Player {
            shape: rectangle::Rectangle::new((0.0, PLAYER_Z), (super::PLAYER_WIDTH, PLAYER_DEPTH)),
            y: map_y + FLOAT_HEIGHT,
            last_update_time: time,
            fwd_speed: INITAL_FWD_SPEED,
            roll_position: RollPosition::Level,
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
        //self.shape.draw(self.get_y(), &compensated_projection);
        self.draw_body(&compensated_projection);
        self.draw_shadow(&compensated_projection);
    }

    fn roll_angle(&self) -> f32 {
        match self.roll_position {
            RollPosition::Level => 0.0,
            RollPosition::Left => -TILT_ANGLE_RAD,
            RollPosition::Right => TILT_ANGLE_RAD,
        }
    }

    fn get_triangle_corners(&self) -> [projection::Point3D; 3] {
        let pos = self.get_position();
        let def = 0.50 * super::PLAYER_WIDTH;
        let ang = self.roll_angle();
        let p1 = projection::Point3D::new(pos.0, self.y, pos.1 + 0.50 * PLAYER_DEPTH);
        let p2 = projection::Point3D::new(
            pos.0 + 0.50 * super::PLAYER_WIDTH * ang.cos(),
            self.y - ang.sin() * def,
            pos.1 - 0.50 * PLAYER_DEPTH,
        );
        let p3 = projection::Point3D::new(
            pos.0 - 0.50 * super::PLAYER_WIDTH * ang.cos(),
            self.y + ang.sin() * def,
            pos.1 - 0.50 * PLAYER_DEPTH,
        );
        [p1, p2, p3]
    }

    fn draw_body(&self, projection: &projection::Projection) {
        let corners = self.get_triangle_corners();
        self.draw_triangle(corners, projection, engine::PHOENIX_LINE);
    }

    fn project_shadow(
        &self,
        mut corners: [projection::Point3D; 3],
        y_projection: f32,
    ) -> [projection::Point3D; 3] {
        corners[0].y = y_projection;
        corners[1].y = y_projection;
        corners[2].y = y_projection;
        corners
    }

    fn draw_shadow(&self, projection: &projection::Projection) {
        let corners = self.get_triangle_corners();
        let shadow_y = self.get_y() - FLOAT_HEIGHT;
        let shadow_corners = self.project_shadow(corners, shadow_y);
        self.draw_triangle(shadow_corners, projection, engine::DEFAULT_LINE);
    }

    fn draw_triangle(
        &self,
        corners: [projection::Point3D; 3],
        projection: &projection::Projection,
        line: engine::DrawParameters,
    ) {
        engine::draw_line_personalized(
            projection.to_screen(&corners[0]),
            projection.to_screen(&corners[1]),
            line,
        );
        engine::draw_line_personalized(
            projection.to_screen(&corners[1]),
            projection.to_screen(&corners[2]),
            line,
        );
        engine::draw_line_personalized(
            projection.to_screen(&corners[2]),
            projection.to_screen(&corners[0]),
            line,
        );
    }

    pub fn update(&mut self, current_time: f64, active_keys: &engine::MoveKeys) {
        let delta_t = self.time_since_last_update(current_time);
        if delta_t > 0.0 {
            self.update_fwd_speed(active_keys, delta_t);
            self.update_size_position(active_keys, delta_t);
            self.update_roll_position(active_keys);
            self.update_forward_position(delta_t);
        }
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

    fn update_roll_position(&mut self, active_keys: &engine::MoveKeys) {
        self.roll_position = RollPosition::Level;
        if active_keys.left && !active_keys.right {
            self.roll_position = RollPosition::Left;
        }
        if !active_keys.left && active_keys.right {
            self.roll_position = RollPosition::Right;
        }
    }

    fn time_since_last_update(&self, current_time: f64) -> f32 {
        (current_time - self.last_update_time) as f32
    }

    fn update_forward_position(&mut self, delta_t: f32) {
        self.shape.move_y(self.fwd_speed * delta_t);
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

    const NEXT_FRAME: f64 = (1.0 / 50.0) * (1.05 as f64);
    const INIT_TIME: f64 = 0.0;

    #[test]
    fn player_floats_above_ground() {
        let map_y_coord = -400.0;
        let player = Player::new(map_y_coord, INIT_TIME);
        assert_eq! { player.get_y(), map_y_coord + FLOAT_HEIGHT}
    }

    #[test]
    fn player_initial_position() {
        let player = Player::new(0.0, INIT_TIME);
        assert_eq! {player.get_position(), (0.0, PLAYER_Z)}
    }

    #[test]
    fn player_moves_left() {
        let mut player = Player::new(0.0, INIT_TIME);
        player.update(NEXT_FRAME, &LEFT_PRESS);
        let player_pos = player.get_position();
        assert! {player_pos.0 < 0.0}
    }

    #[test]
    fn player_moves_right() {
        let mut player = Player::new(0.0, INIT_TIME);
        player.update(NEXT_FRAME, &RIGHT_PRESS);
        let player_pos = player.get_position();
        assert! {player_pos.0 > 0.0}
    }

    #[test]
    fn player_does_not_move_sideways() {
        let mut player = Player::new(0.0, INIT_TIME);
        player.update(NEXT_FRAME, &LEFT_RIGHT_PRESS);
        let player_pos = player.get_position();
        assert! {player_pos.0 == 0.0}
    }

    #[test]
    fn player_moves_forward() {
        let mut player = Player::new(0.0, INIT_TIME);
        player.update(NEXT_FRAME, &UP_PRESS);
        let player_pos = player.get_position();
        assert! {player_pos.1 > PLAYER_Z}
    }

    #[test]
    fn player_moves_forward_with_inital_speed() {
        let mut player = Player::new(0.0, INIT_TIME);
        player.update(NEXT_FRAME, &NO_PRESS);
        let player_pos = player.get_position();
        let new_z = PLAYER_Z + INITAL_FWD_SPEED * (NEXT_FRAME as f32);
        assert_eq! {player_pos.1, new_z}
    }

    #[test]
    fn player_accelerates_on_up_press() {
        let mut player = Player::new(0.0, INIT_TIME);
        player.update(NEXT_FRAME, &UP_PRESS);
        let player_pos = player.get_position();
        let new_z = PLAYER_Z + INITAL_FWD_SPEED * (NEXT_FRAME as f32);
        // new_z is the position if no acceleration was present
        assert! {player_pos.1 > new_z}
    }
}
