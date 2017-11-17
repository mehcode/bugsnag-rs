//! An example showing the integration of the bugsnag api with the logging framework of rust.
//!
//! This simple implementations consumes the api object. So, we can not change
//! any parameters after registering the logger.

extern crate bugsnag;
#[macro_use]
extern crate log;

use log::{LogLevel, LogMetadata, LogRecord, SetLoggerError};

/// Our logger for bugsnag
struct BugsnagLogger {
    max_loglevel: LogLevel,
    api: bugsnag::Bugsnag,
}

impl BugsnagLogger {
    pub fn init(api: bugsnag::Bugsnag, max_loglevel: LogLevel) -> Result<(), SetLoggerError> {
        log::set_logger(|max_level| {
            max_level.set(log::LogLevelFilter::Info);
            Box::new(BugsnagLogger {
                api: api,
                max_loglevel: max_loglevel,
            })
        })
    }
}

fn convert_log_level(level: LogLevel) -> bugsnag::Severity {
    match level {
        LogLevel::Info => bugsnag::Severity::Info,
        LogLevel::Warn => bugsnag::Severity::Warning,
        LogLevel::Error => bugsnag::Severity::Error,
        _ => bugsnag::Severity::Info,
    }
}

impl log::Log for BugsnagLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.max_loglevel
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let level = convert_log_level(record.metadata().level());
            self.api
                .notify(
                    record.metadata().level().to_string().as_str(),
                    record.args().to_string().as_str(),
                    level,
                    None,
                    None,
                )
                .unwrap();
        }
    }
}



fn main() {
    let mut api =
        bugsnag::Bugsnag::new("api-key", concat!(env!("CARGO_MANIFEST_DIR"), "/examples"));
    api.set_app_info(
        Some(env!("CARGO_PKG_VERSION")),
        Some("development"),
        Some("rust"),
    );

    // initialize the logger
    BugsnagLogger::init(api, LogLevel::Warn).unwrap();

    // the following two messages should not show up in bugsnag, because
    // we set the maximum log level to errors
    debug!("Hello this is a debug message!");
    info!("Hello this is a info message!");

    // the following two should be send to bugsnag
    warn!("Hello this is a warn message!");
    error!("Hello this is a error message!");
}
