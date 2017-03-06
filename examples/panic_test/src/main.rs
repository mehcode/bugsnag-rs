extern crate bugsnag_api;

use bugsnag_api::{handler, bugsnag};

fn main() {
    let api = bugsnag::Bugsnag::new("add-api-key", Some(env!("CARGO_MANIFEST_DIR").to_string()));

    handler::register_panic_handler(api);

    panic!("Hello from a Rust panic!");
}
