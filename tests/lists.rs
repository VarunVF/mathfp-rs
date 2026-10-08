mod common;

use common::{assert_bool, assert_list};
use mathfp::runtime::RuntimeValue;

#[test]
fn test_empty() {
    assert_list("[]", &[]);
}

#[test]
fn test_single_element() {
    assert_list("[7]", &[RuntimeValue::Number(7.0)]);
}

#[test]
fn test_trailing_comma() {
    assert_list(
        "[2, 3,]",
        &[RuntimeValue::Number(2.0), RuntimeValue::Number(3.0)],
    );
}

#[test]
fn test_same_types() {
    assert_list(
        "[1, 2, 3]",
        &[
            RuntimeValue::Number(1.0),
            RuntimeValue::Number(2.0),
            RuntimeValue::Number(3.0),
        ],
    );
}

#[test]
fn test_different_types() {
    assert_list(
        "[7, \"hello\", true]",
        &[
            RuntimeValue::Number(7.0),
            RuntimeValue::String("hello".to_string()),
            RuntimeValue::Boolean(true),
        ],
    );
}

#[test]
fn test_nesting() {
    assert_list(
        "[7, [\"hello\", true]]",
        &[
            RuntimeValue::Number(7.0),
            RuntimeValue::List {
                elements: vec![
                    RuntimeValue::String("hello".to_string()),
                    RuntimeValue::Boolean(true),
                ],
            },
        ],
    );
}

#[test]
fn test_concat() {
    assert_list(
        "[1] + [2]",
        &[RuntimeValue::Number(1.0), RuntimeValue::Number(2.0)],
    );
}

#[test]
fn test_concat_nested() {
    assert_list(
        "([1] + [2]) + [3]",
        &[
            RuntimeValue::Number(1.0),
            RuntimeValue::Number(2.0),
            RuntimeValue::Number(3.0),
        ],
    );
}

#[test]
fn test_equality() {
    assert_bool("[3, 4] == [3, 4]", true);
    assert_bool("[] == []", true);
    assert_bool("[] == [4, 3]", false);
    assert_bool("[3, 4] == [4, 3]", false);
}

#[test]
fn test_inequality() {
    assert_bool("[3, 4] != [3, 4]", false);
    assert_bool("[] != []", false);
    assert_bool("[] != [4, 3]", true);
    assert_bool("[3, 4] != [4, 3]", true);
}
