extern crate tddrust;
mod common;

#[test]
fn holding(){
    let larger  = tddrust::Rectangle::new(8, 4);
    let smaller = tddrust::Rectangle::new(2, 3);

    assert!(larger.can_hold(&smaller))
}

#[test]
fn subdirectory_common_test(){
    common::setup();
    assert!(true)
}
