/// Resource receiving messages from JS
pub struct MessageHandler {
    receiver: crate::message::Receiver,
}

impl MessageHandler {
    pub fn new(receiver: crate::message::Receiver) -> Self {
        Self { receiver }
    }

    pub fn recv(&mut self) -> Result<crate::message::Message, std::sync::mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}

/// Resource to hold the loaded assets.
#[derive(bevy::prelude::Resource)]
pub struct BuildingAssets {
    pub house: bevy::prelude::Handle<bevy::prelude::Scene>,
}

/// Resource to hold building ids to bevy entities.
#[derive(bevy::prelude::Resource)]
pub struct BuildingIndex {
    buildings: std::collections::HashMap<uuid::Uuid, bevy::prelude::Entity>,
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
        self.buildings.get(id).cloned()
    }

    /// Insert a new building id / entity pair
    pub fn insert(&mut self, id: uuid::Uuid, entity: bevy::prelude::Entity) {
        self.buildings.insert(id, entity);
    }
}
