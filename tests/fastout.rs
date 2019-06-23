use proconio_derive::fastout;

#[fastout]
fn foo() -> i32 {
    println!("4");
    3
}

#[fastout]
fn main() {
    let clo = || "AtCoder"; // OK;
    let name = clo();
    // let _err = || println!("hello"); // error even it's not spown
    // std::thread::spawn(|| {
    //     std::println!("hello");
    // }); // of course error
    println!("hello, world, {}!", name);
    std::println!("{}", foo());
    print!("{}{}, ", 'h', "ello"); // "hello"       (no newline)
    println!("{}!", "world"); // "world!\n"
    println!("{}", 123456789); // "123456789\n"
}
