use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, ShellError, Signature, Span, SyntaxShape, Type, Value,
};

use crate::StrutilsPlugin;

pub struct StrIncrement;

impl SimplePluginCommand for StrIncrement {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str increment"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .named(
                "by",
                SyntaxShape::Int,
                "Amount to add to the last number (default 1).",
                Some('n'),
            )
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Increment the last run of decimal digits, keeping zero-padding."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["number", "bump", "version", "rename"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Increment a zero-padded filename",
                example: r#""img001.png" | str increment"#,
                result: Some(Value::test_string("img002.png")),
            },
            Example {
                description: "Increment the last number in a version string",
                example: r#""v1.2.9" | str increment"#,
                result: Some(Value::test_string("v1.2.10")),
            },
        ]
    }

    fn run(
        &self,
        _plugin: &StrutilsPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let by: i64 = call.get_flag("by")?.unwrap_or(1);
        do_increment(input, by, call.head)
    }
}

fn do_increment(input: &Value, by: i64, head: Span) -> Result<Value, LabeledError> {
    match input {
        Value::String { val, .. } => increment_last_digits(val, by)
            .map(|s| Value::string(s, head))
            .map_err(|msg| LabeledError::new(msg).with_label(msg, head)),
        Value::Error { .. } => Ok(input.clone()),
        _ => Ok(Value::error(
            ShellError::OnlySupportsThisInputType {
                exp_input_type: "string".into(),
                wrong_type: input.get_type().to_string(),
                dst_span: head,
                src_span: input.span(),
            },
            head,
        )),
    }
}

fn increment_last_digits(s: &str, by: i64) -> Result<String, &'static str> {
    let bytes = s.as_bytes();
    let mut end = None;
    let mut start = 0;

    for (i, b) in bytes.iter().enumerate().rev() {
        if b.is_ascii_digit() {
            if end.is_none() {
                end = Some(i.checked_add(1).ok_or("no digits")?);
            }
            start = i;
        } else if end.is_some() {
            break;
        }
    }

    let end = end.ok_or("no digits in input")?;
    let digits = s.get(start..end).ok_or("no digits in input")?;
    let width = digits.len();
    let n: i128 = digits.parse().map_err(|_| "number is too large")?;
    let new = n.checked_add(i128::from(by)).ok_or("number is too large")?;
    if new < 0 {
        return Err("result would be negative");
    }
    let new_digits = format!("{new:0width$}");
    let mut out = String::with_capacity(s.len().saturating_add(1));
    out.push_str(&s[..start]);
    out.push_str(&new_digits);
    out.push_str(&s[end..]);
    Ok(out)
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrIncrement)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_padding() {
        assert_eq!(
            increment_last_digits("img001.png", 1).unwrap(),
            "img002.png"
        );
    }

    #[test]
    fn grows_when_needed() {
        assert_eq!(increment_last_digits("v1.2.9", 1).unwrap(), "v1.2.10");
    }

    #[test]
    fn decrements() {
        assert_eq!(
            increment_last_digits("img002.png", -1).unwrap(),
            "img001.png"
        );
    }

    #[test]
    fn rejects_negative_result() {
        assert!(increment_last_digits("0", -1).is_err());
    }

    #[test]
    fn rejects_no_digits() {
        assert!(increment_last_digits("file", 1).is_err());
    }

    #[test]
    fn by_flag_adds_custom_amount() {
        assert_eq!(
            super::super::test_support::eval_str(r#""n01" | str increment --by 5"#),
            "n06"
        );
    }

    #[test]
    fn by_flag_can_decrement() {
        assert_eq!(
            super::super::test_support::eval_str(r#""n05" | str increment --by -2"#),
            "n03"
        );
    }

    #[test]
    fn no_digits_errors_through_command() {
        super::super::test_support::assert_error_contains(r#""file" | str increment"#, "no digits");
    }

    #[test]
    fn negative_result_errors_through_command() {
        super::super::test_support::assert_error_contains(
            r#""0" | str increment --by -1"#,
            "negative",
        );
    }

    #[test]
    fn rejects_non_string_input() {
        super::super::test_support::assert_rejects("42 | str increment");
    }
}
