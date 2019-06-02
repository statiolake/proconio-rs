use proconio::input;

#[test]
#[ignore]
fn main() {
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
