use super::exception::Exception;
use super::Severity;
use super::deviceinfo::DeviceInfo;
use super::appinfo::AppInfo;

pub const PAYLOAD_VERSION: u32 = 4;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event<'a> {
    payload_version: u32,
    exceptions: &'a [Exception<'a>],
    #[serde(skip_serializing_if = "Option::is_none")] severity: Option<&'a Severity>,
    #[serde(skip_serializing_if = "Option::is_none")] context: Option<&'a str>,
    device: &'a DeviceInfo,
    #[serde(skip_serializing_if = "Option::is_none")] app: &'a Option<AppInfo>,
    #[serde(skip_serializing_if = "Option::is_none")] grouping_hash: Option<&'a str>,
}

impl<'a> Event<'a> {
    pub fn new(
        exceptions: &'a [Exception],
        severity: Option<&'a Severity>,
        context: Option<&'a str>,
        grouping_hash: Option<&'a str>,
        device: &'a DeviceInfo,
        app: &'a Option<AppInfo>,
    ) -> Event<'a> {
        Event {
            payload_version: PAYLOAD_VERSION,
            exceptions,
            severity,
            context,
            device,
            app,
            grouping_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AppInfo, DeviceInfo, Event, Severity, PAYLOAD_VERSION};
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn test_event_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let evt = Event::new(
            &empty_vec,
            Some(&Severity::Error),
            None,
            None,
            &device,
            &app,
        );

        assert_ser_tokens(
            &evt,
            &[
                Token::Struct {
                    name: "Event",
                    len: 4,
                },
                Token::Str("payloadVersion"),
                Token::U32(PAYLOAD_VERSION),
                Token::Str("exceptions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("severity"),
                Token::Some,
                Token::UnitVariant {
                    name: "Severity",
                    variant: "error",
                },
                Token::Str("device"),
                Token::Struct {
                    name: "DeviceInfo",
                    len: 2,
                },
                Token::Str("osVersion"),
                Token::Str("1.0.0"),
                Token::Str("hostname"),
                Token::Str("testmachine"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_event_with_context_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let evt = Event::new(
            &empty_vec,
            Some(&Severity::Error),
            Some("test/context"),
            None,
            &device,
            &app,
        );

        assert_ser_tokens(
            &evt,
            &[
                Token::Struct {
                    name: "Event",
                    len: 5,
                },
                Token::Str("payloadVersion"),
                Token::U32(PAYLOAD_VERSION),
                Token::Str("exceptions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("severity"),
                Token::Some,
                Token::UnitVariant {
                    name: "Severity",
                    variant: "error",
                },
                Token::Str("context"),
                Token::Some,
                Token::Str("test/context"),
                Token::Str("device"),
                Token::Struct {
                    name: "DeviceInfo",
                    len: 2,
                },
                Token::Str("osVersion"),
                Token::Str("1.0.0"),
                Token::Str("hostname"),
                Token::Str("testmachine"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_event_with_app_info_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = Some(AppInfo::new(Some("1.0.0"), Some("test"), Some("rust")));
        let evt = Event::new(
            &empty_vec,
            Some(&Severity::Error),
            None,
            None,
            &device,
            &app,
        );

        assert_ser_tokens(
            &evt,
            &[
                Token::Struct {
                    name: "Event",
                    len: 5,
                },
                Token::Str("payloadVersion"),
                Token::U32(PAYLOAD_VERSION),
                Token::Str("exceptions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("severity"),
                Token::Some,
                Token::UnitVariant {
                    name: "Severity",
                    variant: "error",
                },
                Token::Str("device"),
                Token::Struct {
                    name: "DeviceInfo",
                    len: 2,
                },
                Token::Str("osVersion"),
                Token::Str("1.0.0"),
                Token::Str("hostname"),
                Token::Str("testmachine"),
                Token::StructEnd,
                Token::Str("app"),
                Token::Some,
                Token::Struct {
                    name: "AppInfo",
                    len: 3,
                },
                Token::Str("version"),
                Token::Some,
                Token::Str("1.0.0"),
                Token::Str("releaseStage"),
                Token::Some,
                Token::Str("test"),
                Token::Str("type"),
                Token::Some,
                Token::Str("rust"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
