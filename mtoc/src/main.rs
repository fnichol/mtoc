// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use mtoc_parser::{headers, Formatter, Header};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() {
    let input = buf_from_file(env::args().nth(1).unwrap());

    Formatter::default()
        .fmt(
            &mut std::io::stdout(),
            headers(&input)
                .filter(|h| h.level() > 1)
                .map(Header::promote),
        )
        .unwrap();
}

fn buf_from_file<P: AsRef<Path>>(path: P) -> String {
    let input = File::open(path).expect("cannot open file for reading");
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input
        .read_to_string(&mut buffer)
        .expect("cannot read file to buffer");

    buffer
}
