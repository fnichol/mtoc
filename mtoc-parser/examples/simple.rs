// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use mtoc_parser::{headers, Formatter};

const MD: &str = "# Title\n## Introduction\n## Body\n### Detail\n### Detail\n## Conclusion";

/// Parse an inline Markdown document and output a table of contents to standard out.
fn main() {
    Formatter::default()
        .fmt(&mut std::io::stdout(), headers(MD))
        .unwrap();
}
