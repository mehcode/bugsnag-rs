use super::{event, exception, notification, stacktrace};

use serde_json;

use hyper::Client;
use hyper::header::ContentType;

const NOTIFY_URL: &'static str = "http://notify.bugsnag.com";

pub enum Error {
    JsonConversionFailed,
    JsonTransferFailed,
}

pub struct Bugsnag {
    api_key: String,
    project_source_dir: Option<String>,
}

impl Bugsnag {
    pub fn new(api_key: &str, proj_source_dir: Option<String>) -> Bugsnag {
        Bugsnag {
            api_key: api_key.to_owned(),
            project_source_dir: proj_source_dir,
        }
    }

    pub fn notify(&self,
                  error_class: &str,
                  message: &str,
                  stacktrace: &[stacktrace::Frame])
                  -> Result<(), Error> {
        let exceptions = vec![exception::Exception::new(error_class, message, stacktrace)];
        let events = vec![event::Event::new(&exceptions)];
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
}
