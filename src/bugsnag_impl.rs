use super::{appinfo, deviceinfo, event, exception, notification, stacktrace};

use std::fmt;
use std::error::Error as StdError;

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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &'static str {
        match *self {
            Error::JsonConversionFailed => "conversion to json failed",
            Error::JsonTransferFailed => {
                "while transferring the json to Bugsnag, a problem occurred"
            }
        }
    }
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

/// Builder for creating the notification that will be send to Bugsnag.
/// If the object is dropped, the notification is send to Bugsnag.
pub struct NotifyBuilder<'a, 'bugsnag> {
    bugsnag: &'bugsnag Bugsnag,
    error_class: &'a str,
    message: &'a str,
    send_executed: bool,
    methods_to_ignore: Option<&'a [&'a str]>,
    context: Option<&'a str>,
    severity: Option<Severity>,
    grouping_hash: Option<&'a str>,
}

impl<'a, 'bugsnag> NotifyBuilder<'a, 'bugsnag> {
    fn new(
        bugsnag: &'bugsnag Bugsnag,
        error_class: &'a str,
        message: &'a str,
    ) -> NotifyBuilder<'a, 'bugsnag> {
        NotifyBuilder {
            bugsnag,
            error_class,
            message,
            send_executed: false,
            methods_to_ignore: None,
            context: None,
            severity: None,
            grouping_hash: None,
        }
    }

    /// Sets a list of methods that should be marked as not belonging
    /// to the project when the stacktrace is generated. The Bugsnag web
    /// interface will use this information to hide unnecessary data.
    /// To check if a method should be marked as not belonging to the
    /// project, the method name reported by the stacktrace is checked if it
    /// contains a method name in this list.
    pub fn methods_to_ignore(mut self, val: &'a [&'a str]) -> Self {
        self.methods_to_ignore = Some(val);
        self
    }

    /// Sets a context that describes the state of the application while the error occurred.
    pub fn context(mut self, val: &'a str) -> Self {
        self.context = Some(val);
        self
    }

    /// Sets the severity of the error.
    pub fn severity(mut self, val: Severity) -> Self {
        self.severity = Some(val);
        self
    }

    /// Sets the grouping hash for the Bugsnag web interface.
    pub fn grouping_hash(mut self, val: &'a str) -> Self {
        self.grouping_hash = Some(val);
        self
    }

    /// Call this function to explicitly send the notification to Bugsnag.
    /// This function will be called implicit if this object is dropped, but the notification will
    /// not be send twice.
    pub fn send(&mut self) -> Result<(), Error> {
        if self.send_executed {
            return Ok(());
        }

        self.send_executed = true;

        let json = self.prepare_json()?;
        self.bugsnag.send(&json)
    }

    /// Prepares the json as string
    fn prepare_json(&self) -> Result<String, Error> {
        let stacktrace = self.bugsnag.create_stacktrace(self.methods_to_ignore);
        let exceptions = vec![
            exception::Exception::new(self.error_class, self.message, &stacktrace),
        ];
        let events = vec![
            event::Event::new(
                &exceptions,
                self.severity.as_ref(),
                self.context,
                self.grouping_hash,
                &self.bugsnag.device_info,
                &self.bugsnag.app_info,
            ),
        ];
        let notification = notification::Notification::new(&self.bugsnag.api_key, &events);

        match serde_json::to_string(&notification) {
            Ok(json) => Ok(json),
            Err(_) => Err(Error::JsonConversionFailed),
        }
    }
}

impl<'a, 'bugsnag> Drop for NotifyBuilder<'a, 'bugsnag> {
    fn drop(&mut self) {
        let _ = self.send();
    }
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

    /// Notifies the Bugsnag web-interface about an error.
    /// The function returns a builder to provide more information about the error.
    pub fn notify<'a, 'bugsnag>(
        &'bugsnag self,
        error_class: &'a str,
        message: &'a str,
    ) -> NotifyBuilder<'a, 'bugsnag> {
        NotifyBuilder::new(&self, error_class, message)
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

    /// Send a json string to the Bugsnag endpoint
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

        assert_ser_tokens(
            &severity,
            &[
                Token::UnitVariant {
                    name: "Severity",
                    variant: "error",
                },
            ],
        );
    }

    #[test]
    fn test_info_to_json() {
        let severity = Severity::Info;

        assert_ser_tokens(
            &severity,
            &[
                Token::UnitVariant {
                    name: "Severity",
                    variant: "info",
                },
            ],
        );
    }

    #[test]
    fn test_warning_to_json() {
        let severity = Severity::Warning;

        assert_ser_tokens(
            &severity,
            &[
                Token::UnitVariant {
                    name: "Severity",
                    variant: "warning",
                },
            ],
        );
    }

    #[test]
    fn test_get_project_dir() {
        let api = Bugsnag::new("api-key", "my-dir");
        assert_eq!(api.get_project_source_dir(), "my-dir");
    }
}
