use super::{event, exception, notification, stacktrace, deviceinfo, appinfo, globalinstance};

use std::sync::{Arc, Mutex};

use serde_json;

use hyper::Client;
use hyper::header::ContentType;

const NOTIFY_URL: &'static str = "http://notify.bugsnag.com";

#[derive(Debug, PartialEq)]
pub enum Error {
    JsonConversionFailed,
    JsonTransferFailed,
    GlobalInstanceExists,
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
    pub fn new(api_key: &str, proj_source_dir: Option<&str>) -> Result<Bugsnag, Error> {
        if let Some(_) = Bugsnag::global_instance() {
            return Err(Error::GlobalInstanceExists);
        }

        Ok(Bugsnag {
            api_key: api_key.to_owned(),
            project_source_dir: proj_source_dir.map(|s| s.to_string()),
            device_info: deviceinfo::DeviceInfo::generate(),
            app_info: None,
        })
    }

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

    pub fn set_app_info(&mut self,
                        version: Option<&str>,
                        release_stage: Option<&str>,
                        atype: Option<&str>) {
        self.app_info = Some(appinfo::AppInfo::new(version, release_stage, atype));
    }

    pub fn reset_app_info(&mut self) {
        self.app_info = None;
    }

    pub fn global_instance() -> Option<Arc<Mutex<Bugsnag>>> {
        match globalinstance::get().lock() {
            Ok(mut res) => res.instance(),
            Err(_) => None,
        }
    }

    pub fn reset_global_instance() {
        if let Ok(mut res) = globalinstance::get().lock() {
            res.reset_instance()
        }
    }

    pub fn to_global_instance(self) {
        if let Ok(mut res) = globalinstance::get().lock() {
            if !res.has_instance() {
                res.set_instance(self)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Bugsnag, Severity, Error};
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
    fn test_global_instance_create() {
        let bugsnag = Bugsnag::new("my-api-key", None).unwrap();
        bugsnag.to_global_instance();

        let instance = Bugsnag::global_instance();
        assert!(instance.is_some());

        let instance_unpacked = instance.unwrap();
        assert!(instance_unpacked.lock().is_ok());

        Bugsnag::reset_global_instance();
    }

    #[test]
    fn test_new_with_existing_global_instance() {
        let bugsnag = Bugsnag::new("my-api-key", None).unwrap();
        bugsnag.to_global_instance();

        assert_eq!(Bugsnag::new("my-second-api-key", None).err().unwrap(),
                   Error::GlobalInstanceExists);
        Bugsnag::reset_global_instance();
    }

    #[test]
    fn test_global_instance_reset() {
        let bugsnag = Bugsnag::new("my-api-key", None).unwrap();
        bugsnag.to_global_instance();
        Bugsnag::reset_global_instance();

        assert!(Bugsnag::global_instance().is_none());
    }

    #[test]
    fn test_new_after_global_instance_reset() {
        let bugsnag = Bugsnag::new("my-api-key", None).unwrap();
        bugsnag.to_global_instance();
        Bugsnag::reset_global_instance();

        assert!(Bugsnag::global_instance().is_none());
        let bugsnag_option = Bugsnag::new("my-api-key", Some("path"));
        assert!(bugsnag_option.is_ok());
        assert!(bugsnag_option.unwrap().get_project_source_dir().is_some());
        Bugsnag::reset_global_instance();
    }
}
