pub struct Projection {
    fov_distance: f32,
    offset_x: f32,
    offset_z: f32,
}

impl Projection {
    pub fn new(camera_height: f32, z_max: f32, horizon_delta: f32) -> Projection {
        let horizon_drop: f32 = horizon_delta / camera_height;
        let fov_distance = z_max / (1.0 + 1.0 / horizon_drop);
        Projection {
            fov_distance,
            offset_x: 0.0,
            offset_z: 0.0,
        }
    }

    pub fn to_screen(&self, point: vector3d::Vector3d<f32>) -> vector2d::Vector2D<f32> {
        let corrected_z = point.z - self.offset_z;
        let projection: f32 = self.fov_distance / (corrected_z + self.fov_distance);
        vector2d::Vector2D {
            x: (point.x - self.offset_x) * projection,
            y: point.y * projection,
        }
    }

    pub fn set_offset(&mut self, x: f32, z: f32) {
        self.offset_x = x;
        self.offset_z = z;
    }

    pub fn get_fov(&self) -> f32 {
        self.fov_distance
    }

    pub fn make_compensated_projection(base: &Projection, offset: (f32, f32)) -> Projection {
        Projection {
            fov_distance: base.get_fov(),
            offset_x: offset.0,
            offset_z: 0.0,
        }
    }
}
