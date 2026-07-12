// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use proconio::read_value_interactive;

fn test_stdin() {
    // read_value_interactive! must be usable in expression position
    let n = read_value_interactive!(usize);
    println!("{n}");

    let mut sum = 0;
    for _ in 0..n {
        sum += read_value_interactive!(u32);
    }
    println!("{sum}");
}

fn test_for(input: &str, expected_stdout: &str) {
    use assert_cli::Assert;
    use std::env::args;
    Assert::command(&[&*args().next().unwrap(), "foo"])
        .stdin(input)
        .stdout()
        .is(expected_stdout)
        .and()
        .stderr()
        .is("")
        .unwrap();
}

fn main() {
    use std::env::args;
    if args().len() == 1 {
        test_for("3\n2 3 7\n", "3\n12\n");
        return;
    }

    test_stdin();
}
