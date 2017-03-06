use super::stacktrace::Frame;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Exception {
    error_class: String,
    message: String,
    stacktrace: Vec<Frame>,
}

impl Exception {
    pub fn new(errorclass: &str, message: &str, stacktrace: Vec<Frame>) -> Exception {
        Exception {
            error_class: errorclass.to_owned(),
            message: message.to_owned(),
            stacktrace: stacktrace,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Exception;
    use serde_test::{Token, assert_ser_tokens};

    #[test]
    fn test_exception_to_json() {
        let ex = Exception::new("Assert", "Assert", Vec::new());

        assert_ser_tokens(&ex,
                          &[Token::StructStart("Exception", 3),
                            Token::StructSep,
                            Token::Str("errorClass"),
                            Token::Str("Assert"),
                            Token::StructSep,
                            Token::Str("message"),
                            Token::Str("Assert"),
                            Token::StructSep,
                            Token::Str("stacktrace"),
                            Token::SeqStart(Some(0)),
                            Token::SeqEnd,
                            Token::StructEnd]);
    }
}
