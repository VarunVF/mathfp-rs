use mathfp::execute_env_or_panic;
use mathfp::interpreter::Interpreter;
use mathfp::runtime::RuntimeValue;

// Helper for executing a string where success is expected
fn run_string_helper(input: &str) -> RuntimeValue {
    let interpreter = Interpreter::new();
    execute_env_or_panic(input, &interpreter)
}

#[test]
fn test_empty() {
    let value = run_string_helper("[]");
    assert_eq!(value, RuntimeValue::List { elements: vec![] });
}

#[test]
fn test_single_element() {
    let value = run_string_helper("[7]");
    assert_eq!(
        value,
        RuntimeValue::List {
            elements: vec![RuntimeValue::Number(7.0)]
        }
    );
}

#[test]
fn test_trailing_comma() {
    let value = run_string_helper("[2, 3,]");
    assert_eq!(
        value,
        RuntimeValue::List {
            elements: vec![RuntimeValue::Number(2.0), RuntimeValue::Number(3.0)]
        }
    );
}

#[test]
fn test_same_types() {
    let value = run_string_helper("[1, 2, 3]");
    assert_eq!(
        value,
        RuntimeValue::List {
            elements: vec![
                RuntimeValue::Number(1.0),
                RuntimeValue::Number(2.0),
                RuntimeValue::Number(3.0),
            ]
        }
    );
}

#[test]
fn test_different_types() {
    let value = run_string_helper("[7, \"hello\", true]");
    assert_eq!(
        value,
        RuntimeValue::List {
            elements: vec![
                RuntimeValue::Number(7.0),
                RuntimeValue::String("hello".to_string()),
                RuntimeValue::Boolean(true),
            ]
        }
    );
}

#[test]
fn test_nesting() {
    let value = run_string_helper("[7, [\"hello\", true]]");
    assert_eq!(
        value,
        RuntimeValue::List {
            elements: vec![
                RuntimeValue::Number(7.0),
                RuntimeValue::List {
                    elements: vec![
                        RuntimeValue::String("hello".to_string()),
                        RuntimeValue::Boolean(true),
                    ]
                },
            ]
        }
    );
}

#[test]
fn test_concat() {
    let value = run_string_helper("[1] + [2]");
    assert_eq!(
        value,
        RuntimeValue::List {
            elements: vec![RuntimeValue::Number(1.0), RuntimeValue::Number(2.0),]
        }
    );
}

#[test]
fn test_concat_nested() {
    let value = run_string_helper("([1] + [2]) + [3]");
    assert_eq!(
        value,
        RuntimeValue::List {
            elements: vec![
                RuntimeValue::Number(1.0),
                RuntimeValue::Number(2.0),
                RuntimeValue::Number(3.0),
            ]
        }
    );
}

#[test]
fn test_equality() {
    assert_eq!(
        run_string_helper("[3, 4] == [3, 4]"),
        RuntimeValue::Boolean(true)
    );
    assert_eq!(run_string_helper("[] == []"), RuntimeValue::Boolean(true));
    assert_eq!(
        run_string_helper("[] == [4, 3]"),
        RuntimeValue::Boolean(false)
    );
    assert_eq!(
        run_string_helper("[3, 4] == [4, 3]"),
        RuntimeValue::Boolean(false)
    );
}

#[test]
fn test_inequality() {
    assert_eq!(
        run_string_helper("[3, 4] != [3, 4]"),
        RuntimeValue::Boolean(false)
    );
    assert_eq!(run_string_helper("[] != []"), RuntimeValue::Boolean(false));
    assert_eq!(
        run_string_helper("[] != [4, 3]"),
        RuntimeValue::Boolean(true)
    );
    assert_eq!(
        run_string_helper("[3, 4] != [4, 3]"),
        RuntimeValue::Boolean(true)
    );
}
