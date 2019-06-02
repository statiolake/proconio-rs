use proconio::input;
// use proconio::source::Source;

#[test]
#[ignore]
fn main() {
    // let stdin = std::io::stdin();
    // let mut source = Source::new(stdin.lock());
    input! {
        n: usize,
    }
    eprintln!("{}", n);

    for c in 0..n {
        eprintln!("start {}", c);
        input! {
            i: isize,
            j: isize,
        }

        eprintln!("{} {}", i, j);
    }
}
