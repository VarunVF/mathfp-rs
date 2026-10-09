mod common;

use common::{assert_bool, assert_list, assert_nil, assert_number, assert_str};
use mathfp::{execute_or_panic, runtime::RuntimeValue};

#[test]
fn test_io() {
    assert_list("args()", &[]);
    assert_nil("print(\"\")");
    assert_nil("println(\"\")");

    // just ensuring file functions are defined
    execute_or_panic("read_file", &[]);
    execute_or_panic("write_file", &[]);
}

#[test]
fn test_math() {
    assert_number("sin(0.0)", 0.0);
    assert_number("cos(0.0)", 1.0);
    assert_number("sqrt(9.0)", f64::sqrt(9.0));
    assert_number("floor(9.7)", 9.0);
    assert_number("ceil(9.7)", 10.0);
}

#[test]
fn test_timing() {
    assert!(matches!(
        execute_or_panic("time(clock, nil)", &[]),
        RuntimeValue::Number(_)
    ));

    assert!(matches!(
        execute_or_panic("clock()", &[]),
        RuntimeValue::Number(_)
    ));
}

#[test]
fn test_type() {
    assert_str("type(1)", "number");
    assert_str("type(\"hi\")", "str");
    assert_str("type(true)", "bool");
    assert_str("type(type)", "function");
    assert_str("type(__type)", "function");
    assert_str("type([])", "list");
    assert_str("type(nil)", "nil");
}

#[test]
fn test_bool() {
    assert_bool("bool(1)", true);
    assert_bool("bool(-1)", true);
    assert_bool("bool(7.2)", true);
    assert_bool("bool(0)", false);

    assert_bool("bool(\"hi\")", true);
    assert_bool("bool(\"\")", false);

    assert_bool("bool(true)", true);
    assert_bool("bool(false)", false);

    assert_bool("bool(type)", true);

    assert_bool("bool(__type)", true);

    assert_bool("bool([1])", true);
    assert_bool("bool([])", false);

    assert_bool("bool(nil)", false);
}

#[test]
fn test_str() {
    assert_str("str(45.2)", "45.2");
    assert_str("str(\"hello\")", "\"hello\"");
    assert_str("str(true)", "true");
    assert_str("str(false)", "false");
    assert_str("str(str)", "<function in x>");
    assert_str("str(__str)", "<native function __str>");
    assert_str("str([1, 2, 3])", "[1, 2, 3]");
    assert_str("str(nil)", "nil");
}

#[test]
fn test_map() {
    assert_list(
        "map(x |-> 2*x, [1, 2, 3])",
        &[
            RuntimeValue::Number(2.0),
            RuntimeValue::Number(4.0),
            RuntimeValue::Number(6.0),
        ],
    );

    assert_list("map(x |-> x, [])", &[]);
}

#[test]
fn test_filter() {
    assert_list(
        "filter(x |-> x >= 2, [0, 1, 2, 3])",
        &[RuntimeValue::Number(2.0), RuntimeValue::Number(3.0)],
    );

    assert_list(
        "filter(_ |-> true, [1, 2, 3])",
        &[
            RuntimeValue::Number(1.0),
            RuntimeValue::Number(2.0),
            RuntimeValue::Number(3.0),
        ],
    );

    assert_list("map(_ |-> true, [])", &[]);
    assert_list("map(_ |-> false, [])", &[]);
}

#[test]
fn test_foldl() {
    assert_number("foldl(x |-> y |-> x - y, 0, [1, 2])", -3.0);
    assert_number("foldl(x |-> y |-> x + y, 0, [1, 2])", 3.0);
    assert_number("foldl(x |-> y |-> x + y, 0, [])", 0.0);
}

#[test]
fn test_foldr() {
    assert_number("foldr(x |-> y |-> x - y, 0, [1, 2])", -1.0);
    assert_number("foldr(x |-> y |-> x + y, 0, [1, 2])", 3.0);
    assert_number("foldr(x |-> y |-> x + y, 0, [])", 0.0);
}

#[test]
fn test_len() {
    assert_number("len(\"\")", 0.0);
    assert_number("len([])", 0.0);
    assert_number("len(\"hello\")", 5.0);
    assert_number("len([1, 2, 3])", 3.0);
}

#[test]
fn test_get() {
    assert_str("get(\"hello\", 0)", "h");
    assert_number("get([1, 2, 3], 0)", 1.0);
}

#[test]
#[should_panic(expected = "out of bounds for str")]
fn test_get_string_out_of_bounds() {
    execute_or_panic("get(\"hello\", 100)", &[]);
}

#[test]
#[should_panic(expected = "out of bounds for list")]
fn test_get_list_out_of_bounds() {
    execute_or_panic("get([1, 2, 3], 100)", &[]);
}

#[test]
fn test_slice() {
    assert_str("slice(\"hello world\", 0, 5)", "hello");
    assert_list(
        "slice([6, 2, 5, 0], 0, 3)",
        &[
            RuntimeValue::Number(6.0),
            RuntimeValue::Number(2.0),
            RuntimeValue::Number(5.0),
        ],
    );
}

#[test]
fn test_split() {
    assert_list(
        "split(\"hello world\", \" \")",
        &[
            RuntimeValue::String("hello".to_string()),
            RuntimeValue::String("world".to_string()),
        ],
    );
}

#[test]
fn test_join() {
    assert_str("join([\"hello\", \"world\"], \" \")", "hello world");
}

#[test]
fn test_range() {
    assert_list(
        "range(0, 5)",
        &[
            RuntimeValue::Number(0.0),
            RuntimeValue::Number(1.0),
            RuntimeValue::Number(2.0),
            RuntimeValue::Number(3.0),
            RuntimeValue::Number(4.0),
        ],
    );

    assert_list(
        "range(-2, 3)",
        &[
            RuntimeValue::Number(-2.0),
            RuntimeValue::Number(-1.0),
            RuntimeValue::Number(0.0),
            RuntimeValue::Number(1.0),
            RuntimeValue::Number(2.0),
        ],
    );

    assert_list("range(0, 0)", &[]);
}
