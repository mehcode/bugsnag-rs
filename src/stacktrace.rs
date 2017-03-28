//! Module for creating a stacktrace in the Bugsnag format.
//!
//! # Example
//! ```
//! use bugsnag::stacktrace::create_stacktrace;
//! let stacktrace =
//!     create_stacktrace(Some(&|file, _| file.starts_with(env!("CARGO_MANIFEST_DIR"))));
//! assert!(!stacktrace.is_empty())
//! ```

use std::path::Path;
use backtrace::{self, Symbol};

/// Struct for storing the one frame of the stacktrace.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    file: String,
    line_number: u32,
    method: String,
    in_project: bool,
}

impl Frame {
    pub fn new(file: &str, linenumber: u32, method: &str, in_proj: bool) -> Frame {
        Frame {
            file: file.to_owned(),
            line_number: linenumber,
            method: method.to_owned(),
            in_project: in_proj,
        }
    }

    /// Converts from a backtrace::Symbol into a Frame
    ///
    /// # Arguments
    ///
    /// * `trace` - The backtrace::Symbol with all the information for the frame.
    /// * `in_project_func` - Function to check if a file and a function belongs to the project.
    pub fn from_symbol<F>(trace: &Symbol, in_project_func: Option<&F>) -> Frame
        where F: Fn(&str, &str) -> bool
    {
        let file = trace.filename()
            .unwrap_or_else(|| Path::new(""))
            .to_str()
            .unwrap_or("");
        let linenumber = trace.lineno().unwrap_or(0);
        let method = match trace.name() {
            Some(name) => name.to_string(),
            None => "unknown".to_string(),
        };

        let in_project = if let Some(func) = in_project_func {
            func(file, method.as_str())
        } else {
            false
        };

        Frame::new(file, linenumber, method.as_str(), in_project)
    }
}

/// Create a stacktrace and returns this stacktrace as vector of Frames
///
/// # Arguments
///
/// * `in_project` - A function that gets the following arguments (file, method). The function is
///                  used to determine if a file and method belongs to the project.
///
/// # Remarks
///
/// Bugsnag will use the information about a frame belonging to a project to hide
/// unnecessary information in the web interface.
pub fn create_stacktrace<F>(in_project: Option<&F>) -> Vec<Frame>
    where F: Fn(&str, &str) -> bool
{
    let mut result: Vec<Frame> = Vec::new();

    backtrace::trace(|frame| {
        backtrace::resolve(frame.ip(),
                           |symbol| result.push(Frame::from_symbol(&symbol, in_project)));
        true
    });

    result
}

#[cfg(test)]
mod tests {
    use super::{create_stacktrace, Frame};
    use serde_test::{Token, assert_ser_tokens};

    #[test]
    fn test_create_stacktrace() {
        let frames =
            create_stacktrace(Some(&|file, _| file.starts_with(env!("CARGO_MANIFEST_DIR"))));
        let mut found_frame = false;
        let file = file!();

        for frame in frames {
            if frame.method == "bugsnag::stacktrace::tests::test_create_stacktrace" {
                if frame.file.ends_with(file) {
                    if frame.in_project {
                        found_frame = true;
                        break;
                    }
                }
            }
        }

        assert!(found_frame);
    }

    #[test]
    fn test_frame_to_json() {
        let frame = Frame::new("test.rs", 500, "test_json", false);

        assert_ser_tokens(&frame,
                          &[Token::StructStart("Frame", 4),
                            Token::StructSep,
                            Token::Str("file"),
                            Token::Str("test.rs"),
                            Token::StructSep,
                            Token::Str("lineNumber"),
                            Token::U32(500),
                            Token::StructSep,
                            Token::Str("method"),
                            Token::Str("test_json"),
                            Token::StructSep,
                            Token::Str("inProject"),
                            Token::Bool(false),
                            Token::StructEnd]);
    }

    #[test]
    fn test_create_stacktrace_with_ignore() {
        let frames =
            create_stacktrace(&|_, method| !method.contains("test_create_stacktrace_with_ignore"));
        let mut found_frame = false;
        let file = file!();

        for frame in frames {
            if frame.method == "bugsnag::stacktrace::tests::test_create_stacktrace_with_ignore" {
                if frame.file.ends_with(file) {
                    if frame.in_project == false {
                        found_frame = true;
                        break;
                    }
                }
            }
        }

        assert!(found_frame);
    }
}
