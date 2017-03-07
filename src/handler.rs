use super::bugsnag::{Bugsnag, Severity};
use super::stacktrace;

use std::panic;

pub fn register_panic_handler(api: Bugsnag) {
    panic::set_hook(Box::new(move |info| {
        let message = match info.payload().downcast_ref::<&str>() {
            Some(msg) => msg,
            None => "unknown error!",
        };

        let stacktrace = stacktrace::create_stacktrace(api.get_project_source_dir());

        api.notify("Panic", message, Severity::Error, &stacktrace, None);
    }));
}

pub fn unregister_panic_handler() {
    panic::take_hook();
}
