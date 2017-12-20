use super::stacktrace::Frame;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Exception<'a> {
    error_class: &'a str,
    message: &'a str,
    stacktrace: &'a [Frame],
}

impl<'a> Exception<'a> {
    pub fn new(errorclass: &'a str, message: &'a str, stacktrace: &'a [Frame]) -> Exception<'a> {
        Exception {
            error_class: errorclass,
            message: message,
            stacktrace: stacktrace,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Exception;
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn test_exception_to_json() {
        let empty_vec = Vec::new();
        let ex = Exception::new("Assert", "Assert", &empty_vec);

        assert_ser_tokens(
            &ex,
            &[
                Token::Struct {
                    name: "Exception",
                    len: 3,
                },
                Token::Str("errorClass"),
                Token::Str("Assert"),
                Token::Str("message"),
                Token::Str("Assert"),
                Token::Str("stacktrace"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
