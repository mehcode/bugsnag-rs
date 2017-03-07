use super::exception::Exception;
use super::bugsnag::Severity;

pub const PAYLOAD_VERSION: u32 = 2;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event<'a> {
    payload_version: u32,
    exceptions: &'a [Exception<'a>],
    severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<&'a str>,
}

impl<'a> Event<'a> {
    pub fn new(exceptions: &'a [Exception],
               severity: Severity,
               context: Option<&'a str>)
               -> Event<'a> {
        Event {
            payload_version: PAYLOAD_VERSION,
            exceptions: exceptions,
            severity: severity,
            context: context,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, PAYLOAD_VERSION, Severity};
    use serde_test::{Token, assert_ser_tokens};
    use serde_json;

    #[test]
    fn test_event_to_json() {
        let empty_vec = Vec::new();
        let evt = Event::new(&empty_vec, Severity::Error, None);

        assert_ser_tokens(&evt,
                          &[Token::StructStart("Event", 3),
                            Token::StructSep,
                            Token::Str("payloadVersion"),
                            Token::U32(PAYLOAD_VERSION),
                            Token::StructSep,
                            Token::Str("exceptions"),
                            Token::SeqStart(Some(0)),
                            Token::SeqEnd,
                            Token::StructSep,
                            Token::Str("severity"),
                            Token::EnumUnit("Severity", "error"),
                            Token::StructEnd]);
    }

    #[test]
    fn test_event_with_context_to_json() {
        let empty_vec = Vec::new();
        let evt = Event::new(&empty_vec, Severity::Error, Some("test/context"));

        println!("{}", serde_json::to_string(&evt).unwrap());

        assert_ser_tokens(&evt,
                          &[Token::StructStart("Event", 4),
                            Token::StructSep,
                            Token::Str("payloadVersion"),
                            Token::U32(PAYLOAD_VERSION),
                            Token::StructSep,
                            Token::Str("exceptions"),
                            Token::SeqStart(Some(0)),
                            Token::SeqEnd,
                            Token::StructSep,
                            Token::Str("severity"),
                            Token::EnumUnit("Severity", "error"),
                            Token::StructSep,
                            Token::Str("context"),
                            Token::Option(true),
                            Token::Str("test/context"),
                            Token::StructEnd]);
    }
}
