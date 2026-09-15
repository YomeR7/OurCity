mod handle_message;

use crate::engine::components;
use crate::engine::components::building::SelectedBuilding;
use crate::engine::components::construct::Construct;
use crate::engine::components::construct::SelectedConstruct;
use crate::engine::components::ui::BuildingInfoPanel;
use crate::engine::components::ui::ConstructInfoPanel;
use crate::engine::components::ui::ConstructPanelContent;
use crate::engine::components::ui::HiddableSelectionMarker;
use crate::engine::components::ui::UiSelectionBeam;
use crate::engine::resources;

/// System to load all assets into the ECS
pub fn load_assets(mut commands: bevy::prelude::Commands, assets: bevy::prelude::Res<bevy::prelude::AssetServer>) {
    /* Load all assets */

    /* House and house construct */
    let house = assets.load(bevy::prelude::GltfAssetLabel::Scene(0).from_asset("house.glb"));
    let house_construct = assets.load(bevy::prelude::GltfAssetLabel::Scene(0).from_asset("house_construct.glb"));

    let assets_resources = resources::BuildingAssets { house, house_construct };

    commands.insert_resource(assets_resources);
}

/// System to drain the message handler and handle all the messages.
pub fn handle_messages(
    models: bevy::prelude::Res<resources::BuildingAssets>,
    mut handler: bevy::prelude::NonSendMut<resources::MessageHandler>,
    mut commands: bevy::prelude::Commands,
    mut building_map: bevy::prelude::ResMut<resources::BuildingIndex>,
    ghost_query: bevy::prelude::Query<(bevy::prelude::Entity, &components::building::BuildingGhost)>,
    mut construct_query: bevy::prelude::Query<&mut components::construct::Construct>,
) {
    /* Drain all pending messages */
    loop {
        match handler.recv() {
            Ok(message) => handle_message::handle_message(
                message,
                &models,
                &mut commands,
                &mut building_map,
                &ghost_query,
                &mut construct_query,
            ),
            Err(std::sync::mpsc::TryRecvError::Empty) => break,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                if handler.disconnected_sent() {
                    bevy::log::tracing::warn!("Bevy com channel disconnected!");
                }
                break;
            }
        }
    }
}

/// Update the position of the construction ghost based on a raycast through the camera
pub fn update_construction_ghost(
    camera: bevy::prelude::Single<(&bevy::prelude::Camera, &bevy::prelude::GlobalTransform)>,
    window: bevy::prelude::Single<&bevy::prelude::Window>,
    mut ghost: bevy::prelude::Single<(&mut bevy::prelude::Transform, &mut components::building::BuildingGhost)>,
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

    let hit_point = match ray.plane_intersection_point(
        bevy::prelude::Vec3::ZERO,
        bevy::prelude::InfinitePlane3d::new(bevy::prelude::Vec3::Y),
    ) {
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

/// Update the UI position of the building panel info from the selected building.
pub fn update_building_panel_info(
    camera: bevy::prelude::Single<(&bevy::prelude::Camera, &bevy::prelude::GlobalTransform)>,
    selected_building: bevy::prelude::Single<
        &bevy::prelude::GlobalTransform,
        bevy::prelude::With<components::building::SelectedBuilding>,
    >,
    panel: bevy::prelude::Single<
        (&mut bevy::prelude::Node, &bevy::prelude::ComputedNode),
        bevy::prelude::With<components::ui::BuildingInfoPanel>,
    >,
) {
    let (mut panel_node, panel_computed) = panel.into_inner();
    let (camera, camera_tf) = *camera;

    let anchor = selected_building.translation() + bevy::prelude::Vec3::Y * 2.0;
    let screen_position = match camera.world_to_viewport(camera_tf, anchor) {
        Ok(position) => position,
        Err(_) => return,
    };

    let size = panel_computed.size() * panel_computed.inverse_scale_factor();
    panel_node.display = bevy::prelude::Display::Flex;
    panel_node.left = bevy::prelude::Val::Px(screen_position.x - size.x / 2.0);
    panel_node.top = bevy::prelude::Val::Px(screen_position.y - size.y - 12.0);
}

/// Update the UI position of the construct panel info from the selected construct.
pub fn update_construct_panel_position(
    camera: bevy::prelude::Single<(&bevy::prelude::Camera, &bevy::prelude::GlobalTransform)>,
    selected_construct: bevy::prelude::Single<
        &bevy::prelude::GlobalTransform,
        bevy::prelude::With<components::construct::SelectedConstruct>,
    >,
    panel: bevy::prelude::Single<
        (&mut bevy::prelude::Node, &bevy::prelude::ComputedNode),
        bevy::prelude::With<components::ui::ConstructInfoPanel>,
    >,
) {
    let (mut panel_node, panel_computed) = panel.into_inner();
    let (camera, camera_tf) = *camera;

    let building_height = 1.2;
    let anchor = selected_construct.translation() + bevy::prelude::Vec3::Y * building_height;
    let screen_position = match camera.world_to_viewport(camera_tf, anchor) {
        Ok(position) => position,
        Err(_) => return,
    };

    let size = panel_computed.size() * panel_computed.inverse_scale_factor();
    panel_node.left = bevy::prelude::Val::Px(screen_position.x - size.x / 2.0);
    panel_node.top = bevy::prelude::Val::Px(screen_position.y - size.y);
}

/// Update the UI position of the construct panel info from the selected construct.
pub fn update_building_panel_position(
    camera: bevy::prelude::Single<(&bevy::prelude::Camera, &bevy::prelude::GlobalTransform)>,
    selected_building: bevy::prelude::Single<
        &bevy::prelude::GlobalTransform,
        bevy::prelude::With<components::building::SelectedBuilding>,
    >,
    panel: bevy::prelude::Single<
        (&mut bevy::prelude::Node, &bevy::prelude::ComputedNode),
        bevy::prelude::With<components::ui::BuildingInfoPanel>,
    >,
) {
    let (mut panel_node, panel_computed) = panel.into_inner();
    let (camera, camera_tf) = *camera;

    let building_height = 1.2;
    let anchor = selected_building.translation() + bevy::prelude::Vec3::Y * building_height;
    let screen_position = match camera.world_to_viewport(camera_tf, anchor) {
        Ok(position) => position,
        Err(_) => return,
    };

    let size = panel_computed.size() * panel_computed.inverse_scale_factor();
    panel_node.left = bevy::prelude::Val::Px(screen_position.x - size.x / 2.0);
    panel_node.top = bevy::prelude::Val::Px(screen_position.y - size.y);
}

/// Deselect the currently selected building when a click is detected and hasn't hit anything
pub fn deselect_on_miss(
    pointers: bevy::prelude::Query<&bevy::picking::pointer::PointerInteraction>,
    selected_buildings: bevy::prelude::Query<bevy::prelude::Entity, bevy::prelude::With<SelectedBuilding>>,
    selected_constructs: bevy::prelude::Query<bevy::prelude::Entity, bevy::prelude::With<SelectedConstruct>>,
    ui_nodes: bevy::prelude::Query<(), bevy::prelude::With<bevy::prelude::Node>>,
    selection_objects: bevy::prelude::Query<&mut bevy::prelude::Visibility, bevy::prelude::With<HiddableSelectionMarker>>,
    mut commands: bevy::prelude::Commands,
) {
    /* If the click hit anything, don't deselect. Deselect when the click is a full miss */
    if pointers.iter().any(|p| {
        p.get_nearest_hit()
            .is_some_and(|(e, _)| selected_buildings.contains(*e) || selected_constructs.contains(*e) || ui_nodes.contains(*e))
    }) {
        return;
    }

    /* Remove selection components */
    for selected_building in &selected_buildings {
        commands.entity(selected_building).remove::<SelectedBuilding>();
    }
    for selected_construct in &selected_constructs {
        commands.entity(selected_construct).remove::<SelectedConstruct>();
    }

    /* Hide all UI for selected objects */
    for mut selection_object in selection_objects {
        *selection_object = bevy::prelude::Visibility::Hidden;
    }
}

pub fn update_construction_panel_timer(
    selected_construct: bevy::prelude::Single<(&SelectedConstruct, &components::construct::Construct)>,
    mut construct_info_panel_content: bevy::prelude::Single<(&mut bevy::prelude::Text, &ConstructPanelContent)>,
) {
    let (_, construct) = *selected_construct;
    let (content, _) = &mut *construct_info_panel_content;

    fn get_remaining_voting_time(construction_time: chrono::DateTime<chrono::Utc>) -> Option<chrono::Duration> {
        let now = chrono::Utc::now();
        let voting_deadline = construction_time.checked_add_days(chrono::Days::new(1))?;
        Some(voting_deadline - now)
    }

    let remaining_time_text = match get_remaining_voting_time(construct.building.created_at) {
        Some(remaining) => {
            if remaining >= chrono::Duration::zero() {
                let rem_hours = remaining.num_hours();
                let rem_minutes = (remaining.num_minutes() % 60) + 60 % 60;
                let rem_seconds = (remaining.num_seconds() % 60) + 60 % 60;
                format!("Votes end in {rem_hours}:{rem_minutes:0>2}:{rem_seconds:0>2}")
            } else {
                format!("Vote ended!")
            }
        }
        None => format!("Asked the {}", construct.building.created_at.format("%Y-%m-%d at %H:%M")),
    };

    content.0 = format!(
        "A cute lil house.\nVotes: {} / {}\n{}",
        construct.status.votes, construct.status.cost, remaining_time_text
    );
}

/// System to handle the UI buttons
pub fn construct_panel_buttons_handler(
    buttons: bevy::prelude::Query<
        (&bevy::prelude::Interaction, &components::ui::ButtonKind),
        bevy::prelude::Changed<bevy::prelude::Interaction>,
    >,
    selected_construct: bevy::prelude::Single<
        &components::construct::Construct,
        bevy::prelude::With<components::construct::SelectedConstruct>,
    >,
) {
    for (interaction, kind) in &buttons {
        if *interaction == bevy::prelude::Interaction::Pressed {
            let construct_id = selected_construct.building.id.to_string();
            match kind {
                components::ui::ButtonKind::UpvoteBuilding => crate::api::vote_for_construct(&construct_id, 1),
                components::ui::ButtonKind::DownvoteBuilding => crate::api::vote_for_construct(&construct_id, -1),
            };
        }
    }
}

/// Camera controller to allow to move the camera
pub fn camera_controller(
    buttons: bevy::prelude::Res<bevy::prelude::ButtonInput<bevy::prelude::MouseButton>>,
    motion: bevy::prelude::Res<bevy::input::mouse::AccumulatedMouseMotion>,
    scroll: bevy::prelude::Res<bevy::input::mouse::AccumulatedMouseScroll>,
    window: bevy::prelude::Single<&bevy::prelude::Window, bevy::prelude::With<bevy::window::PrimaryWindow>>,
    camera: bevy::prelude::Single<
        (
            &bevy::prelude::Camera,
            &mut bevy::prelude::Transform,
            &mut bevy::prelude::Projection,
        ),
        bevy::prelude::With<bevy::prelude::Camera3d>,
    >,
) {
    const ZOOM_STEP: f32 = 1.1; // zoom factor per scroll line
    const MIN_HEIGHT: f32 = 2.0;
    const MAX_HEIGHT: f32 = 200.0;
    const ORBIT_SPEED: f32 = 0.005; // radians per pixel

    let (camera, mut transform, mut projection) = camera.into_inner();
    let bevy::prelude::Projection::Orthographic(ortho) = &mut *projection else {
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
    if buttons.pressed(bevy::prelude::MouseButton::Left) && motion.delta != bevy::prelude::Vec2::ZERO {
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
    // if buttons.pressed(bevy::prelude::MouseButton::Right) && motion.delta.x != 0.0 {
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
    //     let rot = bevy::prelude::Quat::from_rotation_y(-motion.delta.x * ORBIT_SPEED);
    //     transform.translation = pivot + rot * (pos - pivot);
    //     transform.rotation = rot * transform.rotation;
    // }
}
