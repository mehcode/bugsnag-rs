# bugsnag-rs
The Bugsnag api in rust.

[![Build Status](https://travis-ci.org/superscale/bugsnag-rs.svg?branch=master)](https://travis-ci.org/superscale/bugsnag-rs)

[![Coverage Status](https://coveralls.io/repos/github/superscale/bugsnag-rs/badge.svg?branch=master)](https://coveralls.io/github/superscale/bugsnag-rs?branch=master)

# Example

```
use bugsnag;
let mut api = bugsnag::Bugsnag::new("api-key", Some(env!("CARGO_MANIFEST_DIR")));

// setting the appinfo is not required, but recommended 
api.set_app_info(Some(env!("CARGO_PKG_VERSION")),
                 Some("development"),
                 Some("rust"));

let stacktrace = bugsnag::stacktrace::create_stacktrace(api.get_project_source_dir());

api.notify("Info", "This is a message from the rust bugsnag api.",
           bugsnag::Severity::Info, &stacktrace, None); 
```

For more examples on how to integrate bugsnag into a project, the examples folder provides some reference implementations.
