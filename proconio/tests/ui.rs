#[rustversion::stable(1.42.0)]
#[test]
fn ui() {
    trybuild::TestCases::new().compile_fail("./tests/ui/**/*.rs");
}
