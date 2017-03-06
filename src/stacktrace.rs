use std::path::Path;
use backtrace::{BacktraceSymbol, Backtrace};

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

    pub fn from_symbol(trace: &BacktraceSymbol, proj_source_dir: &Option<String>) -> Frame {
        let file = trace.filename()
            .unwrap_or(Path::new(""))
            .to_str()
            .unwrap_or("");
        let linenumber = trace.lineno().unwrap_or(0);
        let method = match trace.name() {
            Some(name) => name.as_str().unwrap_or("unknown"),
            None => "unknown",
        };

        let in_project = match *proj_source_dir {
            Some(ref dir) => file.starts_with(dir.as_str()),
            None => false,
        };

        Frame::new(file, linenumber, method, in_project)
    }
}

pub fn create_stacktrace(proj_source_dir: &Option<String>) -> Vec<Frame> {
    let trace = Backtrace::new();
    let mut result: Vec<Frame> = Vec::new();

    for frame in trace.frames() {
        // as one frame can have multiple symbols, we treat each symbol as
        // one frame.
        for symbol in frame.symbols() {
            result.push(Frame::from_symbol(symbol, &proj_source_dir));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{create_stacktrace, Frame};
    use serde_test::{Token, assert_ser_tokens};

    #[test]
    fn test_create_stacktrace() {
        let frames = create_stacktrace(&None);
        let mut found_frame = false;
        let file = file!();

        for frame in frames {
            if frame.method == "bugsnag_api::stacktrace::tests::test_create_stacktrace" {
                if frame.file.ends_with(file) {
                    found_frame = true;
                    break;
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
