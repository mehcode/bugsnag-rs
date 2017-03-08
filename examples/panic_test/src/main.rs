extern crate bugsnag_api;

use bugsnag_api::{bugsnag, stacktrace};

use std::panic;

fn register_panic_handler(api: bugsnag::Bugsnag) {
    panic::set_hook(Box::new(move |info| {
        let message = match info.payload().downcast_ref::<&str>() {
            Some(msg) => msg,
            None => "unknown error!",
        };

        let stacktrace = stacktrace::create_stacktrace(api.get_project_source_dir());

        api.notify("Panic",
                   message,
                   bugsnag::Severity::Error,
                   &stacktrace,
                   None);
    }));
}

fn main() {
    let mut api = bugsnag::Bugsnag::new("add-api-key",
                                        Some(env!("CARGO_MANIFEST_DIR").to_string()));
    api.set_app_info(Some(env!("CARGO_PKG_VERSION")),
                     Some("development"),
                     Some("rust"));

    register_panic_handler(api);

    panic!("Hello from a Rust panic!");
}
