pub enum Message {}

pub type Sender = std::sync::mpsc::Sender<Message>;
pub type Receiver = std::sync::mpsc::Receiver<Message>;
