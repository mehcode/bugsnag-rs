use super::event::Event;

const NOTIFIER_NAME: &'static str = "Bugsnag Rust";
const NOTIFIER_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NOTIFIER_URL: &'static str = "url";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Notifier {
    name: String,
    version: String,
    url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    api_key: String,
    notifier: Notifier,
    events: Vec<Event>,
}

impl Notification {
    pub fn new(apikey: &str, events: Vec<Event>) -> Notification {
        Notification {
            api_key: apikey.to_owned(),
            notifier: Notifier {
                name: NOTIFIER_NAME.to_owned(),
                version: NOTIFIER_VERSION.to_owned(),
                url: NOTIFIER_URL.to_owned(),
            },
            events: events,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Notification, NOTIFIER_NAME, NOTIFIER_VERSION, NOTIFIER_URL};
    use super::super::{event, exception, stacktrace};
    use serde_test::{Token, assert_ser_tokens};
    use serde_json;

    #[test]
    fn test_notification_to_json() {
        let notification = Notification::new("safe-api-key", Vec::new());

        assert_ser_tokens(&notification,
                          &[Token::StructStart("Notification", 3),
                            Token::StructSep,
                            Token::Str("apiKey"),
                            Token::Str("safe-api-key"),
                            Token::StructSep,
                            Token::Str("notifier"),
                            Token::StructStart("Notifier", 3),
                            Token::StructSep,
                            Token::Str("name"),
                            Token::Str(NOTIFIER_NAME),
                            Token::StructSep,
                            Token::Str("version"),
                            Token::Str(NOTIFIER_VERSION),
                            Token::StructSep,
                            Token::Str("url"),
                            Token::Str(NOTIFIER_URL),
                            Token::StructEnd,
                            Token::StructSep,
                            Token::Str("events"),
                            Token::SeqStart(Some(0)),
                            Token::SeqEnd,
                            Token::StructEnd]);
    }

    #[test]
    fn test_notification_with_event_to_json() {
        let frame = stacktrace::Frame::new("test.rs", 400, "test");
        let exception = exception::Exception::new("Assert", "Assert", vec![frame]);
        let event = event::Event::new(vec![exception]);

        let notification = Notification::new("safe-api-key", vec![event]);

        println!("{}", serde_json::to_string(&notification).unwrap());

        assert_ser_tokens(&notification,
                          &[Token::StructStart("Notification", 3),
                            Token::StructSep,
                            Token::Str("piKey"),
                            Token::Str("safe-api-key"),
                            Token::StructSep,
                            Token::Str("notifier"),
                            Token::StructStart("Notifier", 3),
                            Token::StructSep,
                            Token::Str("name"),
                            Token::Str(NOTIFIER_NAME),
                            Token::StructSep,
                            Token::Str("version"),
                            Token::Str(NOTIFIER_VERSION),
                            Token::StructSep,
                            Token::Str("url"),
                            Token::Str(NOTIFIER_URL),
                            Token::StructEnd,
                            Token::StructSep,
                            Token::Str("events"),
                            Token::SeqStart(Some(1)),
                            Token::SeqSep,
                            Token::StructStart("Event", 2),
                            Token::StructSep,
                            Token::Str("payloadVersion"),
                            Token::U32(event::PAYLOAD_VERSION),
                            Token::StructSep,
                            Token::Str("exceptions"),
                            Token::SeqStart(Some(1)),
                            Token::SeqSep,
                            Token::StructStart("Exception", 3),
                            Token::StructSep,
                            Token::Str("errorClass"),
                            Token::Str("Assert"),
                            Token::StructSep,
                            Token::Str("message"),
                            Token::Str("Assert"),
                            Token::StructSep,
                            Token::Str("stacktrace"),
                            Token::SeqStart(Some(1)),
                            Token::SeqSep,
                            Token::StructStart("Frame", 3),
                            Token::StructSep,
                            Token::Str("file"),
                            Token::Str("test.rs"),
                            Token::StructSep,
                            Token::Str("lineNumber"),
                            Token::U32(400),
                            Token::StructSep,
                            Token::Str("method"),
                            Token::Str("test"),
                            Token::StructEnd,
                            Token::SeqEnd,
                            Token::StructEnd,
                            Token::SeqEnd,
                            Token::StructEnd,
                            Token::SeqEnd,
                            Token::StructEnd]);
    }
}
