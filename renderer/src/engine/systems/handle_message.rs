use crate::engine::components;
use crate::engine::resources;
use crate::engine::systems;
use bevy::prelude::*;

#[derive(bevy::ecs::system::SystemParam)]
pub struct MessageHandlerParams<'w, 's> {
    /// Commands to spawn / despawn entities
    commands: Commands<'w, 's>,

    /// Resource holding all the 3D models used
    assets: Res<'w, resources::BuildingAssets>,
    /// Resource holding the message handler
    handler: NonSendMut<'w, resources::MessageHandler>,
    /// Resource holding the building map
    building_map: ResMut<'w, resources::BuildingIndex>,

    /// Query to all the ghost buildings
    ghost_query: Query<'w, 's, (Entity, &'static components::building::BuildingGhost)>,
    /// Query to all the constructs buildings
    construct_query: Query<'w, 's, &'static mut components::construct::Construct>,
}

/// System to drain the message handler and handle all the messages.
pub fn handle_messages(mut params: MessageHandlerParams) {
    use crate::message::Message;

    /* Drain all pending messages */
    loop {
        match params.handler.recv() {
            Ok(Message::AskConstructPlacement { response }) => get_ghost_position(&mut params, response),
            Ok(Message::ConstructCancelled { id }) => dispawn_construct(&mut params, &id),
            Ok(Message::ConstructConfirmed { id }) => construct_confirmed(&mut params, &id),
            Ok(Message::ConstructStatusUpdated { id, status }) => update_construct_status(&mut params, &id, status),
            Ok(Message::LoadCity { city }) => spawn_city(&mut params, city),
            Ok(Message::NewConstruct { construct }) => spawn_construct(&mut params, construct),
            Ok(Message::StartConstructPlacement { kind }) => spawn_building_ghost(&mut params, &kind),
            Ok(Message::StopConstructPlacement) => despawn_building_ghost(&mut params),
            Err(std::sync::mpsc::TryRecvError::Empty) => break,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                if params.handler.disconnected_sent() {
                    bevy::log::tracing::warn!("Bevy com channel disconnected!");
                }
                break;
            }
        }
    }
}

/// Spawn an entire city into the engine
fn spawn_city(params: &mut MessageHandlerParams, city: api::server_response::GetOurCityResponse) {
    let api::server_response::GetOurCityResponse { buildings, constructs } = city;
    for building in buildings.into_iter() {
        spawn_building(params, building);
    }
    for construct in constructs.into_iter() {
        spawn_construct(params, construct);
    }
}

/// Spawn a new building in the engine
fn spawn_building(params: &mut MessageHandlerParams, building: api::common::Building) {
    /* Create the components for the building */
    let id = building.id;
    let position = components::grid::GridPos {
        x: building.pos.x,
        y: building.pos.y,
    };
    let transform = Transform::from_translation(position.world());
    let mesh = SceneRoot(params.assets.house.clone());
    let building_component = components::building::Building::new(building);

    /* Insert the entity in Bevy ECS */
    let entity = params
        .commands
        .spawn((position, transform, mesh, building_component))
        .observe(systems::on_building_clicked)
        .id();

    /* Store the entity in the id map */
    params.building_map.insert_building(id, entity);
}

/// Spawn a new construct in the engine
fn spawn_construct(params: &mut MessageHandlerParams, construct: api::common::Construct) {
    /* Create the components for the building */
    let id = construct.building.id;
    let position = components::grid::GridPos {
        x: construct.building.pos.x,
        y: construct.building.pos.y,
    };
    let transform = Transform::from_translation(position.world());
    let mesh = SceneRoot(params.assets.house_construct.clone());
    let construct_component = components::construct::Construct::new(construct);

    /* Insert the entity in Bevy ECS */
    let entity = params
        .commands
        .spawn((position, transform, mesh, construct_component))
        .observe(systems::on_construct_clicked)
        .id();

    /* Store the entity in the id map */
    params.building_map.insert_construct(id, entity);
}

/// Spawn a new construct in the engine
fn update_construct_status(params: &mut MessageHandlerParams, construct: &uuid::Uuid, status: api::common::ConstructStatus) {
    let construct_entity = match params.building_map.get_construct(construct) {
        Some(entity) => entity,
        None => return,
    };
    let mut construct = match params.construct_query.get_mut(construct_entity) {
        Ok(construct) => construct,
        Err(_) => return,
    };

    construct.update_status(status);
}

/// Dispawn the given construct
fn dispawn_construct(params: &mut MessageHandlerParams, construct: &uuid::Uuid) {
    let construct_entity = match params.building_map.get_construct(construct) {
        Some(entity) => entity,
        None => return,
    };
    let mut construct_entity = match params.commands.get_entity(construct_entity) {
        Ok(construct) => construct,
        Err(_) => return,
    };
    construct_entity.despawn();
}

/// Spawn a ghost building template for construct indication
fn spawn_building_ghost(params: &mut MessageHandlerParams, building_kind: &str) {
    /* Create the components for the building */
    let transform = Transform::IDENTITY;
    let mesh = SceneRoot(params.assets.house.clone());
    let ghost = components::building::BuildingGhost { position: None };

    /* Insert the entity in Bevy ECS */
    params.commands.spawn((transform, mesh, ghost));
}

/// Despawn all ghost building templates
fn despawn_building_ghost(params: &mut MessageHandlerParams) {
    for (entity, _) in params.ghost_query {
        params.commands.entity(entity).despawn();
    }
}

/// Get the ghost building position and return it to the sender
fn get_ghost_position(
    params: &MessageHandlerParams,
    sender: futures::channel::oneshot::Sender<Result<crate::api::ConstructPlacement, String>>,
) {
    let ghost_components = params.ghost_query.iter().map(|(_, g)| g).collect::<Vec<_>>();
    let message = match ghost_components.as_slice() {
        [] => Err(format!("No ghost building to provide placement!")),
        [ghost] => match &ghost.position {
            Some(position) => Ok(crate::api::ConstructPlacement {
                x: position.x,
                y: position.y,
                width: 1,
                height: 1,
            }),
            None => Err(format!("Ghost has no known position found!")),
        },
        _ => Err(format!("Multiple ghost building found!")),
    };
    if let Err(e) = sender.send(message) {}
}

fn construct_confirmed(params: &mut MessageHandlerParams, construct: &uuid::Uuid) {
    /* Fixme: if this is the selected construct, select the building ? */
    let construct_entity = match params.building_map.get_construct(construct) {
        Some(entity) => entity,
        None => return,
    };
    let building = match params.construct_query.get(construct_entity) {
        Ok(construct) => construct.building(),
        Err(_) => return,
    };
    dispawn_construct(params, construct);
    spawn_building(params, building);
}
