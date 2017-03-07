use super::exception::Exception;
use super::bugsnag::Severity;
use super::deviceinfo::DeviceInfo;
use super::appinfo::AppInfo;

pub const PAYLOAD_VERSION: u32 = 2;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event<'a> {
    payload_version: u32,
    exceptions: &'a [Exception<'a>],
    severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<&'a str>,
    device: &'a DeviceInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    app: &'a Option<AppInfo>,
}

impl<'a> Event<'a> {
    pub fn new(exceptions: &'a [Exception],
               severity: Severity,
               context: Option<&'a str>,
               device: &'a DeviceInfo,
               app: &'a Option<AppInfo>)
               -> Event<'a> {
        Event {
            payload_version: PAYLOAD_VERSION,
            exceptions: exceptions,
            severity: severity,
            context: context,
            device: device,
            app: app,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, PAYLOAD_VERSION, Severity, DeviceInfo, AppInfo};
    use serde_test::{Token, assert_ser_tokens};

    #[test]
    fn test_event_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let evt = Event::new(&empty_vec, Severity::Error, None, &device, &app);

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
                            Token::Str("device"),
                            Token::StructStart("DeviceInfo", 2),
                            Token::StructSep,
                            Token::Str("osVersion"),
                            Token::Str("1.0.0"),
                            Token::StructSep,
                            Token::Str("hostname"),
                            Token::Str("testmachine"),
                            Token::StructEnd,
                            Token::StructEnd]);
    }

    #[test]
    fn test_event_with_context_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let evt = Event::new(&empty_vec,
                             Severity::Error,
                             Some("test/context"),
                             &device,
                             &app);

        assert_ser_tokens(&evt,
                          &[Token::StructStart("Event", 5),
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
                            Token::StructEnd]);
    }

    #[test]
    fn test_event_with_app_info_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = Some(AppInfo::new(Some("1.0.0"), Some("test"), Some("rust")));
        let evt = Event::new(&empty_vec, Severity::Error, None, &device, &app);

        assert_ser_tokens(&evt,
                          &[Token::StructStart("Event", 5),
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
                            Token::Str("device"),
                            Token::StructStart("DeviceInfo", 2),
                            Token::StructSep,
                            Token::Str("osVersion"),
                            Token::Str("1.0.0"),
                            Token::StructSep,
                            Token::Str("hostname"),
                            Token::Str("testmachine"),
                            Token::StructEnd,
                            Token::StructSep,
                            Token::Str("app"),
                            Token::Option(true),
                            Token::StructStart("AppInfo", 3),
                            Token::StructSep,
                            Token::Str("version"),
                            Token::Option(true),
                            Token::Str("1.0.0"),
                            Token::StructSep,
                            Token::Str("releaseStage"),
                            Token::Option(true),
                            Token::Str("test"),
                            Token::StructSep,
                            Token::Str("type"),
                            Token::Option(true),
                            Token::Str("rust"),
                            Token::StructEnd,
                            Token::StructEnd]);
    }
}
