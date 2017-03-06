use super::{event, exception, notification, stacktrace};

use serde_json;

use hyper::Client;
use hyper::header::ContentType;

const NOTIFY_URL: &'static str = "http://notify.bugsnag.com";

pub struct Bugsnag {
    api_key: String,
}

impl Bugsnag {
    pub fn new(api_key: &str) -> Bugsnag {
        Bugsnag { api_key: api_key.to_owned() }
    }

    pub fn notify(&self, error_class: &str, message: &str, stacktrace: Vec<stacktrace::Frame>) {
        let exception = exception::Exception::new(error_class, message, stacktrace);
        let event = event::Event::new(vec![exception]);
        let notification = notification::Notification::new(self.api_key.as_str(), vec![event]);

        if let Ok(json) = serde_json::to_string(&notification) {
            self.send(json.as_str());
        }
    }

    fn send(&self, json: &str) {
        Client::new()
            .post(NOTIFY_URL)
            .header(ContentType::json())
            .body(json)
            .send();
    }
}
