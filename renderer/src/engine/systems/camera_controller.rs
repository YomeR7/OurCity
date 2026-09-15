use bevy::prelude::*;

const ZOOM_STEP: f32 = 1.1; // zoom factor per scroll line
const MIN_HEIGHT: f32 = 2.0;
const MAX_HEIGHT: f32 = 200.0;
const ORBIT_SPEED: f32 = 0.005; // radians per pixel

/// Camera controller to allow to move the camera
pub fn camera_controller(
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<bevy::input::mouse::AccumulatedMouseMotion>,
    scroll: Res<bevy::input::mouse::AccumulatedMouseScroll>,
    window: Single<&Window, With<bevy::window::PrimaryWindow>>,
    camera: Single<(&Camera, &mut Transform, &mut Projection), With<Camera3d>>,
) {
    let (camera, mut transform, mut projection) = camera.into_inner();
    let Projection::Orthographic(ortho) = &mut *projection else {
        return;
    };
    let bevy::camera::ScalingMode::FixedVertical { viewport_height } = &mut ortho.scaling_mode else {
        return;
    };
    let Some(viewport) = camera.logical_viewport_size() else {
        return;
    };

    // World units per logical pixel at the current zoom.
    let units_per_px = *viewport_height / viewport.y;
    let right = transform.right();
    let up = transform.up();

    // Drag: move the camera opposite to the cursor delta (screen y points down).
    if buttons.pressed(MouseButton::Left) && motion.delta != Vec2::ZERO {
        let d = motion.delta * units_per_px;
        transform.translation += right * -d.x + up * d.y;
    }

    // Scroll: zoom around the cursor so the world point under it stays fixed.
    if scroll.delta.y != 0.0 {
        let lines = match scroll.unit {
            bevy::input::mouse::MouseScrollUnit::Line => scroll.delta.y,
            bevy::input::mouse::MouseScrollUnit::Pixel => scroll.delta.y / 50.0,
        };
        let new_height = (*viewport_height * ZOOM_STEP.powf(-lines)).clamp(MIN_HEIGHT, MAX_HEIGHT);
        let new_units_per_px = new_height / viewport.y;

        if let Some(cursor) = window.cursor_position() {
            // Cursor offset from the viewport centre, in pixels, y up.
            let mut off = cursor - viewport / 2.0;
            off.y = -off.y;
            // The point under the cursor is centre + off * scale; keep it in place.
            let shift = off * (units_per_px - new_units_per_px);
            transform.translation += right * shift.x + up * shift.y;
        }
        *viewport_height = new_height;
    }

    // Right-drag: orbit around the ground point at the view centre, yaw only.
    // if buttons.pressed(MouseButton::Right) && motion.delta.x != 0.0 {
    //     let pos = transform.translation;
    //     let fwd = transform.forward();
    //     // Intersect the camera's forward ray with the y = 0 plane; fall back to the camera itself.
    //     let pivot = if fwd.y.abs() > 1e-4 {
    //         let t = -pos.y / fwd.y;
    //         if t > 0.0 { pos + fwd * t } else { pos }
    //     } else {
    //         pos
    //     };
    //
    //     let rot = Quat::from_rotation_y(-motion.delta.x * ORBIT_SPEED);
    //     transform.translation = pivot + rot * (pos - pivot);
    //     transform.rotation = rot * transform.rotation;
    // }
}
