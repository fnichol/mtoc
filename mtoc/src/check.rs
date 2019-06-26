// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! A module to compute and display the textural differences between two sources.
//!
//! This implementation was taken, adapted, and simplified from the Rustfmt project's
//! `rustfmt_diff` module. Rustfmt is dual licensed under the Apache v2 license and the MIT
//! license.
//!
//! Source:
//! * https://github.com/rust-lang/rustfmt
//! * https://git.io/fjrwX
use crate::Result;
use std::collections::VecDeque;
use std::fmt::Display;
use std::io::Write;

/// Represents a comparison line between two sources.
#[derive(Debug, PartialEq)]
enum DiffLine {
    Context(String),
    Expected(String),
    Resulting(String),
}

/// Represents a contiguous collection of lines that differ between two sources.
#[derive(Debug, PartialEq)]
struct Mismatch {
    line_number_orig: u32,
    lines: Vec<DiffLine>,
}

impl Mismatch {
    /// Builds and returns a new instance.
    fn new(line_number_orig: u32) -> Mismatch {
        Mismatch {
            line_number_orig,
            lines: Vec::new(),
        }
    }
}

/// Writes a diff-like report to a 'writer' between two sources.
pub(crate) fn write_diff<D, W>(
    expected: &str,
    actual: &str,
    source: D,
    writer: &mut W,
) -> Result<()>
where
    D: Display,
    W: Write,
{
    let diff = make_diff(expected, actual, 3);

    for mismatch in diff {
        writeln!(
            writer,
            "Diff in {} at line {}:",
            source, mismatch.line_number_orig
        )?;

        for line in mismatch.lines {
            match line {
                DiffLine::Context(ref str) => {
                    writeln!(writer, " {}", str)?;
                }
                DiffLine::Expected(ref str) => {
                    writeln!(writer, "+{}", str)?;
                }
                DiffLine::Resulting(ref str) => {
                    writeln!(writer, "-{}", str)?;
                }
            }
        }
    }

    Ok(())
}

/// Builds a list of contiguous mismatching lines between two sources.
fn make_diff(expected: &str, actual: &str, context_size: usize) -> Vec<Mismatch> {
    let mut line_number_orig = 1;
    let mut context_queue: VecDeque<&str> = VecDeque::with_capacity(context_size);
    let mut lines_since_mismatch = context_size + 1;
    let mut results = Vec::new();
    let mut mismatch = Mismatch::new(0);

    for result in diff::lines(expected, actual) {
        match result {
            diff::Result::Left(str) => {
                if lines_since_mismatch >= context_size && lines_since_mismatch > 0 {
                    results.push(mismatch);
                    mismatch = Mismatch::new(line_number_orig - context_queue.len() as u32);
                }

                while let Some(line) = context_queue.pop_front() {
                    mismatch.lines.push(DiffLine::Context(line.to_owned()));
                }

                mismatch.lines.push(DiffLine::Resulting(str.to_owned()));
                line_number_orig += 1;
                lines_since_mismatch = 0;
            }
            diff::Result::Right(str) => {
                if lines_since_mismatch >= context_size && lines_since_mismatch > 0 {
                    results.push(mismatch);
                    mismatch = Mismatch::new(line_number_orig - context_queue.len() as u32);
                }

                while let Some(line) = context_queue.pop_front() {
                    mismatch.lines.push(DiffLine::Context(line.to_owned()));
                }

                mismatch.lines.push(DiffLine::Expected(str.to_owned()));
                lines_since_mismatch = 0;
            }
            diff::Result::Both(str, _) => {
                if context_queue.len() >= context_size {
                    let _ = context_queue.pop_front();
                }

                if lines_since_mismatch < context_size {
                    mismatch.lines.push(DiffLine::Context(str.to_owned()));
                } else if context_size > 0 {
                    context_queue.push_back(str);
                }

                line_number_orig += 1;
                lines_since_mismatch += 1;
            }
        }
    }

    results.push(mismatch);
    results.remove(0);

    results
}

#[cfg(test)]
mod test {
    use super::*;
    use DiffLine::*;

    #[test]
    fn make_diff_simple() {
        let src = "one\ntwo\nthree\nfour\nfive\n";
        let dst = "one\ntwo\ntrois\nfour\nfive\n";
        let diff = make_diff(src, dst, 1);

        assert_eq!(
            diff,
            vec![Mismatch {
                line_number_orig: 2,
                lines: vec![
                    Context("two".to_owned()),
                    Resulting("three".to_owned()),
                    Expected("trois".to_owned()),
                    Context("four".to_owned()),
                ]
            }]
        );
    }

    #[test]
    fn make_diff_simple2() {
        let src = "one\ntwo\nthree\nfour\nfive\nsix\nseven\n";
        let dst = "one\ntwo\ntrois\nfour\ncinq\nsix\nseven\n";
        let diff = make_diff(src, dst, 1);

        assert_eq!(
            diff,
            vec![
                Mismatch {
                    line_number_orig: 2,
                    lines: vec![
                        Context("two".to_owned()),
                        Resulting("three".to_owned()),
                        Expected("trois".to_owned()),
                        Context("four".to_owned()),
                    ],
                },
                Mismatch {
                    line_number_orig: 5,
                    lines: vec![
                        Resulting("five".to_owned()),
                        Expected("cinq".to_owned()),
                        Context("six".to_owned()),
                    ],
                }
            ]
        );
    }

    #[test]
    fn make_diff_zerocontext() {
        let src = "one\ntwo\nthree\nfour\nfive\n";
        let dst = "one\ntwo\ntrois\nfour\nfive\n";
        let diff = make_diff(src, dst, 0);

        assert_eq!(
            diff,
            vec![Mismatch {
                line_number_orig: 3,
                lines: vec![Resulting("three".to_owned()), Expected("trois".to_owned())],
            }]
        );
    }

    #[test]
    fn make_diff_trailing_newline() {
        let src = "one\ntwo\nthree\nfour\nfive";
        let dst = "one\ntwo\nthree\nfour\nfive\n";
        let diff = make_diff(src, dst, 1);

        assert_eq!(
            diff,
            vec![Mismatch {
                line_number_orig: 5,
                lines: vec![Context("five".to_owned()), Expected("".to_owned())],
            }]
        );
    }

    #[test]
    fn write_diff_simple() {
        let src = "one\ntwo\nthree\nfour\nfive\n";
        let dst = "one\ntwo\ntrois\nfour\nfive\n";
        let mut buf = Vec::new();

        write_diff(src, dst, "<src>", &mut buf).unwrap();

        assert_eq!(
            "\
Diff in <src> at line 1:
 one
 two
-three
+trois
 four
 five
 
",
            std::str::from_utf8(&buf).unwrap()
        );
    }
}
