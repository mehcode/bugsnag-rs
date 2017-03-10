use super::{event, exception, notification, stacktrace, deviceinfo, appinfo};

use serde_json;

use hyper::Client;
use hyper::header::ContentType;

const NOTIFY_URL: &'static str = "http://notify.bugsnag.com";

#[derive(Debug, PartialEq)]
pub enum Error {
    /// The conversion to json failed.
    JsonConversionFailed,
    /// While transferring the json to Bugsnag, a problem occurred.
    /// This error does not reflect if Bugsnag rejected the json.
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
    app_info: Option<appinfo::AppInfo>,
}

impl Bugsnag {
    /// Creates a new instance of the Bugsnag api
    pub fn new(api_key: &str, proj_source_dir: Option<&str>) -> Bugsnag {
        Bugsnag {
            api_key: api_key.to_owned(),
            project_source_dir: proj_source_dir.map(|s| s.to_string()),
            device_info: deviceinfo::DeviceInfo::generate(),
            app_info: None,
        }
    }

    /// Converts all data into the Bugsnag json formats and sends this json to
    /// the Bugsnag web interface.
    pub fn notify(&self,
                  error_class: &str,
                  message: &str,
                  severity: Severity,
                  stacktrace: &[stacktrace::Frame],
                  context: Option<&str>)
                  -> Result<(), Error> {
        let exceptions = vec![exception::Exception::new(error_class, message, stacktrace)];
        let events = vec![event::Event::new(&exceptions,
                                            severity,
                                            context,
                                            &self.device_info,
                                            &self.app_info)];
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

    /// Returns the path to the project source directory
    pub fn get_project_source_dir(&self) -> &Option<String> {
        &self.project_source_dir
    }

    /// Sets information about the device. These information will be send to
    /// Bugsnag when notify is called.
    pub fn set_device_info(&mut self, hostname: Option<&str>, version: Option<&str>) {
        if let Some(name) = hostname {
            self.device_info.set_hostname(name);
        }

        if let Some(ver) = version {
            self.device_info.set_os_version(ver);
        }
    }

    /// Sets information about the application that uses this api. These information
    /// will be send to Bugsnag when notify is called.
    pub fn set_app_info(&mut self,
                        version: Option<&str>,
                        release_stage: Option<&str>,
                        atype: Option<&str>) {
        self.app_info = Some(appinfo::AppInfo::new(version, release_stage, atype));
    }

    pub fn reset_app_info(&mut self) {
        self.app_info = None;
    }
}

#[cfg(test)]
mod tests {
    use super::{Bugsnag, Severity};
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

    #[test]
    fn test_get_project_source_dir() {
        let api = Bugsnag::new("api-key", Some("my/project/path"));
        let source_dir = api.get_project_source_dir().as_ref().unwrap();
        assert_eq!(source_dir, "my/project/path");
    }
}
