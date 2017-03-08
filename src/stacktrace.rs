use std::path::Path;
use backtrace::{self, Symbol};

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

    pub fn from_symbol(trace: &Symbol, proj_source_dir: &Option<String>) -> Frame {
        let file = trace.filename()
            .unwrap_or_else(|| Path::new(""))
            .to_str()
            .unwrap_or("");
        let linenumber = trace.lineno().unwrap_or(0);
        let method = match trace.name() {
            Some(name) => name.to_string(),
            None => "unknown".to_string(),
        };

        let in_project = match *proj_source_dir {
            Some(ref dir) => file.starts_with(dir.as_str()),
            None => false,
        };

        Frame::new(file, linenumber, method.as_str(), in_project)
    }
}

pub fn create_stacktrace(proj_source_dir: &Option<String>) -> Vec<Frame> {
    let mut result: Vec<Frame> = Vec::new();

    backtrace::trace(|frame| {
        backtrace::resolve(frame.ip(),
                           |symbol| result.push(Frame::from_symbol(&symbol, proj_source_dir)));
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
        let frames = create_stacktrace(&Some(env!("CARGO_MANIFEST_DIR").to_string()));
        let mut found_frame = false;
        let file = file!();

        for frame in frames {
            if frame.method == "bugsnag_api::stacktrace::tests::test_create_stacktrace" {
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
}
