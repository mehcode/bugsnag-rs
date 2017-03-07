use super::{event, exception, notification, stacktrace, deviceinfo};

use serde_json;

use hyper::Client;
use hyper::header::ContentType;

const NOTIFY_URL: &'static str = "http://notify.bugsnag.com";

pub enum Error {
    JsonConversionFailed,
    JsonTransferFailed,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

pub struct Bugsnag {
    api_key: String,
    project_source_dir: Option<String>,
    device_info: deviceinfo::DeviceInfo,
}

impl Bugsnag {
    pub fn new(api_key: &str, proj_source_dir: Option<String>) -> Bugsnag {
        Bugsnag {
            api_key: api_key.to_owned(),
            project_source_dir: proj_source_dir,
            device_info: deviceinfo::DeviceInfo::generate(),
        }
    }

    pub fn notify(&self,
                  error_class: &str,
                  message: &str,
                  severity: Severity,
                  stacktrace: &[stacktrace::Frame],
                  context: Option<&str>)
                  -> Result<(), Error> {
        let exceptions = vec![exception::Exception::new(error_class, message, stacktrace)];
        let events = vec![event::Event::new(&exceptions, severity, context, &self.device_info)];
        let notification = notification::Notification::new(self.api_key.as_str(), &events);

        match serde_json::to_string(&notification) {
            Ok(json) => self.send(json.as_str()),
            Err(_) => Err(Error::JsonTransferFailed),
        }
    }

    fn send(&self, json: &str) -> Result<(), Error> {
        match Client::new()
            .post(NOTIFY_URL)
            .header(ContentType::json())
            .body(json)
            .send() {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::JsonTransferFailed),
        }
    }

    pub fn get_project_source_dir(&self) -> &Option<String> {
        &self.project_source_dir
    }

    pub fn set_device_info(&mut self, hostname: Option<&str>, version: Option<&str>) {
        if let Some(name) = hostname {
            self.device_info.set_hostname(name);
        }

        if let Some(ver) = version {
            self.device_info.set_os_version(ver);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Severity;
    use serde_test::{Token, assert_ser_tokens};

    #[test]
    fn test_error_to_json() {
        let severity = Severity::Error;

        assert_ser_tokens(&severity, &[Token::EnumUnit("Severity", "error")]);
    }

    #[test]
    fn test_info_to_json() {
        let severity = Severity::Info;

        assert_ser_tokens(&severity, &[Token::EnumUnit("Severity", "info")]);
    }

    #[test]
    fn test_warning_to_json() {
        let severity = Severity::Warning;

        assert_ser_tokens(&severity, &[Token::EnumUnit("Severity", "warning")]);
    }
}
