use crate::engine;

/// System to load all assets into the ECS
pub fn load_assets(mut commands: bevy::prelude::Commands, assets: bevy::prelude::Res<bevy::prelude::AssetServer>) {
    /* Load all assets */
    let house = assets.load(bevy::prelude::GltfAssetLabel::Scene(0).from_asset("house.glb"));
    let assets_resources = engine::resources::BuildingAssets { house };

    commands.insert_resource(assets_resources);
}

/// System to drain the message handler and handle all the messages.
pub fn handle_messages(
    models: bevy::prelude::Res<engine::resources::BuildingAssets>,
    mut handler: bevy::prelude::NonSendMut<engine::resources::MessageHandler>,
    mut commands: bevy::prelude::Commands,
    mut building_map: bevy::prelude::ResMut<engine::resources::BuildingIndex>,
) {
    /* Drain all pending messages */
    loop {
        match handler.recv() {
            Ok(message) => handle_message(message, &models, &mut commands, &mut building_map),
            Err(std::sync::mpsc::TryRecvError::Empty) => break,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                bevy::log::tracing::warn!("Bevy com channel disconnected!");
                break;
            }
        }
    }
}

fn handle_message(
    message: crate::message::Message,
    models: &engine::resources::BuildingAssets,
    commands: &mut bevy::prelude::Commands,
    building_map: &mut engine::resources::BuildingIndex,
) {
    match message {
        crate::message::Message::NewConstruct { x, y, id, votes, cost } => {
            /* Create the components for the building */
            let building = engine::components::Construct {
                building: engine::components::Building {
                    id,
                    name: "No names Yet!".to_string(),
                    kind: "No kinds Yet!".to_string(),
                },
                votes,
                cost,
            };
            let position = engine::components::GridPos { x, y };
            let transform = bevy::prelude::Transform::from_translation(position.world());
            let mesh = bevy::prelude::SceneRoot(models.house.clone());

            /* Insert the entity in Bevy ECS */
            let entity = commands.spawn((building, position, transform, mesh)).id();

            /* Store the entity in the id map */
            building_map.insert(id, entity);
        }
        _ => {}
    }
}
