use proconio::input_from_source;
use proconio::source::Source;
use proconio_derive::ReadSource;

#[derive(ReadSource, PartialEq, Debug)]
struct Weight;

#[derive(ReadSource, PartialEq, Debug)]
struct Cost(i32);

#[derive(ReadSource)]
struct Edge {
    from: usize,
    to: usize,
    weight: Weight,
    cost: Cost,
}

fn main() {
    let source = Source::new(&b"  12 32 35"[..]);
    input_from_source! {
        from source,
        edge: Edge,
    }

    assert_eq!(edge.from, 12);
    assert_eq!(edge.to, 32);
    assert_eq!(edge.weight, Weight);
    assert_eq!(edge.cost, Cost(35));
}
