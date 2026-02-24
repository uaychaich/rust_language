use super::*;

#[test]
fn mulcase1() {
    let result = mul(2, 2);
    assert_eq!(result, 4);
}
#[test]
fn mulcase2() {
    let result = mul(3, 2);
    assert_eq!(result, 6);
}

#[test]
fn mulcase3() {
    let result = mul(3, 3);
    assert_eq!(result, 9);
}