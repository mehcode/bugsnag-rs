//! An example showing the integration of the bugsnag api for a panic handler.
//!
//! This simple implementations consumes the api object. So, we can not change
//! any parameters after registering the panic handler.
extern crate bugsnag_api;

use bugsnag_api::{bugsnag, stacktrace};

use std::panic;

/// The panic handler consumes the api object, so no further modifications to
/// the object are possible
fn register_panic_handler(api: bugsnag::Bugsnag) {
    panic::set_hook(Box::new(move |info| {
        let message = match info.payload().downcast_ref::<&str>() {
            Some(msg) => msg,
            None => "unknown error!",
        };

        let stacktrace = stacktrace::create_stacktrace(api.get_project_source_dir());

        if api.notify("Panic",
                    message,
                    bugsnag::Severity::Error,
                    &stacktrace,
                    None)
            .is_err() {
            println!("Error at notifying bugsnag!");
        }
    }));
}

fn main() {
    let mut api = bugsnag::Bugsnag::new("api-key", Some(env!("CARGO_MANIFEST_DIR")));
    api.set_app_info(Some(env!("CARGO_PKG_VERSION")),
                     Some("development"),
                     Some("rust"));

    register_panic_handler(api);

    panic!("Hello from a Rust panic!");
}
