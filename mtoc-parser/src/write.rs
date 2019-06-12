// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use crate::{Format, Formatter, Header};
use pulldown_cmark::{Event, OffsetIter, Parser};
use std::io::{self, Cursor, Write};
use std::marker::PhantomData;

const DEFAULT_BEGIN_MARKER: &str = "<!-- toc -->";
const DEFAULT_END_MARKER: &str = "<!-- tocstop -->";

/// A builder for a writer which outputs a Markdown document with its table of contents.
///
/// The table of contents can be customized or a default implementation is provided. The table of
/// contents will inserted between two delimiting markers which will be honored if the output is
/// used as input in subsequent runs. The collection of [`Header`]s is consumed when [`write`] is
/// called and can be repopulated by calling [`headers`] before calling `write` again.
///
/// # Examples
///
/// Inserting a table of contents using the default start marker:
///
/// ```rust
/// use mtoc_parser::WriterBuilder;
/// use std::str;
///
/// let input = "<!-- toc -->\n\n# Title\n## Intro\nHello.\n";
/// let mut output = Vec::new();
///
/// WriterBuilder::new(&input)
///     .write(&mut output)
///     .unwrap();
///
/// assert_eq!(
///     "<!-- toc -->\n\n- [Intro](#intro)\n\n<!-- tocstop -->\n\n# Title\n## Intro\nHello.\n",
///     str::from_utf8(&output).unwrap()
/// );
/// ```
///
/// Updating a table of contents which uses custom markers:
///
/// ```rust
/// use mtoc_parser::WriterBuilder;
/// use std::str;
///
/// let input = "# Title\n<!-- start -->\n* old\n<!-- end -->\n## Intro\nHello.\n";
/// let mut output = Vec::new();
///
/// WriterBuilder::new(&input)
///     .begin_marker("<!-- start -->")
///     .end_marker("<!-- end -->")
///     .write(&mut output)
///     .unwrap();
///
/// assert_eq!(
///     "# Title\n<!-- start -->\n\n- [Intro](#intro)\n\n<!-- end -->\n## Intro\nHello.\n",
///     str::from_utf8(&output).unwrap()
/// );
/// ```
///
/// [`Header`]: struct.Header.html
/// [`headers`]: #fn.headers
/// [`write`]: #fn.write
pub struct WriterBuilder<'a, 'c, 'd> {
    src: &'a str,
    begin_marker: &'c str,
    end_marker: &'d str,
}

impl<'a, 'c, 'd> WriterBuilder<'a, 'c, 'd> {
    /// Builds a new `WriterBuilder` holding a reference to a Markdown document as a string slice.
    pub fn new(src: &'a str) -> Self {
        WriterBuilder {
            src,
            begin_marker: DEFAULT_BEGIN_MARKER,
            end_marker: DEFAULT_END_MARKER,
        }
    }

    /// Sets a custom [`Format`] implementation for the table of contents [`Header`] elements.
    ///
    /// The default implementation is the `Default` for [`Formatter`] which is
    /// [`Formatter::AlternatingBullets`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use mtoc_parser::{Formatter, WriterBuilder};
    /// use std::str;
    ///
    /// let input = "<!-- toc -->\n\n# Title\n## Intro\nHello.\n";
    /// let mut output = Vec::new();
    ///
    /// WriterBuilder::new(&input)
    ///     .formatter(Formatter::Numbers)
    ///     .write(&mut output)
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     "<!-- toc -->\n\n1. [Intro](#intro)\n\n<!-- tocstop -->\n\n# Title\n## Intro\nHello.\n",
    ///     str::from_utf8(&output).unwrap()
    /// );
    /// ```
    ///
    /// For an example of using a custom `Format` implementation, see the examples in the
    /// [`Format`] trait documentation.
    ///
    /// [`Format`]: trait.Format.html
    /// [`Formatter`]: struct.Formatter.html
    /// [`Formatter::AlternatingBullets`]: enum.Formatter.html#variant.AlternatingBullets
    /// [`Header`]: struct.Header.html
    pub fn formatter<'b, F>(self, formatter: F) -> WriterFormatBuilder<'a, 'b, 'c, 'd, F>
    where
        F: Format + 'b,
    {
        WriterFormatBuilder {
            src: self.src,
            begin_marker: self.begin_marker,
            end_marker: self.end_marker,
            phantom: PhantomData,
            formatter,
        }
    }

    /// Sets a custom `Iterator` of [`Header`]s to be used for the table of contents.
    ///
    /// The default implementation skips the title (i.e. heading level 1) entry and promotes all
    /// sub-headings up one level meaning that a level 2 `## Heading` would become a level one
    /// table of contents entry.
    ///
    /// # Examples
    ///
    /// Basic usage which includes the title heading:
    ///
    /// ```rust
    /// use mtoc_parser::{headers, Formatter, WriterBuilder};
    /// use std::str;
    ///
    /// let input = "<!-- toc -->\n\n# h1\n## h1.1\n";
    /// let mut output = Vec::new();
    ///
    /// WriterBuilder::new(&input)
    ///     .headers(Box::new(headers(&input)))
    ///     .write(&mut output)
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     "<!-- toc -->\n\n- [h1](#h1)\n  * [h1.1](#h11)\n\n<!-- tocstop -->\n\n# h1\n## h1.1\n",
    ///     str::from_utf8(&output).unwrap()
    /// );
    /// ```
    /// For more examples of providing custom iterators, see the examples in the [module]
    /// documentation.
    ///
    /// [`Header`]: struct.Header.html
    /// [module]: index.html
    pub fn headers<'b>(
        self,
        headers: Box<Iterator<Item = Header> + 'a>,
    ) -> Writer<'a, 'b, 'c, 'd, Formatter<'b>> {
        self.formatter(Formatter::default()).headers(headers)
    }

    /// Sets a custom beginning marker where the table of contents will be inserted.
    ///
    /// The default beginning marker is `"<!-- toc -->"`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use mtoc_parser::{Formatter, WriterBuilder};
    /// use std::str;
    ///
    /// let input = "<!-- gotoc -->\n\n# Title\n## Intro\nHello.\n";
    /// let mut output = Vec::new();
    ///
    /// WriterBuilder::new(&input)
    ///     .begin_marker("<!-- gotoc -->")
    ///     .write(&mut output)
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     "<!-- gotoc -->\n\n- [Intro](#intro)\n\n<!-- tocstop -->\n\n# Title\n## Intro\nHello.\n",
    ///     str::from_utf8(&output).unwrap()
    /// );
    /// ```
    pub fn begin_marker(mut self, begin_marker: &'c str) -> Self {
        self.begin_marker = begin_marker;
        self
    }

    /// Sets a custom ending marker up to where the table of contents will be inserted.
    ///
    /// The default ending marker is `"<!-- tocstop -->"`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use mtoc_parser::{Formatter, WriterBuilder};
    /// use std::str;
    ///
    /// let input = "<!-- toc -->\n\n# Title\n## Intro\nHello.\n";
    /// let mut output = Vec::new();
    ///
    /// WriterBuilder::new(&input)
    ///     .end_marker("<!-- done -->")
    ///     .write(&mut output)
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     "<!-- toc -->\n\n- [Intro](#intro)\n\n<!-- done -->\n\n# Title\n## Intro\nHello.\n",
    ///     str::from_utf8(&output).unwrap()
    /// );
    /// ```
    pub fn end_marker(mut self, end_marker: &'d str) -> Self {
        self.end_marker = end_marker;
        self
    }

    /// Writes the Markdown document with inlined table of contents to the provided 'writer'.AsMut
    ///
    /// Note that this method will consume the iterator of [`Header`]s and returns an internal type
    /// which does not provide the `write` method. To re-populate the iterator, simply call
    /// [`headers`] again and the resulting type will again provide `write`. See the unit tests in
    /// this module for more detailed examples.
    ///
    /// [`Header`]: struct.Header.html
    /// [`headers`]: #fn.headers
    pub fn write<'b, W: Write>(
        self,
        writer: &mut W,
    ) -> io::Result<WriterFormatBuilder<'a, 'b, 'c, 'd, Formatter<'b>>> {
        let formatter = Formatter::default();
        let headers = crate::headers(self.src)
            .filter(|h| h.level() > 1)
            .map(Header::promote);

        self.formatter(formatter)
            .headers(Box::new(headers))
            .write(writer)
    }
}

pub struct WriterFormatBuilder<'a, 'b, 'c, 'd, F>
where
    F: Format + 'b,
{
    src: &'a str,
    formatter: F,
    begin_marker: &'c str,
    end_marker: &'d str,
    phantom: PhantomData<&'b F>,
}

impl<'a, 'b, 'c, 'd, F> WriterFormatBuilder<'a, 'b, 'c, 'd, F>
where
    F: Format + 'b,
{
    pub fn formatter<G>(self, formatter: G) -> WriterFormatBuilder<'a, 'b, 'c, 'd, G>
    where
        G: Format + 'b,
    {
        WriterFormatBuilder {
            src: self.src,
            begin_marker: self.begin_marker,
            end_marker: self.end_marker,
            phantom: PhantomData,
            formatter,
        }
    }

    pub fn headers(self, headers: Box<Iterator<Item = Header> + 'a>) -> Writer<'a, 'b, 'c, 'd, F> {
        Writer {
            src: self.src,
            formatter: self.formatter,
            begin_marker: self.begin_marker,
            end_marker: self.end_marker,
            phantom: PhantomData,
            headers,
        }
    }

    pub fn begin_marker(mut self, begin_marker: &'c str) -> Self {
        self.begin_marker = begin_marker;
        self
    }

    pub fn end_marker(mut self, end_marker: &'d str) -> Self {
        self.end_marker = end_marker;
        self
    }

    pub fn write<W: Write>(self, writer: &mut W) -> io::Result<Self> {
        let headers = crate::headers(self.src)
            .filter(|h| h.level() > 1)
            .map(Header::promote);
        self.headers(Box::new(headers)).write(writer)
    }
}

pub struct Writer<'a, 'b, 'c, 'd, F>
where
    F: Format + 'b,
{
    src: &'a str,
    formatter: F,
    headers: Box<Iterator<Item = Header> + 'a>,
    begin_marker: &'c str,
    end_marker: &'d str,
    phantom: PhantomData<&'b F>,
}

impl<'a, 'b, 'c, 'd, F> Writer<'a, 'b, 'c, 'd, F>
where
    F: Format + 'b,
{
    pub fn headers(mut self, headers: Box<Iterator<Item = Header> + 'a>) -> Self {
        self.headers = headers;
        self
    }

    pub fn formatter<G>(self, formatter: G) -> Writer<'a, 'b, 'c, 'd, G>
    where
        G: Format + 'b,
    {
        Writer {
            src: self.src,
            headers: self.headers,
            begin_marker: self.begin_marker,
            end_marker: self.end_marker,
            phantom: PhantomData,
            formatter,
        }
    }

    pub fn begin_marker(mut self, begin_marker: &'c str) -> Self {
        self.begin_marker = begin_marker;
        self
    }

    pub fn end_marker(mut self, end_marker: &'d str) -> Self {
        self.end_marker = end_marker;
        self
    }

    pub fn write<W: Write>(
        self,
        writer: &mut W,
    ) -> io::Result<WriterFormatBuilder<'a, 'b, 'c, 'd, F>> {
        let mut parser = Parser::new(self.src).into_offset_iter();

        match begin_marker_eol_idx(self.src, self.begin_marker, &mut parser) {
            Some(begin_marker_eol_idx) => {
                io::copy(
                    &mut Cursor::new(self.src.get(..begin_marker_eol_idx).unwrap()),
                    writer,
                )?;

                writer.write_all(b"\n")?;
                self.formatter.fmt(writer, self.headers)?;
                writer.write_all(b"\n")?;

                match end_marker_sol_idx(self.src, self.end_marker, &mut parser) {
                    Some(end_marker_sol_idx) => {
                        io::copy(
                            &mut Cursor::new(self.src.get(end_marker_sol_idx..).unwrap()),
                            writer,
                        )?;
                    }
                    None => {
                        writer.write_all(format!("{}\n", self.end_marker).as_bytes())?;
                        io::copy(
                            &mut Cursor::new(self.src.get(begin_marker_eol_idx..).unwrap()),
                            writer,
                        )?;
                    }
                }
            }
            None => {
                io::copy(&mut Cursor::new(self.src), writer)?;
            }
        }

        Ok(WriterFormatBuilder {
            src: self.src,
            formatter: self.formatter,
            begin_marker: self.begin_marker,
            end_marker: self.end_marker,
            phantom: PhantomData,
        })
    }
}

fn begin_marker_eol_idx(src: &str, marker: &str, parser: &mut OffsetIter) -> Option<usize> {
    parser
        // Use the markdown parser to find the marker in HTML events only to exclude the same
        // marker appearing in code blocks, etc. Then map the result into an index relative to the
        // raw document slice
        .find_map(|(event, offset)| {
            if let Event::Html(html) = event {
                html.find(marker)
                    .and_then(|html_idx| Some(html_idx + offset.start))
            } else {
                None
            }
        })
        // Find the index just after the marker text followed by a line ending. Account for
        // "newline" and "carriage return/newline" endings.
        .and_then(|idx| {
            let post_marker_idx = idx + marker.len();
            let post_marker_slice = src.get(post_marker_idx..).unwrap();

            if post_marker_slice.starts_with("\r\n") {
                Some(post_marker_idx + 2)
            } else if post_marker_slice.starts_with('\n') {
                Some(post_marker_idx + 1)
            } else {
                None
            }
        })
}

fn end_marker_sol_idx(src: &str, marker: &str, parser: &mut OffsetIter) -> Option<usize> {
    parser
        // Use the markdown parser to find the marker in HTML events only to exclude the same
        // marker appearing in code blocks, etc. Then map the result into an index relative to the
        // raw document slice
        .find_map(|(event, offset)| {
            if let Event::Html(html) = event {
                html.find(marker)
                    .and_then(|html_idx| Some(html_idx + offset.start))
            } else {
                None
            }
        })
        // Find the index just before the marker text followed by a line ending. Account for
        // "newline" and "carriage return/newline" endings.
        .and_then(|idx| {
            let post_marker_idx = idx + marker.len();
            let post_marker_slice = src.get(post_marker_idx..).unwrap();

            if post_marker_slice.starts_with("\r\n") || post_marker_slice.starts_with('\n') {
                Some(idx)
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::str;

    const EXPECTED_1: &str = indoc!(
        "<!-- toc -->

        - [Intro](#intro)
        - [Body](#body)
          * [Detail](#detail)
        - [Conclusion](#conclusion)

        <!-- tocstop -->

        # Title
        ## Intro
        ## Body
        ### Detail
        ## Conclusion
        "
    );

    mod markers {
        use super::*;

        #[test]
        fn with_begin_marker() {
            let md = indoc!(
                "<!-- toc -->

                # Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md).write(&mut out).unwrap();

            assert_eq!(EXPECTED_1, str::from_utf8(&out).unwrap());
        }

        #[test]
        fn with_both_markers_squashed() {
            let md = indoc!(
                "<!-- toc -->
                <!-- tocstop -->

                # Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md).write(&mut out).unwrap();

            assert_eq!(EXPECTED_1, str::from_utf8(&out).unwrap());
        }

        #[test]
        fn with_both_markers_vertical_space() {
            let md = indoc!(
                "<!-- toc -->





                <!-- tocstop -->

                # Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md).write(&mut out).unwrap();

            assert_eq!(EXPECTED_1, str::from_utf8(&out).unwrap());
        }

        #[test]
        fn with_outdated_toc() {
            let md = indoc!(
                "<!-- toc -->

                - [Old](#old)

                <!-- tocstop -->

                # Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md).write(&mut out).unwrap();

            assert_eq!(EXPECTED_1, str::from_utf8(&out).unwrap());
        }

        #[test]
        fn with_identical_toc() {
            let mut out = Vec::new();
            WriterBuilder::new(EXPECTED_1).write(&mut out).unwrap();

            assert_eq!(EXPECTED_1, str::from_utf8(&out).unwrap());
        }

        #[test]
        fn with_custom_begin_marker() {
            let md = indoc!(
                "<!-- muzak -->

                # Title
                ## Intro
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .begin_marker("<!-- muzak -->")
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- muzak -->

                    - [Intro](#intro)

                    <!-- tocstop -->

                    # Title
                    ## Intro
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }

        #[test]
        fn with_custom_begin_marker_and_default_end() {
            let md = indoc!(
                "<!-- muzak -->

                - [Old](#old)

                <!-- tocstop -->

                # Title
                ## Intro
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .begin_marker("<!-- muzak -->")
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- muzak -->

                    - [Intro](#intro)

                    <!-- tocstop -->

                    # Title
                    ## Intro
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }

        #[test]
        fn with_custom_end_marker_and_only_default_begin() {
            let md = indoc!(
                "<!-- toc -->

                # Title
                ## Intro
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .end_marker("<!-- stahwp -->")
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- toc -->

                    - [Intro](#intro)

                    <!-- stahwp -->

                    # Title
                    ## Intro
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }

        #[test]
        fn with_custom_end_marker() {
            let md = indoc!(
                "<!-- toc -->

                - [Old](#old)

                <!-- stahwp -->

                # Title
                ## Intro
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .end_marker("<!-- stahwp -->")
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- toc -->

                    - [Intro](#intro)

                    <!-- stahwp -->

                    # Title
                    ## Intro
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }

        #[test]
        fn with_custom_begin_and_end_markers_only_begin() {
            let md = indoc!(
                "<!-- start -->

                # Title
                ## Intro
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .begin_marker("<!-- start -->")
                .end_marker("<!-- stop -->")
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- start -->

                    - [Intro](#intro)

                    <!-- stop -->

                    # Title
                    ## Intro
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }

        #[test]
        fn with_custom_begin_and_end_markers() {
            let md = indoc!(
                "<!-- start -->

                - [Old](#old)

                <!-- stop -->

                # Title
                ## Intro
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .begin_marker("<!-- start -->")
                .end_marker("<!-- stop -->")
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- start -->

                    - [Intro](#intro)

                    <!-- stop -->

                    # Title
                    ## Intro
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }

        #[test]
        fn writes_document_without_markers() {
            let md = indoc!(
                "# Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md).write(&mut out).unwrap();

            assert_eq!(md, str::from_utf8(&out).unwrap());
        }

        #[test]
        fn writes_document_without_begin_markers() {
            let md = indoc!(
                "<!-- tocstop -->

                # Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md).write(&mut out).unwrap();

            assert_eq!(md, str::from_utf8(&out).unwrap());
        }

        #[test]
        fn skips_default_end_with_custom_end() {
            let md = indoc!(
                "<!-- toc -->

                <!-- tocstop -->

                - [Old](#old)

                <!-- tocstop -->

                <!-- stop -->

                <!-- tocstop -->

                # Title
                ## Intro
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .end_marker("<!-- stop -->")
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- toc -->

                    - [Intro](#intro)

                    <!-- stop -->

                    <!-- tocstop -->

                    # Title
                    ## Intro
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }

        #[test]
        fn only_updates_first_toc_block() {
            let md = indoc!(
                "<!-- toc -->

                <!-- tocstop -->

                - [Old](#old)

                <!-- toc -->

                wat

                <!-- tocstop -->

                # Title
                ## Intro
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md).write(&mut out).unwrap();

            assert_eq!(
                indoc!(
                    "<!-- toc -->

                    - [Intro](#intro)

                    <!-- tocstop -->

                    - [Old](#old)

                    <!-- toc -->

                    wat

                    <!-- tocstop -->

                    # Title
                    ## Intro
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }
    }

    mod formatter {
        use super::*;

        #[test]
        fn numbered() {
            let md = indoc!(
                "<!-- toc -->

                # Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .formatter(Formatter::Numbers)
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- toc -->

                    1. [Intro](#intro)
                    1. [Body](#body)
                       1. [Detail](#detail)
                    1. [Conclusion](#conclusion)

                    <!-- tocstop -->

                    # Title
                    ## Intro
                    ## Body
                    ### Detail
                    ## Conclusion
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }

        #[test]
        fn asterisks() {
            let md = indoc!(
                "<!-- toc -->

                # Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .formatter(Formatter::AsteriskBullets)
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- toc -->

                    * [Intro](#intro)
                    * [Body](#body)
                      * [Detail](#detail)
                    * [Conclusion](#conclusion)

                    <!-- tocstop -->

                    # Title
                    ## Intro
                    ## Body
                    ### Detail
                    ## Conclusion
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }

        #[test]
        fn custom_impl() {
            struct Custom;

            impl std::default::Default for Custom {
                fn default() -> Self {
                    Custom
                }
            }

            impl Format for Custom {
                fn fmt<W, I>(&self, writer: &mut W, mut headers: I) -> io::Result<()>
                where
                    W: Write,
                    I: Iterator<Item = Header>,
                {
                    headers.try_for_each(|h| {
                        writeln!(
                            writer,
                            "- {{ title={:?}, anchor={:?}, level={} }}",
                            h.title(),
                            h.anchor(),
                            h.level()
                        )
                    })
                }
            }

            let md = indoc!(
                "<!-- toc -->

                # Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .formatter(Custom)
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    r##"<!-- toc -->

                    - { title="Intro", anchor="#intro", level=1 }
                    - { title="Body", anchor="#body", level=1 }
                    - { title="Detail", anchor="#detail", level=2 }
                    - { title="Conclusion", anchor="#conclusion", level=1 }

                    <!-- tocstop -->

                    # Title
                    ## Intro
                    ## Body
                    ### Detail
                    ## Conclusion
                    "##
                ),
                str::from_utf8(&out).unwrap()
            );
        }

    }

    mod headers {
        use super::*;

        #[test]
        fn with_all_headers() {
            let md = indoc!(
                "<!-- toc -->

                # Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .headers(Box::new(crate::headers(md)))
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- toc -->

                    - [Title](#title)
                      * [Intro](#intro)
                      * [Body](#body)
                        + [Detail](#detail)
                      * [Conclusion](#conclusion)

                    <!-- tocstop -->

                    # Title
                    ## Intro
                    ## Body
                    ### Detail
                    ## Conclusion
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }

        #[test]
        fn only_level_two_headers() {
            let md = indoc!(
                "<!-- toc -->

                # Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .headers(Box::new(
                    crate::headers(md)
                        .filter(|h| h.level() == 2)
                        .map(|h| h.promote()),
                ))
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- toc -->

                    - [Intro](#intro)
                    - [Body](#body)
                    - [Conclusion](#conclusion)

                    <!-- tocstop -->

                    # Title
                    ## Intro
                    ## Body
                    ### Detail
                    ## Conclusion
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }

        #[test]
        fn repopulate_headers() {
            let md = indoc!(
                "<!-- toc -->

                # Title
                ## Intro
                ## Body
                ### Detail
                ## Conclusion
                "
            );

            let mut out = Vec::new();
            WriterBuilder::new(md)
                .headers(Box::new(crate::headers(md)))
                .write(&mut out)
                .unwrap()
                .headers(Box::new(crate::headers(md)))
                .write(&mut out)
                .unwrap();

            assert_eq!(
                indoc!(
                    "<!-- toc -->

                    - [Title](#title)
                      * [Intro](#intro)
                      * [Body](#body)
                        + [Detail](#detail)
                      * [Conclusion](#conclusion)

                    <!-- tocstop -->

                    # Title
                    ## Intro
                    ## Body
                    ### Detail
                    ## Conclusion
                    <!-- toc -->

                    - [Title](#title)
                      * [Intro](#intro)
                      * [Body](#body)
                        + [Detail](#detail)
                      * [Conclusion](#conclusion)

                    <!-- tocstop -->

                    # Title
                    ## Intro
                    ## Body
                    ### Detail
                    ## Conclusion
                    "
                ),
                str::from_utf8(&out).unwrap()
            );
        }
    }
}
