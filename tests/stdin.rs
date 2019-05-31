use proconio::input;
use proconio::source::BufferedSource;

#[test]
#[ignore]
fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    input! {
        from BufferedSource::new(stdin),
        i: u8,
        j: u8,
    }
    eprintln!("{} {}", i, j);
}
