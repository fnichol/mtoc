// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use assert_cmd::crate_name;
use assert_cmd::prelude::*;
use std::process::Command;

pub const BIN_NAME: &str = crate_name!();

pub fn mtoc() -> Command {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.current_dir("tests/fixtures");
    cmd
}
