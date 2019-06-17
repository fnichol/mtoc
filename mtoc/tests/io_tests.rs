// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

mod helper;

use assert_cmd::assert::Assert;
use assert_cmd::prelude::*;
use helper::*;
use insta::assert_snapshot_matches;
use std::str;

macro_rules! test {
    (
        $name:ident, $fixture_file:expr
    ) => {
        #[test]
        fn $name() {
            let cmd = mtoc().arg($fixture_file).assert().success().stderr("");

            assert_snapshot_matches!(format!("{}.stdout", $fixture_file), stdout(&cmd));
        }
    };
}

fn stdout(cmd: &Assert) -> &str {
    str::from_utf8(&cmd.get_output().stdout).unwrap()
}

test!(simple_new, "simple-new.md");
test!(simple_old, "simple-old.md");
test!(simple_empty, "simple-empty.md");
test!(complex, "complex.md");
