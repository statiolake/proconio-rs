// The test target source is at issue_14_target.rs

use proconio::fastout;

#[fastout]
fn test_issue_14() {
    for i in 0..2 {
        if i % 2 == 0 {
            println!("-1");
            continue;
        }

        let success = 0;
        match success {
            _ => {
                println!("1");
            }
        }
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
