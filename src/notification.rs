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
    use super::{Notification, NOTIFIER_NAME, NOTIFIER_URL, NOTIFIER_VERSION};
    use super::super::{deviceinfo, event, exception, stacktrace};
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn test_notification_to_json() {
        let empty_vec = Vec::new();
        let notification = Notification::new("safe-api-key", &empty_vec);

        assert_ser_tokens(
            &notification,
            &[
                Token::Struct {
                    name: "Notification",
                    len: 3,
                },
                Token::Str("apiKey"),
                Token::Str("safe-api-key"),
                Token::Str("notifier"),
                Token::Struct {
                    name: "Notifier",
                    len: 3,
                },
                Token::Str("name"),
                Token::Str(NOTIFIER_NAME),
                Token::Str("version"),
                Token::Str(NOTIFIER_VERSION),
                Token::Str("url"),
                Token::Str(NOTIFIER_URL),
                Token::StructEnd,
                Token::Str("events"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_notification_with_event_to_json() {
        let frames = vec![stacktrace::Frame::new("test.rs", 400, "test", false)];
        let exceptions = vec![exception::Exception::new("Assert", "Assert", &frames)];
        let device = deviceinfo::DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let events = vec![
            event::Event::new(&exceptions, None, None, None, &device, &app),
        ];

        let notification = Notification::new("safe-api-key", &events);

        assert_ser_tokens(
            &notification,
            &[
                Token::Struct { name: "Notification", len: 3 },
                Token::Str("apiKey"),
                Token::Str("safe-api-key"),
                Token::Str("notifier"),
                Token::Struct { name: "Notifier", len: 3 },
                Token::Str("name"),
                Token::Str(NOTIFIER_NAME),
                Token::Str("version"),
                Token::Str(NOTIFIER_VERSION),
                Token::Str("url"),
                Token::Str(NOTIFIER_URL),
                Token::StructEnd,
                Token::Str("events"),
                Token::Seq { len: Some(1) },
                Token::Struct { name: "Event", len: 3 },
                Token::Str("payloadVersion"),
                Token::U32(event::PAYLOAD_VERSION),
                Token::Str("exceptions"),
                Token::Seq { len: Some(1) },
                Token::Struct { name: "Exception", len: 3 },
                Token::Str("errorClass"),
                Token::Str("Assert"),
                Token::Str("message"),
                Token::Str("Assert"),
                Token::Str("stacktrace"),
                Token::Seq { len: Some(1) },
                Token::Struct { name: "Frame", len: 4 },
                Token::Str("file"),
                Token::Str("test.rs"),
                Token::Str("lineNumber"),
                Token::U32(400),
                Token::Str("method"),
                Token::Str("test"),
                Token::Str("inProject"),
                Token::Bool(false),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("device"),
                Token::Struct { name: "DeviceInfo", len: 2 },
                Token::Str("osVersion"),
                Token::Str("1.0.0"),
                Token::Str("hostname"),
                Token::Str("testmachine"),
                Token::StructEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
