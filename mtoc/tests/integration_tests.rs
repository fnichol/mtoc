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
fn check_and_output_conflict() {
    mtoc()
        .arg("--check")
        .arg("--output")
        .arg("nope.md")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            (predicate::str::contains(
                "The argument '--output <OUTPUT>' cannot be used with '--check'",
            )
            .or(predicate::str::contains(
                "The argument '--check' cannot be used with '--output <OUTPUT>'",
            )))
            .and(predicate::str::contains("USAGE:")),
        );
}

#[test]
fn in_place_and_output_conflict() {
    mtoc()
        .arg("--in-place")
        .arg("--output")
        .arg("nope.md")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            (predicate::str::contains(
                "The argument '--output <OUTPUT>' cannot be used with '--in-place'",
            )
            .or(predicate::str::contains(
                "The argument '--in-place' cannot be used with '--output <OUTPUT>'",
            )))
            .and(predicate::str::contains("USAGE:")),
        );
}

#[test]
fn in_place_and_check_conflict() {
    mtoc()
        .arg("--in-place")
        .arg("--check")
        .arg("nope.md")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            (predicate::str::contains("The argument '--in-place' cannot be used with '--check'")
                .or(predicate::str::contains(
                    "The argument '--check' cannot be used with '--in-place'",
                )))
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

#[test]
fn check_identical() {
    mtoc()
        .arg("--check")
        .arg("simple-current.md")
        .assert()
        .success()
        .stdout("")
        .stderr("");
}

#[test]
fn check_new_format_differs() {
    mtoc()
        .arg("--check")
        .arg("--format")
        .arg("asterisks")
        .arg("simple-current.md")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            "\
Diff in simple-current.md at line 2:
 
 <!-- toc -->
 
-- [Introduction](#introduction)
-- [Body](#body)
+* [Introduction](#introduction)
+* [Body](#body)
   * [Detail](#detail)
-- [Conclusion](#conclusion)
+* [Conclusion](#conclusion)
 
 <!-- tocstop -->
 
",
        );
}

#[test]
fn check_new_missing_toc_differs() {
    mtoc()
        .arg("--check")
        .arg("simple-new.md")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            "\
Diff in simple-new.md at line 2:
 
 <!-- toc -->
 
+- [Introduction](#introduction)
+- [Body](#body)
+  * [Detail](#detail)
+- [Conclusion](#conclusion)
+
+<!-- tocstop -->
+
 ## Introduction
 
 Introduction content.
",
        );
}
