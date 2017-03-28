//! # The Bugsnag api
//!
//! This crate provides an interface for reporting messages to Bugsnag.
//!
//! # Example
//!
//! ```
//! use bugsnag;
//! let mut api = bugsnag::Bugsnag::new("api-key", env!("CARGO_MANIFEST_DIR"));
//!
//! // setting the appinfo is not required, but recommended 
//! api.set_app_info(Some(env!("CARGO_PKG_VERSION")),
//!                  Some("development"),
//!                  Some("rust"));
//!
//! let stacktrace = bugsnag::stacktrace::create_stacktrace(
//!     Some(&|file, _| file.starts_with(env!("CARGO_MANIFEST_DIR"))));
//!
//! api.notify("Info", "This is a message from the rust bugsnag api.",
//!            bugsnag::Severity::Info, &stacktrace, None); 
//! ```
//!
//! For more examples on how to integrate bugsnag into a project, the examples
//! folder provides some reference implementations.

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_test;
extern crate backtrace;
extern crate hyper;
extern crate sys_info;

mod event;
mod notification;
pub mod stacktrace;
mod exception;
mod bugsnag_impl;
pub use self::bugsnag_impl::*;
mod deviceinfo;
mod appinfo;
