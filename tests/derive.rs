use proconio::input;
use proconio::source::auto::AutoSource;
use proconio_derive::derive_readable;

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
    pub to: proconio::types::Usize1,
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
