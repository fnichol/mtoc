// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use crate::normalize;
use pulldown_cmark::{Event, OffsetIter, Parser, Tag};
use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt;
use std::ops::Range;
use std::usize;

/// Returns an `Iterator` of [`Header`]s (a [`Headers`]) from a Markdown ([CommonMark]) string
/// slice.
///
/// # Allocations
///
/// The underlying [Markdown parser] performs minimal allocations when processing the underlying
/// string slice source, but once a header has been fully captured a `Header` is produced which
/// allocates on the heap. The header's title and anchor link are normalized which involves string
/// manipulation so therefore no longer necessarily corresponds to the underlying raw text in the
/// slice. However, once the parser is complete or when the iterator is fully consumed, the source
/// string slice and any other allocated resources will be reclaimed while still allowing the
/// `Header`s to live.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use mtoc_parser::{headers, Header};
/// let markdown = "# One\n## Two\n## Three";
///
/// // Map each `Header` into its title which is a `String`
/// let mut iter = headers(markdown).map(Header::into_title);
///
/// assert_eq!(Some("One"), iter.next().as_ref().map(String::as_str));
/// assert_eq!(Some("Two"), iter.next().as_ref().map(String::as_str));
/// assert_eq!(Some("Three"), iter.next().as_ref().map(String::as_str));
/// assert_eq!(None, iter.next());
/// ```
///
/// [CommonMark]: https://commonmark.org/
/// [`Header`]: struct.Header.html
/// [`Headers`]: struct.Headers.html
/// [Markdown parser]: https://docs.rs/pulldown-cmark/
pub fn headers(buf: &str) -> Headers {
    Headers {
        slugger: AnchorSlugger::new(),
        iter: Parser::new(buf).into_offset_iter(),
        buf,
    }
}

/// A heading entry from a parsed Markdown ([CommonMark]) document.
///
/// `Header`s are produced via the [`headers`] function from parsing an underlying Markdown string
/// slice. The following is captured for each Markdown header:
///
/// - The headling level is recorded and accessible via the [`level`] method
/// - The heading title is normalized and accessible via the [`title`] method
/// - The heading anchor link is normalized and accessible via the [`anchor`] method
///
/// [`anchor`]: #method.anchor
/// [`headers`]: fn.headers.html
/// [`level`]: #method.level
/// [`title`]: #method.title
#[derive(Debug, PartialEq)]
pub struct Header {
    level: usize,
    title: String,
    anchor: String,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]({})", self.title, self.anchor,)
    }
}

impl Header {
    /// Returns the level of the header.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use mtoc_parser::{headers};
    /// let header = headers("# <blink>A Title</blink>").next().unwrap();
    ///
    /// assert_eq!(1, header.level());
    /// ```
    pub fn level(&self) -> usize {
        self.level
    }

    /// Returns the normalized title of the header.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use mtoc_parser::{headers};
    /// let header = headers("# <blink>A Title</blink>").next().unwrap();
    ///
    /// assert_eq!("A Title", header.title());
    /// ```
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Returns the normalized anchor link of the header.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use mtoc_parser::{headers};
    /// let header = headers("# <blink>A Title</blink>").next().unwrap();
    ///
    /// assert_eq!("#a-title", header.anchor());
    /// ```
    pub fn anchor(&self) -> &str {
        self.anchor.as_str()
    }

    /// Consumes this `Header`, returning the underlying normalized title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use mtoc_parser::{headers};
    /// let header = headers("# A Title").next().unwrap();
    ///
    /// assert_eq!("A Title".to_string(), header.into_title());
    /// ```
    pub fn into_title(self) -> String {
        self.title
    }

    /// Consumes this `Header`, returning the underlying normalized anchor.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use mtoc_parser::{headers};
    /// let header = headers("# A Title").next().unwrap();
    ///
    /// assert_eq!("#a-title".to_string(), header.into_anchor());
    /// ```
    pub fn into_anchor(self) -> String {
        self.anchor
    }

    /// Consumes this `Header`, returning a new `Header` with a level one number lower than the
    /// original.
    ///
    /// If the `Header`'s level is `1` then it will not be further decremented and a level of `1`
    /// will be used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use mtoc_parser::{headers};
    /// let header = headers("### Sub Heading").next().unwrap();
    ///
    /// assert_eq!(1, header.promote().promote().level());
    /// ```
    pub fn promote(self) -> Self {
        Header {
            level: if self.level == 1 {
                self.level
            } else {
                self.level - 1
            },
            title: self.title,
            anchor: self.anchor,
        }
    }

    /// Consumes this `Header`, returning a new `Header` with a level one number higher than the
    /// original.
    ///
    /// The `Header`'s max level is the `6` (according to the [ATX headings spec]) and further
    /// calls to `demote` will keep using this maximum value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use mtoc_parser::{headers};
    /// let header = headers("# Sub Heading").next().unwrap();
    ///
    /// assert_eq!(3, header.demote().demote().level());
    /// ```
    ///
    /// [ATX headings spec]: https://spec.commonmark.org/0.29/#atx-headings
    pub fn demote(self) -> Self {
        Header {
            level: if self.level == 6 {
                self.level
            } else {
                self.level + 1
            },
            title: self.title,
            anchor: self.anchor,
        }
    }
}

/// An iterator of [`Header`]s from an underlying Markdown string slice.
///
/// This `struct` is created by the [`headers`] function. See its documentation for more.
///
/// [`Header`]: struct.Header.html
/// [`headers`]: fn.headers.html
pub struct Headers<'a> {
    slugger: AnchorSlugger,
    iter: OffsetIter<'a>,
    buf: &'a str,
}

impl<'a> Iterator for Headers<'a> {
    type Item = Header;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = &mut self.iter;
        let mut state = State::NoHeader;

        for (event, offset) in iter {
            match event {
                // Found the start of a new header. Capture its level and the ending offset which
                // corresponds to the last character of the raw header text
                Event::Start(Tag::Header(level)) => {
                    state = State::FoundHeader(level, offset.end);
                }
                // Found the end of the header. Only enter if we're processing a header.
                Event::End(Tag::Header(_)) if state.processing_header() => {
                    // If no events were found between the start and end of a header then there is
                    // no raw text and it is empty, so we'll set start offset to the end offset
                    // (range length == 0)
                    if let State::FoundHeader(level, end) = state {
                        state = State::FoundRange(level, Range { start: end, end });
                    }

                    match state {
                        State::FoundRange(level, range) => {
                            let mut raw_header = self.buf.get(range).expect("range should exist");
                            if raw_header.ends_with('\n') {
                                raw_header = raw_header.get(..raw_header.len() - 1).unwrap();
                            }
                            if raw_header.ends_with('\r') {
                                raw_header = raw_header.get(..raw_header.len() - 1).unwrap();
                            }

                            let level = level.try_into().expect("level should not be negative");
                            let title = normalize::titleize(raw_header);
                            let anchor = format!("#{}", self.slugger.slug(raw_header));

                            return Some(Header {
                                level,
                                title,
                                anchor,
                            });
                        }
                        _ => unreachable!("state can only be FoundRange"),
                    }
                }
                // Found the first event inside a header. Capture its starting offset which
                // corresponds to the first character of the raw header text
                _ if state.found_header() => match state {
                    State::FoundHeader(level, end) => {
                        let range = Range {
                            start: offset.start,
                            end,
                        };

                        state = State::FoundRange(level, range);
                    }
                    _ => unreachable!("state can only be FoundHeader"),
                },
                // Skip all other events.
                _ => (),
            }
        }
        None
    }
}

#[derive(Debug)]
enum State {
    NoHeader,
    FoundHeader(i32, usize),
    FoundRange(i32, Range<usize>),
}

impl State {
    fn found_header(&self) -> bool {
        match *self {
            State::FoundHeader(_, _) => true,
            _ => false,
        }
    }

    fn processing_header(&self) -> bool {
        match *self {
            State::FoundHeader(_, _) | State::FoundRange(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct AnchorSlugger(HashSet<String>);

impl AnchorSlugger {
    fn new() -> Self {
        AnchorSlugger(HashSet::new())
    }

    fn slug(&mut self, text: &str) -> &str {
        self.unique_slug(normalize::slugify(text))
    }

    fn unique_slug(&mut self, mut candidate: String) -> &str {
        if self.0.contains(&candidate) {
            let mut x = 1;
            loop {
                let new_candidate = format!("{}-{}", &candidate, x);
                if !self.0.contains(&new_candidate) {
                    candidate = new_candidate;
                    break;
                }
                x += 1;
            }
        }
        self.0.insert(candidate.clone());

        self.0.get(&candidate).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod headers {
        use super::*;

        #[test]
        fn returned_in_order() {
            let md = "# Alpha\n# Bravo\n# Charlie\n# Delta\n# Echo\n# Foxtrot";
            let mut iter = headers(md).map(Header::into_title);

            assert_eq!(Some("Alpha"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("Bravo"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("Charlie"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("Delta"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("Echo"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("Foxtrot"), iter.next().as_ref().map(String::as_str));
            assert_eq!(None, iter.next());
        }

        #[test]
        fn unique_slugs_are_unique() {
            let md = "# Alpha\n# Bravo\n# Charlie\n# Delta\n# Echo\n# Foxtrot";
            let mut iter = headers(md).map(Header::into_anchor);

            assert_eq!(Some("#alpha"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("#bravo"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("#charlie"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("#delta"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("#echo"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("#foxtrot"), iter.next().as_ref().map(String::as_str));
            assert_eq!(None, iter.next());
        }

        #[test]
        fn duplicate_slugs_are_uniqued() {
            let md = "# Alpha\n# Bravo\n## Alpha\n# Delta\n# Alpha\n###### Alpha";
            let mut iter = headers(md).map(Header::into_anchor);

            assert_eq!(Some("#alpha"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("#bravo"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("#alpha-1"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("#delta"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("#alpha-2"), iter.next().as_ref().map(String::as_str));
            assert_eq!(Some("#alpha-3"), iter.next().as_ref().map(String::as_str));
            assert_eq!(None, iter.next());
        }
    }

    mod header {
        use super::*;

        fn header() -> Header {
            Header {
                level: 3,
                title: "A Title to Remember".to_string(),
                anchor: "#a-title-to-remember".to_string(),
            }
        }

        #[test]
        fn level() {
            assert_eq!(3, header().level());
        }

        #[test]
        fn title() {
            assert_eq!("A Title to Remember", header().title());
        }

        #[test]
        fn anchor() {
            assert_eq!("#a-title-to-remember", header().anchor());
        }

        #[test]
        fn into_title() {
            assert_eq!("A Title to Remember", header().into_title());
        }

        #[test]
        fn into_anchor() {
            assert_eq!("#a-title-to-remember", header().into_anchor());
        }

        #[test]
        fn promote() {
            assert_eq!(2, header().promote().level());
        }

        #[test]
        fn promote_floor_is_one() {
            assert_eq!(1, header().promote().promote().promote().promote().level());
        }

        #[test]
        fn demote() {
            assert_eq!(4, header().demote().level());
        }

        #[test]
        fn demote_ceiling_is_six() {
            assert_eq!(6, header().demote().demote().demote().demote().level());
        }

        #[test]
        fn display() {
            assert_eq!(
                "[A Title to Remember](#a-title-to-remember)",
                header().to_string()
            )
        }
    }
}
