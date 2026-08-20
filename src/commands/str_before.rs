use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, ShellError, Signature, Span, SyntaxShape, Type, Value,
};

use super::extract;
use crate::StrutilsPlugin;

pub struct StrBefore;

impl SimplePluginCommand for StrBefore {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str before"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .required("delimiter", SyntaxShape::String, "Substring to search for.")
            .switch(
                "last",
                "Use the last occurrence instead of the first.",
                Some('l'),
            )
            .switch(
                "inclusive",
                "Include the delimiter in the result.",
                Some('i'),
            )
            .switch("strict", "Error if the delimiter is not found.", Some('s'))
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Return the substring before a delimiter."
    }

    fn extra_description(&self) -> &str {
        "If the delimiter is missing, the original string is returned unless --strict is set."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["split", "substring", "prefix", "extract"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Take the text before the first delimiter",
                example: r#""key=value=extra" | str before "=""#,
                result: Some(Value::test_string("key")),
            },
            Example {
                description: "Take the text before the last delimiter",
                example: r#""a/b/c.txt" | str before --last ".""#,
                result: Some(Value::test_string("a/b/c")),
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
        let delimiter: String = call.req(0)?;
        let last = call.has_flag("last")?;
        let inclusive = call.has_flag("inclusive")?;
        let strict = call.has_flag("strict")?;

        if delimiter.is_empty() {
            return Err(LabeledError::new("delimiter cannot be empty")
                .with_label("provide a non-empty delimiter", call.head));
        }

        do_before(input, &delimiter, last, inclusive, strict, call.head)
    }
}

fn do_before(
    input: &Value,
    delimiter: &str,
    last: bool,
    inclusive: bool,
    strict: bool,
    head: Span,
) -> Result<Value, LabeledError> {
    match input {
        Value::String { val, .. } => match extract::before(val, delimiter, last, inclusive) {
            Some(s) => Ok(Value::string(s, head)),
            None if strict => Err(LabeledError::new("delimiter not found")
                .with_label("the input does not contain this delimiter", head)),
            None => Ok(Value::string(val.clone(), head)),
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

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrBefore)
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{assert_error_contains, assert_rejects, eval_str};

    #[test]
    fn missing_delimiter_returns_original() {
        assert_eq!(eval_str(r#""abc" | str before "=""#), "abc");
    }

    #[test]
    fn inclusive_keeps_delimiter() {
        assert_eq!(
            eval_str(r#""key=value" | str before --inclusive "=""#),
            "key="
        );
    }

    #[test]
    fn last_occurrence() {
        assert_eq!(eval_str(r#""a.b.c" | str before --last ".""#), "a.b");
    }

    #[test]
    fn strict_errors_when_missing() {
        assert_error_contains(r#""abc" | str before --strict "=""#, "delimiter not found");
    }

    #[test]
    fn empty_delimiter_errors() {
        assert_error_contains(r#""abc" | str before """#, "delimiter cannot be empty");
    }

    #[test]
    fn rejects_non_string_input() {
        assert_rejects("42 | str before '='");
    }
}
