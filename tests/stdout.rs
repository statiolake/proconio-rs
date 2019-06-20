use proconio::{flush_output, output, outputln};

#[test]
fn main() {
    output!("hello, ");
    outputln!("world!");
    outputln!("hello, world!");
    output!("{}{} {}!", 'h', "ello", "world");
    outputln!("{}", 123456789);

    flush_output();
}
