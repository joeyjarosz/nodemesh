use std::result::Result;

use protocol::messages;

/// Types that can send messages to and receive messages from `nmd`.
pub trait Transport {
    /// Send a batch of messages to the daemon and return the responses in
    /// order.
    fn send(
        &mut self, &messages::Requests
    ) -> Result<messages::Responses, String>;
}
