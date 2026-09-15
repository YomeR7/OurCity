#[derive(Debug)]
pub enum Message {
    AskConstructionPlacement {
        response: futures::channel::oneshot::Sender<Result<crate::api::ConstructPlacement, String>>,
    },
    ConstructCancelled {
        id: uuid::Uuid,
    },
    ConstructConfirmed {
        id: uuid::Uuid,
    },
    ConstructStatusUpdated {
        id: uuid::Uuid,
        status: api::common::ConstructStatus,
    },
    LoadCity {
        city: api::server_response::GetOurCityResponse,
    },
    NewConstruct {
        construct: api::common::Construct,
    },
    StartConstructPlacement {
        kind: String,
    },
    StopConstructPlacement,
}

pub type Sender = std::sync::mpsc::Sender<Message>;
pub type Receiver = std::sync::mpsc::Receiver<Message>;
