use app::hello_web;
use web::web_lib_hello;

#[test]
fn test_my() {
    web_lib_hello();
    assert_eq!(1, 1);
}

#[test]
fn test_my2() {
    hello_web();
    assert_eq!(1, 1);
}
