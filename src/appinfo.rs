#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    #[serde(skip_serializing_if = "Option::is_none")] version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] release_stage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    atype: Option<String>,
}

impl AppInfo {
    pub fn new(version: Option<&str>, release_stage: Option<&str>, atype: Option<&str>) -> AppInfo {
        AppInfo {
            version: version.map_or_else(|| None, |v| Some(v.to_owned())),
            release_stage: release_stage.map_or_else(|| None, |v| Some(v.to_owned())),
            atype: atype.map_or_else(|| None, |v| Some(v.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AppInfo;
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn test_appinfo_to_json() {
        let info = AppInfo::new(Some("1.0.0"), Some("test"), Some("rust"));

        assert_ser_tokens(
            &info,
            &[
                Token::StructStart("AppInfo", 3),
                Token::StructSep,
                Token::Str("version"),
                Token::Option(true),
                Token::Str("1.0.0"),
                Token::StructSep,
                Token::Str("releaseStage"),
                Token::Option(true),
                Token::Str("test"),
                Token::StructSep,
                Token::Str("type"),
                Token::Option(true),
                Token::Str("rust"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_appinfo_with_version_to_json() {
        let info = AppInfo::new(Some("1.0.0"), None, None);

        assert_ser_tokens(
            &info,
            &[
                Token::StructStart("AppInfo", 1),
                Token::StructSep,
                Token::Str("version"),
                Token::Option(true),
                Token::Str("1.0.0"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_appinfo_with_release_stage_to_json() {
        let info = AppInfo::new(None, Some("test"), None);

        assert_ser_tokens(
            &info,
            &[
                Token::StructStart("AppInfo", 1),
                Token::StructSep,
                Token::Str("releaseStage"),
                Token::Option(true),
                Token::Str("test"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_appinfo_with_type_to_json() {
        let info = AppInfo::new(None, None, Some("rust"));

        assert_ser_tokens(
            &info,
            &[
                Token::StructStart("AppInfo", 1),
                Token::StructSep,
                Token::Str("type"),
                Token::Option(true),
                Token::Str("rust"),
                Token::StructEnd,
            ],
        );
    }
}
