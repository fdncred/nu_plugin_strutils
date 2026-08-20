use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, ShellError, Signature, Span, SyntaxShape, Type, Value,
};

use super::extract;
use crate::StrutilsPlugin;

pub struct StrBetween;

impl SimplePluginCommand for StrBetween {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str between"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .required("left", SyntaxShape::String, "Left delimiter.")
            .required("right", SyntaxShape::String, "Right delimiter.")
            .switch(
                "last",
                "Use the last left delimiter, then the next right delimiter after it.",
                Some('l'),
            )
            .switch(
                "strict",
                "Error if either delimiter is not found.",
                Some('s'),
            )
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Return the substring between two delimiters."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["split", "substring", "extract", "slice"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Take the text between brackets",
                example: r#""foo[bar]baz" | str between "[" "]""#,
                result: Some(Value::test_string("bar")),
            },
            Example {
                description: "Take the text between two markers",
                example: r#""id=42;" | str between "id=" ";""#,
                result: Some(Value::test_string("42")),
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
        let left: String = call.req(0)?;
        let right: String = call.req(1)?;
        let last = call.has_flag("last")?;
        let strict = call.has_flag("strict")?;

        if left.is_empty() || right.is_empty() {
            return Err(LabeledError::new("delimiter cannot be empty")
                .with_label("provide non-empty left and right delimiters", call.head));
        }

        do_between(input, &left, &right, last, strict, call.head)
    }
}

fn do_between(
    input: &Value,
    left: &str,
    right: &str,
    last: bool,
    strict: bool,
    head: Span,
) -> Result<Value, LabeledError> {
    match input {
        Value::String { val, .. } => match extract::between(val, left, right, last) {
            Some(s) => Ok(Value::string(s, head)),
            None if strict => Err(LabeledError::new("delimiter not found")
                .with_label("the input does not contain both delimiters", head)),
            None => Ok(Value::string(String::new(), head)),
        },
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

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrBetween)
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{assert_error_contains, assert_rejects, eval_str};

    #[test]
    fn missing_returns_empty() {
        assert_eq!(eval_str(r#""nope" | str between "[" "]""#), "");
    }

    #[test]
    fn last_pair() {
        assert_eq!(eval_str(r#""[a] [b]" | str between --last "[" "]""#), "b");
    }

    #[test]
    fn empty_between_delimiters() {
        assert_eq!(eval_str(r#""[]" | str between "[" "]""#), "");
    }

    #[test]
    fn strict_errors_when_missing() {
        assert_error_contains(
            r#""nope" | str between --strict "[" "]""#,
            "delimiter not found",
        );
    }

    #[test]
    fn empty_delimiter_errors() {
        assert_error_contains(r#""abc" | str between "" "]""#, "delimiter cannot be empty");
    }

    #[test]
    fn rejects_non_string_input() {
        assert_rejects(r#"42 | str between "[" "]""#);
    }
}
