use proconio::source::Source;
use proconio::types::Usize1;
use proconio::{derive_read_source, input};

derive_read_source! {
    #[derive(PartialEq, Debug)]
    struct Weight;
}

derive_read_source! {
    #[derive(PartialEq, Debug)]
    struct Cost(pub(crate) i32);
}

derive_read_source! {
    #[derive(Debug)]
    struct Edge {
        from: usize,
        pub to: Usize1,
        pub(in self) weight: Weight,
        pub(crate) cost: Cost,
    }
}

#[test]
fn derive() {
    let source = Source::from("  12 32 35");
    input! {
        from source,
        edge: Edge,
    }

    assert_eq!(edge.from, 12);
    assert_eq!(edge.to, 31);
    assert_eq!(edge.weight, Weight);
    assert_eq!(edge.cost, Cost(35));
}
