use crate::engine::components;
use crate::engine::observers;
use crate::engine::resources;

/// Handle a message from JS and update the game engine state accordingly
pub fn handle_message(
    message: crate::message::Message,
    models: &resources::BuildingAssets,
    commands: &mut bevy::prelude::Commands,
    building_map: &mut resources::BuildingIndex,
    ghost_query: &bevy::prelude::Query<(bevy::prelude::Entity, &components::building::BuildingGhost)>,
    construct_query: &mut bevy::prelude::Query<&mut components::construct::Construct>,
) {
    match message {
        crate::message::Message::LoadCity { city } => spawn_city(city, models, commands, building_map),
        crate::message::Message::NewConstruct { construct } => spawn_construct(construct, models, commands, building_map),
        crate::message::Message::StartConstructPlacement { kind } => spawn_building_ghost(&kind, models, commands),
        crate::message::Message::StopConstructPlacement => despawn_building_ghost(commands, ghost_query),
        crate::message::Message::AskConstructionPlacement { response } => get_building_ghost_position(response, ghost_query),
        crate::message::Message::ConstructStatusUpdated { id, status } => {
            update_construct_status(id, status, building_map, construct_query)
        }
        crate::message::Message::ConstructCancelled { id } => dispawn_construct(id, building_map, commands),
        crate::message::Message::ConstructConfirmed { id } => {
            /* Fixme: refactor, + if this is the selected construct we need to deselect */
            let construct_entity = match building_map.get_construct(&id) {
                Some(entity) => entity,
                None => return,
            };
            let building = match construct_query.get(construct_entity) {
                Ok(construct) => construct.building.clone(),
                Err(_) => return,
            };
            dispawn_construct(id, building_map, commands);
            spawn_building(building, models, commands, building_map);
        }
    }
}

/// Spawn an entire city into the engine
fn spawn_city(
    city: api::server_response::GetOurCityResponse,
    models: &resources::BuildingAssets,
    commands: &mut bevy::prelude::Commands,
    building_map: &mut resources::BuildingIndex,
) {
    let api::server_response::GetOurCityResponse { buildings, constructs } = city;
    for building in buildings.into_iter() {
        spawn_building(building, models, commands, building_map);
    }
    for construct in constructs.into_iter() {
        spawn_construct(construct, models, commands, building_map);
    }
}

/// Spawn a new building in the engine
fn spawn_building(
    building: api::common::Building,
    models: &resources::BuildingAssets,
    commands: &mut bevy::prelude::Commands,
    building_map: &mut resources::BuildingIndex,
) {
    /* Create the components for the building */
    let id = building.id;
    let position = components::grid::GridPos {
        x: building.pos.x,
        y: building.pos.y,
    };
    let transform = bevy::prelude::Transform::from_translation(position.world());
    let mesh = bevy::prelude::SceneRoot(models.house.clone());
    let building_component = components::building::Building { building };

    /* Insert the entity in Bevy ECS */
    let entity = commands
        .spawn((position, transform, mesh, building_component))
        .observe(observers::on_building_clicked)
        .id();

    /* Store the entity in the id map */
    building_map.insert_building(id, entity);
}

/// Spawn a new construct in the engine
fn spawn_construct(
    construct: api::common::Construct,
    models: &resources::BuildingAssets,
    commands: &mut bevy::prelude::Commands,
    building_map: &mut resources::BuildingIndex,
) {
    /* Create the components for the building */
    let id = construct.building.id;
    let position = components::grid::GridPos {
        x: construct.building.pos.x,
        y: construct.building.pos.y,
    };
    let transform = bevy::prelude::Transform::from_translation(position.world());
    let mesh = bevy::prelude::SceneRoot(models.house_construct.clone());
    let construct_component = components::construct::Construct {
        building: construct.building,
        status: construct.status,
    };

    /* Insert the entity in Bevy ECS */
    let entity = commands
        .spawn((position, transform, mesh, construct_component))
        .observe(observers::on_construct_clicked)
        .id();

    /* Store the entity in the id map */
    building_map.insert_construct(id, entity);
}

/// Spawn a new construct in the engine
fn update_construct_status(
    construct: uuid::Uuid,
    status: api::common::ConstructStatus,
    building_map: &resources::BuildingIndex,
    construct_query: &mut bevy::prelude::Query<(&mut components::construct::Construct)>,
) {
    let construct_entity = match building_map.get_construct(&construct) {
        Some(entity) => entity,
        None => return,
    };
    let mut construct = match construct_query.get_mut(construct_entity) {
        Ok(construct) => construct,
        Err(_) => return,
    };

    (&mut *construct).status = status;
}

/// Dispawn the given construct
fn dispawn_construct(construct: uuid::Uuid, building_map: &resources::BuildingIndex, commands: &mut bevy::prelude::Commands) {
    let construct_entity = match building_map.get_construct(&construct) {
        Some(entity) => entity,
        None => return,
    };
    let mut construct_entity = match commands.get_entity(construct_entity) {
        Ok(construct) => construct,
        Err(_) => return,
    };
    construct_entity.despawn();
}

/// Spawn a ghost building template for construction indication
fn spawn_building_ghost(building_kind: &str, models: &resources::BuildingAssets, commands: &mut bevy::prelude::Commands) {
    /* Create the components for the building */
    let transform = bevy::prelude::Transform::IDENTITY;
    let mesh = bevy::prelude::SceneRoot(models.house.clone());
    let ghost = components::building::BuildingGhost { position: None };

    /* Insert the entity in Bevy ECS */
    commands.spawn((transform, mesh, ghost));
}

/// Despawn all ghost building templates
fn despawn_building_ghost(
    commands: &mut bevy::prelude::Commands,
    ghost_query: &bevy::prelude::Query<(bevy::prelude::Entity, &components::building::BuildingGhost)>,
) {
    for (entity, _) in ghost_query {
        commands.entity(entity).despawn();
    }
}

/// Get the ghost building position and return it to the sender
fn get_building_ghost_position(
    sender: futures::channel::oneshot::Sender<Result<crate::api::ConstructPlacement, String>>,
    ghost_query: &bevy::prelude::Query<(bevy::prelude::Entity, &components::building::BuildingGhost)>,
) {
    let ghost_components = ghost_query.iter().map(|(_, g)| g).collect::<Vec<_>>();
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
