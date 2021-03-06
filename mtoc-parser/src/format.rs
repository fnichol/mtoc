// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use crate::Header;
use std::default::Default;
use std::io::{self, Write};

const ALTERNATING_BULLET_STYLES: &[&str] = &["-", "*", "+"];

/// An output formatter for an Iterator of [`Header`]s.
///
/// This is primary method of consuming an Iterator of `Header` `struct`s and formatting them for
/// output. The [`Format`] trait implementation takes the `Header`s and anything that implements
/// the `Write` trait.
///
/// # Examples
///
/// Basic usage, using the `Default` implementation and writing to an output `Vec` of bytes:
///
/// ```rust
/// use mtoc_parser::{headers, Format, Formatter};
/// use std::str;
///
/// let input = "# Title";
/// let mut output = Vec::new();
///
/// Formatter::default()
///     .fmt(&mut output, headers(input))
///     .unwrap();
///
/// assert_eq!("- [Title](#title)\n", str::from_utf8(&output).unwrap());
/// ```
///
/// [`Format`]: trait.Format.html
/// [`Header`]: struct.Header.html
pub enum Formatter<'a> {
    /// A formatter that alternates between dashes (`-`), asterisks (`*`), and pluses (`+`) when
    /// formatting deeply nested [`Header`] entries. For example:
    ///
    /// ```markdown
    /// - [Title](#title)
    ///   * [Introduction](#introduction)
    ///   * [Body](#body)
    ///     + [Detail](#detail)
    ///     + [Detail](#detail-1)
    ///   * [Conclusion](#conclusion)
    /// ```
    ///
    /// [`Header`]: struct.Header.html
    AlternatingBullets,
    /// A formatter that uses only dashes (`-`) when formatting deeply nested [`Header`] entries.
    /// For example:
    ///
    /// ```markdown
    /// * [Title](#title)
    ///   * [Introduction](#introduction)
    ///   * [Body](#body)
    ///     * [Detail](#detail)
    ///     * [Detail](#detail-1)
    ///   * [Conclusion](#conclusion)
    /// ```
    ///
    /// [`Header`]: struct.Header.html
    DashBullets,
    /// A formatter that uses only pluses (`+`) when formatting deeply nested [`Header`] entries.
    /// For example:
    ///
    /// ```markdown
    /// + [Title](#title)
    ///   + [Introduction](#introduction)
    ///   + [Body](#body)
    ///     + [Detail](#detail)
    ///     + [Detail](#detail-1)
    ///   + [Conclusion](#conclusion)
    /// ```
    ///
    /// [`Header`]: struct.Header.html
    PlusBullets,
    /// A formatter that uses only asterisks (`*`) when formatting deeply nested [`Header`] entries.
    /// For example:
    ///
    /// ```markdown
    /// * [Title](#title)
    ///   * [Introduction](#introduction)
    ///   * [Body](#body)
    ///     * [Detail](#detail)
    ///     * [Detail](#detail-1)
    ///   * [Conclusion](#conclusion)
    /// ```
    ///
    /// [`Header`]: struct.Header.html
    AsteriskBullets,
    /// A formatter that uses numbering when formatting deeply nested [`Header`] entries.  For
    /// example:
    ///
    /// ```markdown
    /// 1. [Title](#title)
    ///    1. [Introduction](#introduction)
    ///    1. [Body](#body)
    ///       1. [Detail](#detail)
    ///       1. [Detail](#detail-1)
    ///    1. [Conclusion](#conclusion)
    /// ```
    ///
    /// [`Header`]: struct.Header.html
    Numbers,
    /// A formatter that uses a custom `str` when formatting deeply nested [`Header`] entries.  For
    /// example:
    ///
    /// ```markdown
    /// ★  [Title](#title)
    ///    ★  [Introduction](#introduction)
    ///    ★  [Body](#body)
    ///       ★  [Detail](#detail)
    ///       ★  [Detail](#detail-1)
    ///    ★  [Conclusion](#conclusion)
    /// ```
    ///
    /// [`Header`]: struct.Header.html
    Custom(&'a str),
}

impl<'a> Default for Formatter<'a> {
    fn default() -> Self {
        Formatter::AlternatingBullets
    }
}

impl<'a> Format for Formatter<'a> {
    fn fmt<W, I>(&self, out: &mut W, mut headers: I) -> io::Result<()>
    where
        W: Write,
        I: Iterator<Item = Header>,
    {
        use Formatter::*;

        headers.try_for_each(|header| match self {
            AlternatingBullets => format_alternating_bullets(out, header),
            DashBullets => format_symbols(out, header, "-"),
            PlusBullets => format_symbols(out, header, "+"),
            AsteriskBullets => format_symbols(out, header, "*"),
            Custom(bullet) => format_symbols(out, header, bullet),
            Numbers => format_symbols(out, header, "1."),
        })
    }
}

/// A trait for objects which can format a collection of [`Header`]s to a 'writer'.
///
/// The behavior is defined by one required method, [`fmt`]:
///
/// * The [`fmt`] method writes out the `Header`s to the provided 'writer' which is an object
///   implementing the `Write` trait.
///
/// # Examples
///
/// An example of implementing a custom `Format` implementation:
///
/// ```rust
/// use mtoc_parser::{headers, Format, Header};
/// use std::default::Default;
/// use std::io::{self, Write};
/// use std::str;
///
/// // Setup an empty struct to hold the trait behavior
/// struct DebugFormatter;
///
/// // The `Format` trait requires a `Default` trait implementation
/// impl Default for DebugFormatter {
///     fn default() -> Self {
///         DebugFormatter
///     }
/// }
///
/// // Implement the `Format` trait. In this case, output the members of each `Header`.
/// impl Format for DebugFormatter {
///     fn fmt<W, I>(&self, writer: &mut W, mut headers: I) -> io::Result<()>
///     where
///         W: Write,
///         I: Iterator<Item = Header>,
///     {
///         headers.try_for_each(|h| {
///             writeln!(
///                 writer,
///                 "* {{ level={}, title={:?}, anchor={:?} }}",
///                 h.level(),
///                 h.title(),
///                 h.anchor()
///             )
///         })
///     }
/// }
///
/// let input = "# Title";
/// let mut output = Vec::new();
///
/// DebugFormatter.fmt(&mut output, headers(input)).unwrap();
///
/// assert_eq!(
///     "* { level=1, title=\"Title\", anchor=\"#title\" }\n",
///     str::from_utf8(&output).unwrap()
/// );
/// ```
///
/// [`Header`]: struct.Header.html
/// [`fmt`]: #tymethod.fmt
pub trait Format: Default {
    /// Writes out a collection of [`Header`]s to the provided 'writer'.
    ///
    /// # Errors
    ///
    /// Each call to the writer's underlying `write` method may generate an I/O error indicating
    /// the operation could not be completed.
    ///
    /// [`Header`]: struct.Header.html
    fn fmt<W, I>(&self, writer: &mut W, headers: I) -> io::Result<()>
    where
        W: Write,
        I: Iterator<Item = Header>;
}

fn format_alternating_bullets<W: Write>(out: &mut W, header: Header) -> io::Result<()> {
    let level = header.level();
    let len = ALTERNATING_BULLET_STYLES.len();

    format_symbols(out, header, ALTERNATING_BULLET_STYLES[(level - 1) % len])
}

fn format_symbols<W: Write>(out: &mut W, header: Header, bullet: &str) -> io::Result<()> {
    let level = header.level();
    let indent = bullet.chars().count() + 1;

    writeln!(
        out,
        "{: <indent$}{bullet} {header}",
        "",
        indent = (level - 1) * indent,
        bullet = bullet,
        header = header,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers;
    use std::str;

    const MD: &str =
        "# Title\n## Introduction\n## Body\n### Detail\n#### Minutiae\n### Detail\n## Conclusion";

    #[test]
    fn alternating_bullets() {
        let mut out = Vec::new();
        Formatter::AlternatingBullets
            .fmt(&mut out, headers(MD))
            .unwrap();
        let mut lines = str::from_utf8(&out).unwrap().lines();

        assert_eq!(Some("- [Title](#title)"), lines.next());
        assert_eq!(Some("  * [Introduction](#introduction)"), lines.next());
        assert_eq!(Some("  * [Body](#body)"), lines.next());
        assert_eq!(Some("    + [Detail](#detail)"), lines.next());
        assert_eq!(Some("      - [Minutiae](#minutiae)"), lines.next());
        assert_eq!(Some("    + [Detail](#detail-1)"), lines.next());
        assert_eq!(Some("  * [Conclusion](#conclusion)"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn dash_bullets() {
        let mut out = Vec::new();
        Formatter::DashBullets.fmt(&mut out, headers(MD)).unwrap();
        let mut lines = str::from_utf8(&out).unwrap().lines();

        assert_eq!(Some("- [Title](#title)"), lines.next());
        assert_eq!(Some("  - [Introduction](#introduction)"), lines.next());
        assert_eq!(Some("  - [Body](#body)"), lines.next());
        assert_eq!(Some("    - [Detail](#detail)"), lines.next());
        assert_eq!(Some("      - [Minutiae](#minutiae)"), lines.next());
        assert_eq!(Some("    - [Detail](#detail-1)"), lines.next());
        assert_eq!(Some("  - [Conclusion](#conclusion)"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn plus_bullets() {
        let mut out = Vec::new();
        Formatter::PlusBullets.fmt(&mut out, headers(MD)).unwrap();
        let mut lines = str::from_utf8(&out).unwrap().lines();

        assert_eq!(Some("+ [Title](#title)"), lines.next());
        assert_eq!(Some("  + [Introduction](#introduction)"), lines.next());
        assert_eq!(Some("  + [Body](#body)"), lines.next());
        assert_eq!(Some("    + [Detail](#detail)"), lines.next());
        assert_eq!(Some("      + [Minutiae](#minutiae)"), lines.next());
        assert_eq!(Some("    + [Detail](#detail-1)"), lines.next());
        assert_eq!(Some("  + [Conclusion](#conclusion)"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn asterisk_bullets() {
        let mut out = Vec::new();
        Formatter::AsteriskBullets
            .fmt(&mut out, headers(MD))
            .unwrap();
        let mut lines = str::from_utf8(&out).unwrap().lines();

        assert_eq!(Some("* [Title](#title)"), lines.next());
        assert_eq!(Some("  * [Introduction](#introduction)"), lines.next());
        assert_eq!(Some("  * [Body](#body)"), lines.next());
        assert_eq!(Some("    * [Detail](#detail)"), lines.next());
        assert_eq!(Some("      * [Minutiae](#minutiae)"), lines.next());
        assert_eq!(Some("    * [Detail](#detail-1)"), lines.next());
        assert_eq!(Some("  * [Conclusion](#conclusion)"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn numbers() {
        let mut out = Vec::new();
        Formatter::Numbers.fmt(&mut out, headers(MD)).unwrap();
        let mut lines = str::from_utf8(&out).unwrap().lines();

        assert_eq!(Some("1. [Title](#title)"), lines.next());
        assert_eq!(Some("   1. [Introduction](#introduction)"), lines.next());
        assert_eq!(Some("   1. [Body](#body)"), lines.next());
        assert_eq!(Some("      1. [Detail](#detail)"), lines.next());
        assert_eq!(Some("         1. [Minutiae](#minutiae)"), lines.next());
        assert_eq!(Some("      1. [Detail](#detail-1)"), lines.next());
        assert_eq!(Some("   1. [Conclusion](#conclusion)"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn custom() {
        let mut out = Vec::new();
        Formatter::Custom("wat.")
            .fmt(&mut out, headers(MD))
            .unwrap();
        let mut lines = str::from_utf8(&out).unwrap().lines();

        // This might be the most terrible example evar--even Rusfmt clearly doesn't like it!
        assert_eq!(Some("wat. [Title](#title)"), lines.next());
        assert_eq!(
            Some("     wat. [Introduction](#introduction)"),
            lines.next()
        );
        assert_eq!(Some("     wat. [Body](#body)"), lines.next());
        assert_eq!(Some("          wat. [Detail](#detail)"), lines.next());
        assert_eq!(
            Some("               wat. [Minutiae](#minutiae)"),
            lines.next()
        );
        assert_eq!(Some("          wat. [Detail](#detail-1)"), lines.next());
        assert_eq!(Some("     wat. [Conclusion](#conclusion)"), lines.next());
        assert_eq!(None, lines.next());
    }
}
