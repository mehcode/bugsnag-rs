use super::bugsnag::Bugsnag;

use std::sync::{Arc, Mutex};

pub struct Wrapper {
    instance: Option<Arc<Mutex<Bugsnag>>>,
}

impl Wrapper {
    pub fn new() -> Wrapper {
        Wrapper { instance: None }
    }

    pub fn instance(&mut self) -> Option<Arc<Mutex<Bugsnag>>> {
        self.instance.clone()
    }

    pub fn set_instance(&mut self, instance: Bugsnag) {
        self.instance = Some(Arc::new(Mutex::new(instance)))
    }

    pub fn reset_instance(&mut self) {
        self.instance = None
    }

    pub fn has_instance(&self) -> bool {
        self.instance.is_some()
    }
}

lazy_static! {
    static ref INSTANCE: Mutex<Wrapper> = Mutex::new(Wrapper::new());
}

pub fn get() -> &'static Mutex<Wrapper> {
    &INSTANCE
}
