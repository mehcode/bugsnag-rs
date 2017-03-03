use std::convert::From;
use std::path::Path;
use backtrace::{Symbol, BacktraceSymbol, Backtrace};

#[derive(Debug, RustcEncodable)]
pub struct Frame {
    file: String,
    lineNumber: u32,
    method: String,
}

impl<'a> From<&'a BacktraceSymbol> for Frame {
    fn from(trace: &'a BacktraceSymbol) -> Frame {
        let file = trace.filename()
            .unwrap_or(Path::new(""))
            .to_str()
            .unwrap_or("");
        let linenumber = trace.lineno().unwrap_or(0);
        let method = match trace.name() {
            Some(name) => name.as_str().unwrap_or("unknown"),
            None => "unknown",
        };

        Frame {
            file: file.to_string(),
            lineNumber: linenumber,
            method: method.to_string(),
        }
    }
}

pub fn create_stacktrace() -> Vec<Frame> {
    let trace = Backtrace::new();
    let mut result: Vec<Frame> = Vec::new();

    for frame in trace.frames() {
        // as one frame can have multiple symbols, we treat each symbol as
        // one frame.
        for symbol in frame.symbols() {
            result.push(Frame::from(symbol));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{create_stacktrace, Frame};
    use rustc_serialize::json::{self, Json};

    #[test]
    fn test_create_stacktrace() {
        let frames = create_stacktrace();
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
        let frame = Frame {
            file: "test.rs".to_string(),
            lineNumber: 500,
            method: "test_json".to_string(),
        };
        let frame_json = Json::from_str(json::encode(&frame).unwrap().as_str()).unwrap();

        assert_eq!(frame_json.find("file").unwrap().as_string().unwrap(),
                   "test.rs");
        assert_eq!(frame_json.find("lineNumber").unwrap().as_u64().unwrap(),
                   500);
        assert_eq!(frame_json.find("method").unwrap().as_string().unwrap(),
                   "test_json");
    }
}
