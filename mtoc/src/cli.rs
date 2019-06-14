// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use crate::Result;
use mtoc_parser::Formatter;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use structopt::{clap::arg_enum, clap::AppSettings, StructOpt};

/// The "about" string for help messages.
const ABOUT: &str = concat!(
    "
Generates and writes a table of contents into any Markdown document.

Project home page: ",
    env!("CARGO_PKG_HOMEPAGE"),
    r"

Use -h for short descriptions and --help for more details.",
);

/// The "long_about" string for help messages.
const LONG_ABOUT: &str = concat!(
    "
Generates and writes a table of contents into any Markdown document.

mtoc uses a compliant CommonMark pull parser to process its input so the results are always safe \
and semantic. The title and slug normalizing implementation ensures that the generated anchor \
links will be valid and compatible with GitHub README, CHANGELOG, and other Markdown files.

By default, a table of contents will be inserted between 2 HTML marker comments:

    <!-- toc -->
    ...
    <!-- toctop -->

Running mtoc again on the same source will re-generate and update the table of contents. Only the \
begin marker is needed to insert a table of contents the first time.

mtoc can edit a file in place with the -i/--in-place flag and can be used in a pipeline by \
reading from standard input and writing to standard output (its default behavior).

Project home page: ",
    env!("CARGO_PKG_HOMEPAGE"),
    r"

Use -h for short descriptions and --help for more details.",
);

const AFTER_HELP: &str = "\
EXAMPLES:
    Example 1 Adding or updating a table of contents in a Markdown file
      The following command adds a table of contents, which has a new line
      containing the begin marker: '<!-- toc -->'.

      # mtoc -i /path/to/file.md

    Example 2 Using mtoc in a pipeline
      The following command will consume its input from a program 'tool1' and
      will produce its output as input to another program 'tool3'.

      # tool1 --input /path/to/file.md | mtoc | tool3 --output /path/to/file.md

";

/// The parsed CLI arguments.
///
/// This struct also doubles as the CLI parser.
#[derive(Debug, StructOpt)]
#[structopt(raw(
    setting = "AppSettings::UnifiedHelpMessage",
    max_term_width = "100",
    about = "ABOUT",
    long_about = "LONG_ABOUT",
    after_help = "AFTER_HELP"
))]
pub(crate) struct Args {
    /// An input Markdown file to read [default: stdin]
    ///
    /// If the input Markdown file contain a begin marker or begin and end markers, mtoc will
    /// insert a table of contents at the first marker location it finds. If no markers are
    /// present, then the output contents will match the input contents.
    ///
    /// If no INPUT is specified, mtoc will read the Markdown content from the standard input
    /// stream.
    #[structopt(rename_all = "screaming_snake_case")]
    input: Option<PathBuf>,

    /// Sets the output Markdown file to write [default: stdout]
    ///
    /// When this option is used, the output will be written to the OUTPUT file. If no OUTPUT is
    /// specified, mtoc will write the Markdown to the standard output stream.
    ///
    /// This option conflicts with the -i/--in-place option and only one should be used. If the
    /// OUTPUT is the same as INPUT, then -i/--in-place should be used instead.
    #[structopt(
        short = "o",
        long = "output",
        conflicts_with = "IN_PLACE",
        rename_all = "screaming_snake_case"
    )]
    output: Option<PathBuf>,

    /// Edits INPUT file in place.
    ///
    /// When this flag is used, the INPUT file will be read from and written back to in place. If
    /// no INPUT is specified and this flag is used, the output will be written to the standard
    /// output stream.
    ///
    /// This conflicts with the -o/--output option and only one should be used.
    #[structopt(short = "i", long = "in-place")]
    in_place: bool,

    /// Sets the table of contents formatting.
    ///
    /// There are 5 formatting styles, with 'alternating' being the default.
    ///
    #[structopt(
        short = "f",
        long = "format",
        rename_all = "screaming_snake_case",
        raw(
            possible_values = "&CliFormat::variants()",
            default_value = "\"alternating\""
        )
    )]
    format: CliFormat,

    /// Sets a custom begin marker.
    ///
    /// The begin marker is used by mtoc to insert a new table of contents or to replace an
    /// existing table of contents in a Markdown document. Only the first occurrence of this marker
    /// will be used for insertion or replacement.
    ///
    /// Currently only HTML elements, such as HTML comments are supported.
    ///
    /// [default: <!-- toc -->]
    #[structopt(
        short = "b",
        long = "begin-marker",
        rename_all = "screaming_snake_case"
    )]
    begin_marker: Option<String>,

    /// Sets a custom end marker.
    ///
    /// The end marker is used by mtoc to find the end of a table of contents block when inserting
    /// a new table of contents or when replacing an existing table of contents in a Markdown
    /// document. Only the first occurence of this marker will be used for insertion or
    /// replacement.
    ///
    /// The input Markdown document may only contain a begin marker, in which case mtoc will insert
    /// the end marker after inserting the table of contents so that future invocations of mtoc
    /// will know how to update the table of contents region. If an end marker is present, then all
    /// content between the begin and end markers will be replaced.
    ///
    /// Currently only HTML elements, such as HTML comments are supported.
    ///
    /// [default: <!-- tocstop -->]
    #[structopt(short = "e", long = "end-marker", rename_all = "screaming_snake_case")]
    end_marker: Option<String>,

    /// Verbose mode.
    ///
    /// Causes mtoc to print debugging messages about its progress. This is helpful
    /// when debugging problems.
    ///
    /// Multiple -v options increase the verbosity. The maximum is 3.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

impl Args {
    /// Returns the input as a reference to a `Path`.
    ///
    /// If input is `None`, then no argument was provided, and the standard input stream will be
    /// assumed as input source.
    pub(crate) fn input(&self) -> Option<&Path> {
        self.input.as_ref().map(PathBuf::as_path)
    }

    /// Returns the output as a reference to a `Path`.
    ///
    /// If output is `None`, then no output option was provided, meaning that either "in place"
    /// mode has been selected or the standard output stream will be assumed as output target.
    pub(crate) fn output(&self) -> Option<&Path> {
        self.output.as_ref().map(PathBuf::as_path)
    }

    /// Returns the table of contents `Formatter`.
    pub(crate) fn formatter(&self) -> Formatter<'static> {
        Formatter::from(&self.format)
    }

    /// Returns whether or not the "in place" editing mode has been selected.
    ///
    /// If this mode has been selected, then the output target will be the same as the input
    /// source. If the standard input stream is the input source, then the standard output stream
    /// will be the output target.
    pub(crate) fn is_in_place(&self) -> bool {
        self.in_place
    }

    /// Returns the custom begin marker, if provided.
    ///
    /// If the marker is `None`, then the default marker will be used.
    pub(crate) fn begin_marker(&self) -> Option<&str> {
        self.begin_marker.as_ref().map(String::as_str)
    }

    /// Returns the custom end marker, if provided.
    ///
    /// If the marker is `None`, then the default marker will be used.
    pub(crate) fn end_marker(&self) -> Option<&str> {
        self.end_marker.as_ref().map(String::as_str)
    }

    /// Returns the verbosity level.
    ///
    /// A `0` value is "off", and increasing numbers increase verbosity. Any value above `3` will
    /// be treated as identical to `3`.
    pub(crate) fn verbosity(&self) -> usize {
        self.verbose
    }

    /// Allocates and returns a `String` containing the contents of the input source.
    ///
    /// Note that this is a heap allocation, which is required by the underlying Markdown library.
    /// It would have been swell to stream this solution and keep RAM usage down, but sadly we're
    /// stuck with this approach. The good news is that only one copy will be allocated and the
    /// `mtoc_parser` library will only reference one copy.
    pub(crate) fn input_string(&self) -> Result<String> {
        match &self.input {
            Some(input) => string_from_path(input),
            None => string_from_stdin(),
        }
    }
}

arg_enum! {
    /// The possible format values for the CLI.
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum CliFormat {
        alternating,
        asterisks,
        dashes,
        numbers,
        pluses,
    }
}

impl From<&CliFormat> for Formatter<'static> {
    fn from(format: &CliFormat) -> Formatter<'static> {
        use CliFormat::*;

        match format {
            alternating => Formatter::AlternatingBullets,
            asterisks => Formatter::AsteriskBullets,
            dashes => Formatter::DashBullets,
            numbers => Formatter::Numbers,
            pluses => Formatter::PlusBullets,
        }
    }
}

/// Read and return the contents of the standard input stream as a `String`.
///
/// # Errors
///
/// * In an I/O error occurs when reading on `stdin`
fn string_from_stdin() -> Result<String> {
    let mut buf = String::new();
    std::io::stdin().lock().read_to_string(&mut buf)?;

    Ok(buf)
}

/// Read and return the contents of the given file as a `String`.
///
/// # Errors
///
/// * If a file at the `Path` does not exist
/// * If the file cannot be open in read mode due to ownership or permissions
/// * If an I/O error occurs when reading the opened file
/// * If the file size (a `u8`) cannot be converted to a `usize`
fn string_from_path(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut buf = String::with_capacity(file.metadata()?.len().try_into()?);
    let mut file = BufReader::new(file);
    file.read_to_string(&mut buf)?;

    Ok(buf)
}
