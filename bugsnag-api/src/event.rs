use super::exception::Exception;

const PAYLOAD_VERSION: u32 = 2;

// An event represents an exception
pub struct Event {
    exceptions: Vec<Exception>,
}
