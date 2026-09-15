use crate::engine;

/// Resource receiving messages from JS
pub struct MessageHandler {
    receiver: crate::message::Receiver,
    disconnected_flag: bool,
}

impl MessageHandler {
    pub fn new(receiver: crate::message::Receiver) -> Self {
        Self {
            receiver,
            disconnected_flag: false,
        }
    }

    pub fn recv(&mut self) -> Result<crate::message::Message, std::sync::mpsc::TryRecvError> {
        self.receiver.try_recv()
    }

    /// Checks if the disconnected flags is set, if not, return true and sets it.
    /// Returns false otherwise.
    pub fn disconnected_sent(&mut self) -> bool {
        if self.disconnected_flag {
            false
        } else {
            self.disconnected_flag = true;
            true
        }
    }
}

/// Resource to hold the loaded assets.
#[derive(bevy::prelude::Resource)]
pub struct BuildingAssets {
    pub house: bevy::prelude::Handle<bevy::prelude::Scene>,
    pub house_construct: bevy::prelude::Handle<bevy::prelude::Scene>,
}

/// Resource to hold building ids to bevy entities.
#[derive(bevy::prelude::Resource)]
pub struct BuildingIndex {
    buildings: std::collections::HashMap<uuid::Uuid, engine::entity::BuildingEntity>,
}

impl BuildingIndex {
    /// Creates a new empty building index
    pub fn new() -> Self {
        Self {
            buildings: std::collections::HashMap::new(),
        }
    }

    /// Get a building entity from a given index
    pub fn get_building(&self, id: &uuid::Uuid) -> Option<bevy::prelude::Entity> {
        match self.buildings.get(id) {
            Some(engine::entity::BuildingEntity::Building(entity)) => Some(*entity),
            _ => None,
        }
    }

    /// Get a construct entity from a given index
    pub fn get_construct(&self, id: &uuid::Uuid) -> Option<bevy::prelude::Entity> {
        match self.buildings.get(id) {
            Some(engine::entity::BuildingEntity::Construct(entity)) => Some(*entity),
            _ => None,
        }
    }

    /// Insert a new building id / entity pair
    pub fn insert_building(&mut self, id: uuid::Uuid, entity: bevy::prelude::Entity) {
        self.buildings.insert(id, engine::entity::BuildingEntity::Building(entity));
    }

    /// Insert a new building id / entity pair
    pub fn insert_construct(&mut self, id: uuid::Uuid, entity: bevy::prelude::Entity) {
        self.buildings.insert(id, engine::entity::BuildingEntity::Construct(entity));
    }
}
