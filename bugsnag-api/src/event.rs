use super::exception::Exception;

pub const PAYLOAD_VERSION: u32 = 2;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    payload_version: u32,
    exceptions: Vec<Exception>,
}

impl Event {
    pub fn new(exceptions: Vec<Exception>) -> Event {
        Event {
            payload_version: PAYLOAD_VERSION,
            exceptions: exceptions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, PAYLOAD_VERSION};
    use serde_test::{Token, assert_ser_tokens};

    #[test]
    fn test_event_to_json() {
        let ex = Event::new(Vec::new());

        assert_ser_tokens(&ex,
                          &[Token::StructStart("Event", 2),
                            Token::StructSep,
                            Token::Str("payloadVersion"),
                            Token::U32(PAYLOAD_VERSION),
                            Token::StructSep,
                            Token::Str("exceptions"),
                            Token::SeqStart(Some(0)),
                            Token::SeqEnd,
                            Token::StructEnd]);
    }
}
