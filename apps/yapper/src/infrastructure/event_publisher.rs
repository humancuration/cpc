use async_channel::Sender;
use crate::domain::events::{YapperEvent, EventPublisher};

pub struct ChannelEventPublisher {
    sender: Sender<YapperEvent>,
}

impl ChannelEventPublisher {
    pub fn new(sender: Sender<YapperEvent>) -> Self {
        Self { sender }
    }
}

impl EventPublisher for ChannelEventPublisher {
    fn publish(&self, event: YapperEvent) {
        // In a real implementation, we would handle errors properly
        let _ = self.sender.try_send(event);
    }
}