use day_11;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, day_11::add(2, 2));
}
