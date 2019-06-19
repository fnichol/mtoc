// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

mod helper;

use assert_cmd::prelude::*;
use helper::*;
use predicates::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[test]
fn version_short() {
    mtoc()
        .arg("-V")
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(format!("{} {}", BIN_NAME, VERSION))
                .and(predicate::str::contains(format!("binary: {}", BIN_NAME)).not())
                .and(predicate::str::contains(format!("release: {}", VERSION)).not()),
        )
        .stderr("");
}

#[test]
fn version_long() {
    mtoc()
        .arg("--version")
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(format!("{} {}", BIN_NAME, VERSION))
                .and(predicate::str::contains(format!("binary: {}", BIN_NAME)))
                .and(predicate::str::contains(format!("release: {}", VERSION))),
        )
        .stderr("");
}

#[test]
fn in_place_and_output_conflict() {
    mtoc()
        .arg("-i")
        .arg("-o")
        .arg("nope.md")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::contains(
                "The argument '--in-place' cannot be used with '--output <OUTPUT>'",
            )
            .and(predicate::str::contains("USAGE:")),
        );
}

#[test]
fn nonexistent_input_file() {
    // First match is non-Windows and second match is Windows
    mtoc()
        .arg("nonexistent_input_file")
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("No such file or directory").or(
            predicate::str::contains("The system cannot find the file specified"),
        ));
}
