// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use proconio::{input, is_stdin_empty};

#[test]
#[ignore]
fn stdin() {
    assert!(!is_stdin_empty());
    input! {
        n: usize,
    }
    assert!(!is_stdin_empty());
    eprintln!("{}", n);

    for c in 0..n {
        eprintln!("start {}", c);
        assert!(!is_stdin_empty());
        input! {
            i: isize,
            j: isize,
        }

        eprintln!("{} {}", i, j);
    }
    assert!(is_stdin_empty());
}
