extern crate bugsnag_api;

use bugsnag_api::{handler, bugsnag};

fn main() {
    let mut api = bugsnag::Bugsnag::new("add-api-key",
                                        Some(env!("CARGO_MANIFEST_DIR").to_string()));
    api.set_app_info(Some(env!("CARGO_PKG_VERSION")),
                     Some("development"),
                     Some("rust"));

    handler::register_panic_handler(api);

    panic!("Hello from a Rust panic!");
}
