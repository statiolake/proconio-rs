use proconio_derive::fastout;

#[fastout]
fn foo() -> i32 {
    println!("4");
    3
}

#[fastout]
#[test]
fn main() {
    print!("hello, world!");
    std::println!("{}", foo());
    print!("{}{}, ", 'h', "ello"); // "hello"       (no newline)
    println!("{}!", "world"); // "world!\n"
    println!("{}", 123456789); // "123456789\n"
}
