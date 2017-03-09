use super::event::Event;

const NOTIFIER_NAME: &'static str = "Bugsnag Rust";
const NOTIFIER_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NOTIFIER_URL: &'static str = "https://github.com/superscale/bugsnag-api-rs";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Notifier {
    name: &'static str,
    version: &'static str,
    url: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification<'a> {
    api_key: &'a str,
    notifier: Notifier,
    events: &'a [Event<'a>],
}

impl<'a> Notification<'a> {
    pub fn new(apikey: &'a str, events: &'a [Event]) -> Notification<'a> {
        Notification {
            api_key: apikey,
            notifier: Notifier {
                name: NOTIFIER_NAME,
                version: NOTIFIER_VERSION,
                url: NOTIFIER_URL,
            },
            events: events,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Notification, NOTIFIER_NAME, NOTIFIER_VERSION, NOTIFIER_URL};
    use super::super::{event, exception, stacktrace, deviceinfo, Severity};
    use serde_test::{Token, assert_ser_tokens};

    #[test]
    fn test_notification_to_json() {
        let empty_vec = Vec::new();
        let notification = Notification::new("safe-api-key", &empty_vec);

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
        let frames = vec![stacktrace::Frame::new("test.rs", 400, "test", false)];
        let exceptions = vec![exception::Exception::new("Assert", "Assert", &frames)];
        let device = deviceinfo::DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let events = vec![event::Event::new(&exceptions, Severity::Error, None, &device, &app)];

        let notification = Notification::new("safe-api-key", &events);

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
                            Token::SeqStart(Some(1)),
                            Token::SeqSep,
                            Token::StructStart("Event", 4),
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
                            Token::StructStart("Frame", 4),
                            Token::StructSep,
                            Token::Str("file"),
                            Token::Str("test.rs"),
                            Token::StructSep,
                            Token::Str("lineNumber"),
                            Token::U32(400),
                            Token::StructSep,
                            Token::Str("method"),
                            Token::Str("test"),
                            Token::StructSep,
                            Token::Str("inProject"),
                            Token::Bool(false),
                            Token::StructEnd,
                            Token::SeqEnd,
                            Token::StructEnd,
                            Token::SeqEnd,
                            Token::StructSep,
                            Token::Str("severity"),
                            Token::EnumUnit("Severity", "error"),
                            Token::StructSep,
                            Token::Str("device"),
                            Token::StructStart("DeviceInfo", 2),
                            Token::StructSep,
                            Token::Str("osVersion"),
                            Token::Str("1.0.0"),
                            Token::StructSep,
                            Token::Str("hostname"),
                            Token::Str("testmachine"),
                            Token::StructEnd,
                            Token::StructEnd,
                            Token::SeqEnd,
                            Token::StructEnd]);
    }
}
