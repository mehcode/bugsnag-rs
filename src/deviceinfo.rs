use sys_info;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    os_version: String,
    hostname: String,
}

impl DeviceInfo {
    pub fn new(version: &str, name: &str) -> DeviceInfo {
        DeviceInfo {
            os_version: version.to_owned(),
            hostname: name.to_owned(),
        }
    }

    pub fn generate() -> DeviceInfo {
        let mut version = sys_info::os_type().unwrap_or("Unknown".to_owned());
        version.push_str(":");
        version.push_str(&sys_info::os_release().unwrap_or("u.k.n.o.w.n".to_owned()));

        let hostname = sys_info::hostname().unwrap_or("UnknownHost".to_owned());

        DeviceInfo::new(version.as_str(), hostname.as_str())
    }

    pub fn set_os_version(&mut self, version: &str) {
        self.os_version = version.to_owned();
    }

    pub fn set_hostname(&mut self, name: &str) {
        self.hostname = name.to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::DeviceInfo;
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn test_deviceinfo_to_json() {
        let info = DeviceInfo::new("1.0.0", "testmachine");

        assert_ser_tokens(
            &info,
            &[
                Token::Struct {
                    name: "DeviceInfo",
                    len: 2,
                },
                Token::Str("osVersion"),
                Token::Str("1.0.0"),
                Token::Str("hostname"),
                Token::Str("testmachine"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_deviceinfo_to_json_with_set() {
        let mut info = DeviceInfo::generate();
        info.set_hostname("testmachine3");
        info.set_os_version("3.0.0");

        assert_ser_tokens(
            &info,
            &[
                Token::Struct {
                    name: "DeviceInfo",
                    len: 2,
                },
                Token::Str("osVersion"),
                Token::Str("3.0.0"),
                Token::Str("hostname"),
                Token::Str("testmachine3"),
                Token::StructEnd,
            ],
        );
    }
}
