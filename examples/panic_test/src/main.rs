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

fn register_panic_handler_with_global_instance(api: bugsnag::Bugsnag) {
    api.to_global_instance();

    panic::set_hook(Box::new(|info| {
        let message = match info.payload().downcast_ref::<&str>() {
            Some(msg) => msg,
            None => "unknown error!",
        };

        if let Some(api_mtx) = bugsnag::Bugsnag::global_instance() {
            if let Ok(api) = api_mtx.lock() {
                let stacktrace = stacktrace::create_stacktrace(api.get_project_source_dir());

                if api.notify("Panic",
                            message,
                            bugsnag::Severity::Error,
                            &stacktrace,
                            None)
                    .is_err() {
                    println!("Error at notifying bugsnag!");
                }
            }
        }
    }));
}

fn main() {
    let mut api = bugsnag::Bugsnag::new("api-key", Some(env!("CARGO_MANIFEST_DIR"))).unwrap();
    api.set_app_info(Some(env!("CARGO_PKG_VERSION")),
                     Some("development"),
                     Some("rust"));

    //register_panic_handler(api);
    register_panic_handler_with_global_instance(api);

    panic!("Hello from a Rust panic!");
}
