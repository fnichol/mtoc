// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use chrono::{SecondsFormat, Utc};
use std::env;
use std::io;
use std::panic;

/// The logger.
const LOGGER: &Logger = &Logger;

/// A custom and minimal `Log` implementation.
///
/// This logger writes only to the standard error stream so as not to affect the normal operation
/// of mtoc when writing output to the standard output stream.
///
/// Thanks to the logger implementations from ripgrep and the simplelog crate which served as an
/// inspiration.
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let file = record.file().unwrap_or("<unknown>");
        let location = match record.line() {
            Some(line) => format!("{}:{}", file, line),
            None => format!("{}:<unknown>", file),
        };

        eprintln!(
            "{} {:<5} [{}] {}",
            Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            record.level(),
            location,
            record.args()
        );
    }

    fn flush(&self) {
        // `eprintln!` flushes on every call
    }
}

/// Sets up and initializes the logger.
pub(crate) fn init_logger(verbosity: usize) {
    log::set_logger(LOGGER).expect("error setting logger");

    match verbosity {
        1 => log::set_max_level(log::LevelFilter::Info),
        2 => log::set_max_level(log::LevelFilter::Debug),
        v if v >= 3 => log::set_max_level(log::LevelFilter::Trace),
        _ => {}
    }
    log::debug!("verbosity={}", verbosity);
}

/// Wires up a human-first experience if the program panics unexpectedly and also supports the
/// normal `RUST_BACKTRACE` environment variable.
///
/// A big thanks to https://github.com/rustwasm/wasm-pack for demonstrating such a delightful
/// pattern. All credit here goes to the wasm-pack crew, thanks!
pub(crate) fn setup_panic_hooks() {
    let meta = human_panic::Metadata {
        version: env!("CARGO_PKG_VERSION").into(),
        name: env!("CARGO_PKG_NAME").into(),
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
    };

    let default_hook = panic::take_hook();

    if env::var("RUST_BACKTRACE").is_err() {
        panic::set_hook(Box::new(move |info: &panic::PanicInfo| {
            // First call the default hook that prints to standard error
            default_hook(info);

            // Then call human_panic
            let file_path = human_panic::handle_dump(&meta, info);
            human_panic::print_msg(file_path, &meta)
                .expect("human-panic: printing error message to console failed");
        }));
    }
}

/// Determines if the error is caused by an I/O broken pipe.
///
/// Thanks to the imdb-rename crate for this wonderful implementation.
pub(crate) fn is_pipe_error(err: &failure::Error) -> bool {
    for cause in err.iter_chain() {
        if let Some(ioerr) = cause.downcast_ref::<io::Error>() {
            if ioerr.kind() == io::ErrorKind::BrokenPipe {
                return true;
            }
        }
    }
    false
}

/// Return a prettily formatted error, including its entire causal chain.
///
/// Thanks again to the imdb-rename crate and wasm-pack which form the basis of this
/// implementation.
pub(crate) fn pretty_error(err: &failure::Error) -> String {
    let mut pretty = "Error: ".to_string();
    pretty.push_str(&err.to_string());
    pretty.push_str("\n");
    for cause in err.iter_causes() {
        pretty.push_str(&cause.to_string());
        pretty.push_str("\n");
    }
    pretty
}
