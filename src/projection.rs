pub fn ToScreen(point: vector3d::Vector3d<f32>, fov_distance: f32) -> vector2d::Vector2D<f32> {
    let projection: f32 = fov_distance / (point.z + fov_distance);
    vector2d::Vector2D {
        x: point.x * projection,
        y: point.y * projection,
    }
}
