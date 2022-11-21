use super::projection;
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

    pub fn draw(&self, projection: &projection::Projection) {
        let zero_projection = projection::Projection::make_zero_projection(projection); // Verbose, aren't we?
        self.shape.draw(self.get_y(), &zero_projection);
        self.shape
            .draw(self.get_y() - FLOAT_HEIGHT, &zero_projection);
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
