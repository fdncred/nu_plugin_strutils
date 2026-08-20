//! Shared helpers for command tests that run through `PluginTest`.

use crate::StrutilsPlugin;
use nu_plugin_test_support::PluginTest;
use nu_protocol::{ShellError, Span, Value};

pub fn eval(code: &str) -> Result<Value, ShellError> {
    PluginTest::new("strutils", StrutilsPlugin.into())?
        .eval(code)?
        .into_value(Span::test_data())
}

pub fn eval_str(code: &str) -> String {
    match eval(code) {
        Ok(Value::String { val, .. }) => val,
        Ok(other) => panic!("expected string from `{code}`, got {other:?}"),
        Err(err) => panic!("eval failed for `{code}`: {err}"),
    }
}

pub fn eval_int(code: &str) -> i64 {
    match eval(code) {
        Ok(Value::Int { val, .. }) => val,
        Ok(other) => panic!("expected int from `{code}`, got {other:?}"),
        Err(err) => panic!("eval failed for `{code}`: {err}"),
    }
}

pub fn eval_list(code: &str) -> Vec<Value> {
    match eval(code) {
        Ok(Value::List { vals, .. }) => vals.to_vec(),
        Ok(other) => panic!("expected list from `{code}`, got {other:?}"),
        Err(err) => panic!("eval failed for `{code}`: {err}"),
    }
}

pub fn eval_err(code: &str) -> String {
    match eval(code) {
        Err(err) => err.to_string(),
        Ok(Value::Error { error, .. }) => error.to_string(),
        Ok(other) => panic!("expected error from `{code}`, got {other:?}"),
    }
}

pub fn assert_error_contains(code: &str, needle: &str) {
    let msg = eval_err(code);
    assert!(
        msg.contains(needle),
        "error `{msg}` did not contain `{needle}` for `{code}`"
    );
}

/// The engine may reject bad input at parse/type-check time (`Err`) or the
/// command may return `Value::Error`. Both count as a rejection.
pub fn assert_rejects(code: &str) {
    match eval(code) {
        Err(_) | Ok(Value::Error { .. }) => {}
        Ok(other) => panic!("expected `{code}` to be rejected, got {other:?}"),
    }
}
