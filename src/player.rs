use super::rectangle;

const PLAYER_WIDTH: f32 = 100.0;
const PLAYER_DEPTH: f32 = 10.0;
const PLAYER_Z: f32 = 25.0;
const FLOAT_HEIGHT: f32 = 25.0;

pub struct Player {
    shape: rectangle::Rectangle,
    y: f32,
}

impl Player {
    pub fn new(map_y: f32) -> Player {
        Player {
            shape: rectangle::Rectangle::new((0.0, PLAYER_Z), (PLAYER_WIDTH, PLAYER_DEPTH)),
            y: map_y + FLOAT_HEIGHT,
        }
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn draw(&self, fov_distance: f32) {
        self.shape.draw(self.get_y(), fov_distance);
        self.shape.draw(self.get_y() - FLOAT_HEIGHT, fov_distance);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_floats_above_ground() {
        let map_y_coord = -400.0;
        let player = Player::new(map_y_coord);
        assert_eq! { player.get_y(), map_y_coord + FLOAT_HEIGHT}
    }
}
