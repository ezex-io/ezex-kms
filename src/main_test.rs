use super::greet;

#[test]
fn test_greet() {
    let result = greet();
    assert_eq!(result, "Hello, world!");
}
