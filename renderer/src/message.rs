pub enum Message {
    NewConstruct {
        id: uuid::Uuid,
        x: i32,
        y: i32,
        votes: u32,
        cost: u32,
    },
    ConstructConfirmed {
        id: uuid::Uuid,
    },
    ConstructStatusUpdated {
        votes: u32,
        cost: u32,
    },
}

pub type Sender = std::sync::mpsc::Sender<Message>;
pub type Receiver = std::sync::mpsc::Receiver<Message>;
