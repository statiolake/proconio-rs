// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use proconio::{input, input_once};

fn test_stdin() {
    input! {
        n: usize,
    }
    println!("{n}");

    // mixing the line-by-line macros and the read-at-once macros must panic
    input_once! {
        m: usize,
    }
    println!("{m}");
}

fn test_panics(input: &str) {
    use assert_cli::Assert;
    use std::env::args;
    Assert::command(&[&*args().next().unwrap(), "foo"])
        .stdin(input)
        .fails()
        .and()
        .stderr()
        .contains("cannot use `input_once!` / `read_value_once!` after")
        .unwrap();
}

fn main() {
    use std::env::args;
    if args().len() == 1 {
        test_panics("1 2\n");
        return;
    }

    test_stdin();
}
