// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use proconio::derive_readable;
use proconio::input;
use proconio::source::auto::AutoSource;

#[derive_readable]
#[derive(PartialEq, Debug)]
struct Weight;

#[derive_readable]
#[derive(PartialEq, Debug)]
struct Cost(pub(crate) i32);

#[derive_readable]
#[derive(Debug)]
struct Edge {
    from: usize,
    pub to: proconio::marker::Usize1,
    pub(self) weight: Weight,
    pub(crate) cost: Cost,
}

#[test]
fn derive() {
    let source = AutoSource::from("  12 32 35");
    input! {
        from source,
        edge: Edge,
    }

    assert_eq!(edge.from, 12);
    assert_eq!(edge.to, 31);
    assert_eq!(edge.weight, Weight);
    assert_eq!(edge.cost, Cost(35));
}
