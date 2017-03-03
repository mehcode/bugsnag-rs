use super::stacktrace::Frame;

#[derive(Debug, RustcEncodable)]
pub struct Exception {
    errorClass: String,
    message: String,
    stacktrace: Vec<Frame>,
}

