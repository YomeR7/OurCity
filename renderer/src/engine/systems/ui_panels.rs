use bevy::prelude::*;

use crate::engine::components::building;
use crate::engine::components::construct;
use crate::engine::components::ui;

/// Update the UI position of the building panel info from the selected building.
pub fn update_building_panel_info(
    camera: Single<(&Camera, &GlobalTransform)>,
    selected_building: Single<&GlobalTransform, With<building::SelectedBuilding>>,
    panel: Single<(&mut Node, &ComputedNode), With<ui::BuildingInfoPanel>>,
) {
    let (mut panel_node, panel_computed) = panel.into_inner();
    let (camera, camera_tf) = *camera;

    let anchor = selected_building.translation() + Vec3::Y * 2.0;
    let screen_position = match camera.world_to_viewport(camera_tf, anchor) {
        Ok(position) => position,
        Err(_) => return,
    };

    let size = panel_computed.size() * panel_computed.inverse_scale_factor();
    panel_node.display = Display::Flex;
    panel_node.left = Val::Px(screen_position.x - size.x / 2.0);
    panel_node.top = Val::Px(screen_position.y - size.y - 12.0);
}

/// System to update the text of the construct info panel continuously, to see the timer going down live.
pub fn update_construct_panel_timer(
    selected_construct: Single<&construct::Construct, With<construct::SelectedConstruct>>,
    mut construct_info_panel_content: Single<&mut Text, With<ui::ConstructPanelContent>>,
) {
    construct_info_panel_content.0 = selected_construct.description();
}
