//! An example showing the integration of the bugsnag api using a global instance object
//!
//! We provide a class for wrapping the api object in the global state by using
//! the lazy_static crate.

extern crate bugsnag_api;
#[macro_use]
extern crate lazy_static;

use bugsnag_api::{bugsnag, stacktrace};

use std::panic;
use std::sync::{Arc, Mutex};

/// The wrapper for storing the global instance
struct GlobalInstanceWrapper {
    instance: Option<Arc<Mutex<bugsnag::Bugsnag>>>,
}

impl GlobalInstanceWrapper {
    pub fn new() -> GlobalInstanceWrapper {
        GlobalInstanceWrapper { instance: None }
    }

    pub fn instance(&mut self) -> Option<Arc<Mutex<bugsnag::Bugsnag>>> {
        self.instance.clone()
    }

    pub fn set_instance(&mut self, instance: bugsnag::Bugsnag) {
        self.instance = Some(Arc::new(Mutex::new(instance)))
    }
}

// The global instance that holds our wrapper
lazy_static! {
    static ref INSTANCE: Mutex<GlobalInstanceWrapper> = Mutex::new(GlobalInstanceWrapper::new());
}

/// Returns the global api object
/// To be accessible by this function, the api object needs to be registered
/// with the 'to_global_instance' function!
pub fn global_instance() -> Option<Arc<Mutex<bugsnag::Bugsnag>>> {
    match INSTANCE.lock() {
        Ok(mut res) => res.instance(),
        Err(_) => None,
    }
}

/// Consumes the api object and registers this object as the global api object
pub fn to_global_instance(api: bugsnag::Bugsnag) {
    if let Ok(mut res) = INSTANCE.lock() {
        res.set_instance(api);
    }
}

/// Converts our api object to the global api object and registers the panic
/// handler. This panic handler will use the global api object, if called.
fn register_panic_handler_with_global_instance(api: bugsnag::Bugsnag) {
    to_global_instance(api);

    panic::set_hook(Box::new(|info| {
        let message = match info.payload().downcast_ref::<&str>() {
            Some(msg) => msg,
            None => "unknown error!",
        };

        if let Some(api_mtx) = global_instance() {
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
    let mut api = bugsnag::Bugsnag::new("api-key", Some(env!("CARGO_MANIFEST_DIR")));
    api.set_app_info(Some(env!("CARGO_PKG_VERSION")),
                     Some("development"),
                     Some("rust"));

    register_panic_handler_with_global_instance(api);

    panic!("Hello from a Rust panic!");
}
