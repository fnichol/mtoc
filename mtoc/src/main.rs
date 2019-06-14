// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! mtoc CLI.

#![deny(missing_docs)]

use log::{debug, info};
use mtoc_parser::{Formatter, Writer, WriterBuilder};
use std::fs::File;
use std::io;
use std::path::Path;
use std::process;
use std::result;
use structopt::StructOpt;

mod cli;
mod util;

/// Result type alias, using `Failure` to wrap up contexts and causes
type Result<T> = result::Result<T, failure::Error>;

fn main() {
    util::setup_panic_hooks();

    if let Err(err) = try_main() {
        // A pipe error occurs when the consumer of this process's output has hung up. This is a
        // normal event and we should quit gracefully.
        if util::is_pipe_error(&err) {
            info!("pipe error, quitting gracefully");
            process::exit(0);
        }

        // Print the error and all of its underlying causes
        eprintln!("{}", util::pretty_error(&err));

        process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let args = cli::Args::from_args();
    util::init_logger(args.verbosity());
    debug!("parsed cli arguments; args={:?}", args);

    let buf = args.input_string()?;

    let mut builder = WriterBuilder::new(&buf).formatter(args.formatter());
    if let Some(marker) = args.begin_marker() {
        builder = builder.begin_marker(marker);
    }
    if let Some(marker) = args.end_marker() {
        builder = builder.end_marker(marker);
    }

    match args.output() {
        Some(output) => write_to_file(builder, output),
        None => match args.is_in_place() {
            true => {
                debug!("writing in-place");
                match args.input() {
                    Some(input) => write_to_file(builder, input),
                    None => write_to_stdout(builder),
                }
            }
            false => write_to_stdout(builder),
        },
    }
}

fn write_to_stdout(builder: Writer<Formatter>) -> Result<()> {
    info!("writing to stdout");
    builder.write(&mut io::stdout().lock())?;
    Ok(())
}

fn write_to_file(builder: Writer<Formatter>, path: &Path) -> Result<()> {
    info!("writing to file; file={:?}", path);
    let mut output = File::create(path)?;
    builder.write(&mut output)?;
    Ok(())
}
