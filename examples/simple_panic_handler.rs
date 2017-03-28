//! An example showing the integration of the bugsnag api for a panic handler.
//!
//! This simple implementations consumes the api object. So, we can not change
//! any parameters after registering the panic handler.

extern crate bugsnag;

use bugsnag::stacktrace;

use std::panic;

/// The panic handler consumes the api object, so no further modifications to
/// the object are possible
fn register_panic_handler(api: bugsnag::Bugsnag) {
    panic::set_hook(Box::new(move |info| {
        let message = if info.payload().is::<String>() {
            info.payload().downcast_ref::<String>().unwrap().as_str()
        } else if info.payload().is::<&str>() {
            info.payload().downcast_ref::<&str>().unwrap()
        } else {
            "Unknown error!"
        };

        let project_path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples");
        let stacktrace = stacktrace::create_stacktrace(Some(&|file, method| {
            file.starts_with(project_path) && !method.contains("register_panic_handler")
        }));

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

fn test() -> Option<i32> { 
    None
}

fn main() {
    let mut api = bugsnag::Bugsnag::new("api-key", concat!(env!("CARGO_MANIFEST_DIR"), "/examples"));
    api.set_app_info(Some(env!("CARGO_PKG_VERSION")),
                     Some("development"),
                     Some("rust"));

    register_panic_handler(api);

    test().unwrap();

    panic!("Hello from a Rust panic!");
}
