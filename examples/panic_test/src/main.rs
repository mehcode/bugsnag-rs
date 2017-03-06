extern crate bugsnag_api;

use bugsnag_api::{handler, bugsnag};

fn main() {
    let api = bugsnag::Bugsnag::new("enter-api-key");

    handler::register_panic_handler(api);

    panic!("Hello from a Rust panic!");
}
