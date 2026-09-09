pub struct MessageHandler {
    receiver: crate::message::Receiver,
}

impl MessageHandler {
    pub fn new(receiver: crate::message::Receiver) -> Self {
        Self { receiver }
    }
}

pub fn handle_messages(handler: bevy::prelude::NonSendMut<MessageHandler>) {
    /* Drain all pending messages */
    loop {
        match handler.receiver.try_recv() {
            Ok(message) => { /* Handle */ }
            Err(std::sync::mpsc::TryRecvError::Empty) => break,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                bevy::log::tracing::warn!("Bevy com channel disconnected!");
                break;
            }
        }
    }
}
