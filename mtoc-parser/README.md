# mtoc-parser

|                 |                                             |
| --------------: | ------------------------------------------- |
|              CI | [![CI Status][badge-ci-overall]][ci]        |
|  Latest Version | [![Latest version][badge-version]][crate]   |
|   Documentation | [![Documentation][badge-docs]][docs]        |
| Crate Downloads | [![Crate downloads][badge-crate-dl]][crate] |
|         License | [![Crate license][badge-license]][github]   |

**Table of Contents**

<!-- toc -->

- [About](#about)
- [Usage](#usage)
  - [Quick Example](#quick-example)
  - [Headers Iterator](#headers-iterator)
  - [Table of Contents Formatting](#table-of-contents-formatting)
- [Related Projects and References](#related-projects-and-references)
- [License](#license)
  - [Contribution](#contribution)

<!-- tocstop -->

A library to write a hierarchical table of contents into a Markdown
([CommonMark]) document.

[commonmark]: https://commonmark.org/

## About

This library parses a Markdown ([CommonMark]) string slice and generates an
`Iterator` of [`Header`] entries which correspond to the heading structure of
the document. Each heading's level is captured, its title is normalized for
Markdown output, and a URL anchor slug is generated. The title and anchor slug
conform to the auto-generated links produced by GitHub Markdown rendering and
Gists. The `Header`s can be consumed, mutated, transformed, filtered over
trivially as they are presented via an `Iterator`. A [`Formatter`] is provided
which can consume `Header`s and output a formatted table of contents to a
'writer' which implements the `Write` trait. Finally, a [`WriterBuilder`] is
provided which combines all of the above (with reasonable defaults) and writes
the table of contents inlined into the source Markdown document to a provided
'writer'.

[commonmark]: https://commonmark.org/
[`formatter`]: enum.Formatter.html
[`header`]: struct.Header.html
[`writerbuilder`]: struct.WriterBuilder.html

## Usage

Add `mtoc-parser` to your `Cargo.toml`:

```toml
[dependencies]
mtoc-parser = "0.1.0"
```

### Quick Example

To parse a Markdown ([CommonMark]) string slice and output the document with a
table of contents to the standard output stream, use the default
[`WriterBuilder`] behavior:

```rust
use mtoc_parser::WriterBuilder;

let input =
    "# Title\n\n<!-- toc -->\n\n## Intro\n## Body\n### Detail\n### Detail\n## Conclusion\n";

WriterBuilder::new(input)
    .write(&mut std::io::stdout().lock())
    .unwrap();
```

which would write the following on standard output:

```markdown
# Title

<!-- toc -->

- [Intro](#intro)
- [Body](#body)
  - [Detail](#detail)
  - [Detail](#detail-1)
- [Conclusion](#conclusion)

<!-- tocstop -->

## Intro

## Body

### Detail

### Detail

## Conclusion
```

[commonmark]: https://commonmark.org/
[`writerbuilder`]: struct.WriterBuilder.html

### Headers Iterator

The [`headers`] function returns a [`Headers`] which is an `Iterator` over
[`Header`]s so it can be used in combination with any iterator adapters and
combinators:

```rust
let input = "# Title\n## Introduction\n## Body\n### Detail\n### Detail\n## Conclusion";

let titles = headers(&input)
    .filter(|header| header.level() == 2)
    .map(Header::into_title)
    .collect::<Vec<_>>();

assert_eq!("Introduction", titles[0]);
assert_eq!("Body", titles[1]);
assert_eq!("Conclusion", titles[2]);
```

```rust
let input = "# Title\n## Introduction\n## Body\n### Detail\n### Detail\n## Conclusion";

// Drop the title header and promote all remaining header levels so that header
// level 2 becomes 1, etc. Finally, collect into a `Vec<Header>`.
let titles = headers(&input)
    .filter(|header| header.level() > 1)
    .map(Header::promote)
    .collect::<Vec<_>>();

assert_eq!("Introduction", titles[0].title());
assert_eq!("#introduction", titles[0].anchor());
assert_eq!(1, titles[0].level());
assert_eq!("[Introduction](#introduction)", format!("{}", titles[0]));
```

[`header`]: struct.Header.html
[`headers`]: struct.Headers.html
[`headers`]: fn.headers.html

### Table of Contents Formatting

To output a table of contents to the standard output stream, you can use the
default [`Formatter`] and the [`headers`] function together:

```rust
use mtoc_parser::{headers, Format, Formatter};

let input = "# Title\n## Introduction\n## Body\n### Detail\n### Detail\n## Conclusion";

Formatter::default()
    .fmt(&mut std::io::stdout(), headers(input))
    .unwrap();
```

which would write the following on standard output:

```markdown
- [Title](#title)
  - [Introduction](#introduction)
  - [Body](#body)
    - [Detail](#detail)
    - [Detail](#detail-1)
  - [Conclusion](#conclusion)
```

The `Formatter` writes output to a 'writer' which implements the `Write` trait,
so you can also format a table of contents in memory:

```rust
use mtoc_parser::{headers, Format, Formatter};
use std::str;

let input = "# Title";
let mut output = Vec::new();

Formatter::default()
    .fmt(&mut output, headers(input))
    .unwrap();

assert_eq!("- [Title](#title)\n", str::from_utf8(&output).unwrap());
```

To format using the default, `AlternatingBullets`:

```rust
let mut output = Vec::new();
let iter = headers("# Level 1\n## Level 2\n### Level 3");

Formatter::AlternatingBullets.fmt(&mut output, iter).unwrap();

let mut lines = str::from_utf8(&output).unwrap().lines();

assert_eq!(Some("- [Level 1](#level-1)"), lines.next());
assert_eq!(Some("  * [Level 2](#level-2)"), lines.next());
assert_eq!(Some("    + [Level 3](#level-3)"), lines.next());
```

To format with numbering:

```rust
let mut output = Vec::new();
let iter = headers("# Level 1\n## Level 2\n### Level 3");

Formatter::Numbers.fmt(&mut output, iter).unwrap();

let mut lines = str::from_utf8(&output).unwrap().lines();

assert_eq!(Some("1. [Level 1](#level-1)"), lines.next());
assert_eq!(Some("   1. [Level 2](#level-2)"), lines.next());
assert_eq!(Some("      1. [Level 3](#level-3)"), lines.next());
```

Or to format with a custom string:

```rust
let mut output = Vec::new();
let iter = headers("# Level 1\n## Level 2\n### Level 3");

Formatter::Custom("★").fmt(&mut output, iter).unwrap();

let mut lines = str::from_utf8(&output).unwrap().lines();

assert_eq!(Some("★ [Level 1](#level-1)"), lines.next());
assert_eq!(Some("  ★ [Level 2](#level-2)"), lines.next());
assert_eq!(Some("    ★ [Level 3](#level-3)"), lines.next());
```

The `Formatter` implementation is nothing special, so the headers can be output
by simply consuming the `Headers` iterator:

```rust
let mut output = Vec::new();
let mut iter = headers("# Level 1\n## Level 2\n### Level 3");

iter.try_for_each(|header| writeln!(&mut output, "{}", header));

let mut lines = str::from_utf8(&output).unwrap().lines();

assert_eq!(Some("[Level 1](#level-1)"), lines.next());
assert_eq!(Some("[Level 2](#level-2)"), lines.next());
assert_eq!(Some("[Level 3](#level-3)"), lines.next());
```

## Related Projects and References

- [markdown-toc](https://github.com/jonschlinkert/markdown-toc) by Jon
  Schlinkert ([@jonschlinkert](https://github.com/jonschlinkert)), written in
  JavaScript
- [markdown-toc](https://github.com/sebdah/markdown-toc) by Sebastian Dahlgren
  ([@sebdah](https://github.com/sebdah)), written in Go
- [markdown-toc](https://github.com/pbzweihander/markdown-toc) by Thomas Lee
  ([@pbzweihander](https://github.com/pbzweihander)), written in Rust
- [MarkdownTOC](https://github.com/naokazuterada/MarkdownTOC) by Naokazu Terada
  ([@naokazuterada](https://github.com/naokazuterada)), a SublimeText3 plugin
  written in Python
- [github-markdown-toc](https://github.com/ekalinin/github-markdown-toc) by
  Eugene Kalinin ([@ekalinin](https://github.com/ekalinin)), written in Shell
- [HTML::Pipeline](https://github.com/jch/html-pipeline) by Jerry Cheung
  ([@jch](https://github.com/jch)), filters and utilities for processing GitHub
  HTML, written in Ruby
- [Anchors in Markdown](https://gist.github.com/asabaylus/3071099) Gist with
  good links, discussions and edge conditions

## License

Licensed under either of

- The Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- The MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[badge-ci-overall]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square
[badge-crate-dl]:
  https://img.shields.io/crates/d/mtoc-parser.svg?style=flat-square
[badge-docs]: https://docs.rs/mtoc-parser/badge.svg?style=flat-square
[badge-license]:
  https://img.shields.io/crates/l/mtoc-parser.svg?style=flat-square
[badge-version]:
  https://img.shields.io/crates/v/mtoc-parser.svg?style=flat-square
[ci]: https://cirrus-ci.com/github/fnichol/mtoc
[crate]: https://crates.io/crates/mtoc-parser
[docs]: https://docs.rs/mtoc-parser
[github]: https://github.com/fnichol/mtoc/tree/master/mtoc-parser
[license-apachev2]:
  https://github.com/fnichol/mtoc/blob/master/mtoc-parser/LICENSE-APACHE
