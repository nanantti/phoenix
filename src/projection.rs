static mut fov_distance: f32 = 1.0;

pub fn set_fov_distance(fov_dist: f32) {
    unsafe {
        fov_distance = fov_dist;
    }
}

pub fn ToScreen(point: vector3d::Vector3d<f32>) -> vector2d::Vector2D<f32> {
    unsafe {
        let projection: f32 = fov_distance / (point.z + fov_distance);

        vector2d::Vector2D {
            x: point.x * projection,
            y: point.y * projection,
        }
    }
}
