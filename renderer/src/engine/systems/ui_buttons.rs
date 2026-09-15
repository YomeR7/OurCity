use crate::engine::components::building;
use crate::engine::components::construct;
use crate::engine::components::ui;
use bevy::prelude::*;

/// System to handle the UI buttons
pub fn construct_panel_buttons_handler(
    buttons: Query<(&Interaction, &ui::ButtonKind), Changed<Interaction>>,
    selected_construct: Single<&construct::Construct, With<construct::SelectedConstruct>>,
) {
    for (interaction, kind) in &buttons {
        if *interaction == Interaction::Pressed {
            let construct_id = selected_construct.id().to_string();
            match kind {
                ui::ButtonKind::UpvoteBuilding => crate::api::vote_for_construct(&construct_id, 1),
                ui::ButtonKind::DownvoteBuilding => crate::api::vote_for_construct(&construct_id, -1),
            };
        }
    }
}
