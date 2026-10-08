mod common;

use common::assert_bool;

#[test]
fn test_unary_op_number() {
    // 1 is truthy
    assert_bool("!(!1)", true);
    assert_bool("-5 < 0", true);
}

#[test]
fn test_unary_op_boolean() {
    assert_bool("!true", false);
    assert_bool("!false", true);
}
