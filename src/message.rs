use std::sync::mpsc;

#[derive(Debug)]
/// Gloabl message implement with sync::mpsc
pub struct Message {
    pub tx: mpsc::Sender<MessageType>,
    pub rx: mpsc::Receiver<MessageType>,
}

impl Default for Message {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        Self { tx, rx }
    }
}

#[derive(Debug)]
pub enum MessageType {
    /// Message from v2ray core process
    Core(String),
}
