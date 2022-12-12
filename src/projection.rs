pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y:f32, z:f32) -> Point3D {
        Point3D {
            x, y, z,
        }
    }
}

pub struct Projection {
    fov_distance: f32,
    offset_x: f32,
    offset_z: f32,
    draw_distance: f32,
}

impl Projection {
    const FOV_MAX: f32 = 200.0;
    const FOV_MIN: f32 = 400.0;
    const HORIZON_DELTA: f32 = 10.0;
    const Z_MAX: f32 = 4000.0;
    pub fn new(camera_height: f32) -> Projection {
        let horizon_drop: f32 = Projection::HORIZON_DELTA / camera_height;
        let fov_distance = Projection::Z_MAX / (1.0 + 1.0 / horizon_drop);
        Projection {
            fov_distance,
            offset_x: 0.0,
            offset_z: 0.0,
            draw_distance: Projection::Z_MAX,
        }
    }

    pub fn to_screen(&self, point: &Point3D) -> vector2d::Vector2D<f32> {
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

    pub fn set_fov(&mut self, fov_pu: f32) {
        self.fov_distance =
            Projection::FOV_MIN + (fov_pu) * (Projection::FOV_MAX - Projection::FOV_MIN);
    }

    pub fn make_compensated_projection(base: &Projection, offset: (f32, f32)) -> Projection {
        Projection {
            fov_distance: base.get_fov(),
            offset_x: offset.0,
            offset_z: offset.1,
            draw_distance: base.draw_distance,
        }
    }

    pub fn is_point_in_view_zone(&self, point_xz: &(f32, f32)) -> bool {
        self.is_z_in_range(point_xz.1)
        // Optional: add x detection
    }

    pub fn get_view_zone_z_range(&self) -> (f32, f32) {
        (self.offset_z, self.offset_z + self.draw_distance)
    }

    fn is_z_in_range(&self, z: f32) -> bool {
        let range = self.get_view_zone_z_range();
        (range.0 <= z) && (z <= range.1)
    }
}
