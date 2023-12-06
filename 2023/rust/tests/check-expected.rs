#[test]
fn check_expected() {
    assert!(
        aoc::test::check_results(rust::register::register_runners, "expected.txt", false).unwrap()
    );
}
