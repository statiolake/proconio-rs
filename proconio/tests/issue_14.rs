// The test target source is at issue_14_target.rs

use proconio::fastout;

#[fastout]
#[allow(clippy::match_single_binding)]
fn test_issue_14() {
    println!("-1");
    match 0 {
        _ => println!("1"),
    }
}

fn main() {
    use assert_cli::Assert;
    use std::env::args;

    // relaunch the app to capture standard output
    if args().len() == 1 {
        Assert::command(&[&*args().next().unwrap(), "foo"])
            .stdout()
            .is("-1\n1\n")
            .and()
            .stderr()
            .is("")
            .unwrap();
        return;
    }

    test_issue_14();
}
