mod common;

use common::{assert_bool, assert_value};
use mathfp::execute_or_panic;
use mathfp::runtime::RuntimeValue;

#[test]
fn test_numeric_ops() {
    assert_value("42 + 5 - 3 / 2 + 1 * 5", RuntimeValue::Number(50.5));
}

#[test]
fn test_numeric_comparison() {
    assert_bool("2 < 3", true);
    assert_bool("2 <= 2", true);
    assert_bool("4 > 3", true);
    assert_bool("4 >= 4", true);
    assert_bool("12 != 71", true);
    assert_bool("42 == 42", true);
}

#[test]
fn test_string_op() {
    assert_value(
        "\"hello\" + \" world\"",
        RuntimeValue::String("hello world".into()),
    );
}

#[test]
fn test_different_type_compare() {
    assert_bool("5 == 5", true);
    assert_bool("sin >= \"sin\"", false);
}

#[test]
#[should_panic(expected = "Unsupported types for '*'")]
fn test_invalid_type_op() {
    execute_or_panic("\"hello\" * 67", &[]);
}

#[test]
fn test_nil_ops() {
    assert_bool("5 == 5", true);
    assert_bool("5 == 5.0", true); // using f64 internally
    assert_bool("nil == nil", true);
    assert_bool("5 == nil", false);
}

#[test]
fn test_off_by_one() {
    assert_bool("10 > 5", true);
    assert_bool("10 >= 10", true);
    assert_bool("5 < 10", true);
    assert_bool("5 <= 5", true);
    assert_bool("5 != 10", true);
}
