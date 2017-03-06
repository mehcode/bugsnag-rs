use super::bugsnag::Bugsnag;
use super::stacktrace;

use std::panic;

pub fn register_panic_handler(api: Bugsnag) {
    panic::set_hook(Box::new(move |info| {
        let message = match info.payload().downcast_ref::<&str>() {
            Some(msg) => msg,
            None => "unknown error!",
        };

        let stacktrace = stacktrace::create_stacktrace();

        api.notify("Panic", message, stacktrace);
    }));
}

pub fn unregister_panic_handler() {
    panic::take_hook();
}
