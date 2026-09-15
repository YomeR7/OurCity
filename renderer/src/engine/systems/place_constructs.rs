use crate::engine::components;
use bevy::prelude::*;

/// Update the position of the construct ghost based on a raycast through the camera
pub fn update_construct_ghost(
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut ghost: Single<(&mut Transform, &mut components::building::BuildingGhost)>,
) {
    let (camera, cam_tf) = *camera;

    let cursor = match window.cursor_position() {
        Some(cursor) => cursor,
        None => return,
    };
    let ray = match camera.viewport_to_world(cam_tf, cursor) {
        Ok(ray) => ray,
        Err(_) => return,
    };

    let hit_point = match ray.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) {
        Some(hit_point) => hit_point,
        None => return,
    };

    let (ghost_tf, ghost) = &mut *ghost;

    ghost_tf.translation = hit_point.round();
    ghost.position = Some(api::common::Position {
        x: hit_point.x.round() as i32,
        y: hit_point.z.round() as i32,
    });
}
