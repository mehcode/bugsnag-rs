use super::{appinfo, deviceinfo, event, exception, notification, stacktrace};

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
    device_info: deviceinfo::DeviceInfo,
    app_info: Option<appinfo::AppInfo>,
    project_source_dir: String,
}

impl Bugsnag {
    /// Creates a new instance of the Bugsnag api
    pub fn new(api_key: &str, project_source_dir: &str) -> Bugsnag {
        Bugsnag {
            api_key: api_key.to_owned(),
            device_info: deviceinfo::DeviceInfo::generate(),
            app_info: None,
            project_source_dir: project_source_dir.to_owned(),
        }
    }

    /// Converts all data into the Bugsnag json formats and sends this json to
    /// the Bugsnag web interface.
    ///
    /// # Arguments
    ///
    /// * `methods_to_ignore` - A list of methods names. These methods are marked as not belonging
    ///                         to the project when the stacktrace is generated. The Bugsnag web
    ///                         interface will use this information to hide unnecessary data.
    ///                         To check if a method should be marked as not belonging to the
    ///                         project, the method name reported by the stacktrace is checked if it
    ///                         contains a method name in this list.
    pub fn notify(
        &self,
        error_class: &str,
        message: &str,
        severity: Severity,
        methods_to_ignore: Option<&[&str]>,
        context: Option<&str>,
    ) -> Result<(), Error> {
        let stacktrace = self.create_stacktrace(methods_to_ignore);
        let exceptions = vec![exception::Exception::new(error_class, message, &stacktrace)];
        let events = vec![
            event::Event::new(
                &exceptions,
                severity,
                context,
                &self.device_info,
                &self.app_info,
            ),
        ];
        let notification = notification::Notification::new(self.api_key.as_str(), &events);

        match serde_json::to_string(&notification) {
            Ok(json) => self.send(json.as_str()),
            Err(_) => Err(Error::JsonTransferFailed),
        }
    }

    fn create_stacktrace(&self, methods_to_ignore: Option<&[&str]>) -> Vec<stacktrace::Frame> {
        if let Some(ignore) = methods_to_ignore {
            let in_project_check = |file: &str, method: &str| {
                file.starts_with(self.project_source_dir.as_str())
                    && ignore
                        .iter()
                        .find(|check| !method.contains(*check))
                        .is_some()
            };

            stacktrace::create_stacktrace(&in_project_check)
        } else {
            let in_project_check =
                |file: &str, _: &str| file.starts_with(self.project_source_dir.as_str());


            stacktrace::create_stacktrace(&in_project_check)
        }
    }

    fn send(&self, json: &str) -> Result<(), Error> {
        match Client::new()
            .post(NOTIFY_URL)
            .header(ContentType::json())
            .body(json)
            .send()
        {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::JsonTransferFailed),
        }
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
    pub fn set_app_info(
        &mut self,
        version: Option<&str>,
        release_stage: Option<&str>,
        atype: Option<&str>,
    ) {
        self.app_info = Some(appinfo::AppInfo::new(version, release_stage, atype));
    }

    pub fn reset_app_info(&mut self) {
        self.app_info = None;
    }

    pub fn get_project_source_dir(&self) -> &String {
        &self.project_source_dir
    }
}

#[cfg(test)]
mod tests {
    use super::{Bugsnag, Severity};
    use serde_test::{assert_ser_tokens, Token};

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
    fn test_get_project_dir() {
        let api = Bugsnag::new("api-key", "my-dir");
        assert_eq!(api.get_project_source_dir(), "my-dir");
    }
}
