//! An example showing the integration of the bugsnag api for a panic handler.
//!
//! This simple implementations consumes the api object. So, we can not change
//! any parameters after registering the panic handler.

extern crate bugsnag;

use std::panic;

/// The panic handler consumes the api object, so no further modifications to
/// the object are possible
fn register_panic_handler(api: bugsnag::Bugsnag) {
    panic::set_hook(Box::new(move |info| {
        if bugsnag::panic::handle(&api, &info, Some(&["register_panic_handler"])).is_err() {
            println!("Error at notifying bugsnag!");
        }
    }));
}

fn test() -> Option<i32> {
    None
}

fn main() {
    let mut api =
        bugsnag::Bugsnag::new("api-key", concat!(env!("CARGO_MANIFEST_DIR"), "/examples"));
    api.set_app_info(
        Some(env!("CARGO_PKG_VERSION")),
        Some("development"),
        Some("rust"),
    );

    register_panic_handler(api);

    test().unwrap();

    panic!("Hello from a Rust panic!");
}
