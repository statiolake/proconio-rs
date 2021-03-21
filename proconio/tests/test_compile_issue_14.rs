#[test]
fn test_issue_14() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/issue_14.rs");
}
