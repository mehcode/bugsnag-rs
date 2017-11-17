use super::{Bugsnag, Error, Severity};

use std::panic::PanicInfo;

pub fn handle(
    api: &Bugsnag,
    info: &PanicInfo,
    methods_to_ignore: Option<&[&str]>,
) -> Result<(), Error> {
    let message = if let Some(data) = info.payload().downcast_ref::<String>() {
        data.to_owned()
    } else if let Some(data) = info.payload().downcast_ref::<&str>() {
        (*data).to_owned()
    } else {
        format!("Error: {:?}", info.payload())
    };

    api.notify(
        "Panic",
        message.as_str(),
        Severity::Error,
        methods_to_ignore,
        None,
    )
}
