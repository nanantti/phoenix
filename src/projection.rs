const FOV_DISTANCE: f32 = 100.0;

pub fn ToScreen(point: vector3d::Vector3d<f32>) -> vector2d::Vector2D<f32> {
    let projection: f32 = FOV_DISTANCE / (point.z + FOV_DISTANCE);
    vector2d::Vector2D {
        x: point.x * projection,
        y: point.y * projection,
    }
}
