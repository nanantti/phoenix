pub fn calculate_fov(camera_height: f32, z_max: f32, horizon_delta: f32) -> f32 {
    let horizon_drop: f32 = horizon_delta / camera_height;
    return z_max / (1.0 + 1.0 / horizon_drop);
}

pub fn to_screen(point: vector3d::Vector3d<f32>, fov_distance: f32) -> vector2d::Vector2D<f32> {
    let projection: f32 = fov_distance / (point.z + fov_distance);

    vector2d::Vector2D {
        x: point.x * projection,
        y: point.y * projection,
    }
}
