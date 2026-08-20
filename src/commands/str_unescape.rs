use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, ShellError, Signature, Span, Type, Value};

use crate::StrutilsPlugin;

pub struct StrUnescape;

impl SimplePluginCommand for StrUnescape {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str unescape"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Interpret C / Rust / JSON-style backslash escape sequences."
    }

    fn extra_description(&self) -> &str {
        r#"Supports \n \t \r \\ \" \' \0 \xNN and \u{…}. Invalid sequences produce an error."#
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["escape", "backslash", "unicode", "decode"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Turn escaped newlines into real newlines",
                example: r#"'hello\nworld' | str unescape"#,
                result: Some(Value::test_string("hello\nworld")),
            },
            Example {
                description: "Interpret a unicode escape",
                example: r#"'crab: \u{1F980}' | str unescape"#,
                result: Some(Value::test_string("crab: 🦀")),
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
        do_unescape(input, call.head)
    }
}

fn do_unescape(input: &Value, head: Span) -> Result<Value, LabeledError> {
    match input {
        Value::String { val, .. } => unescaper::unescape(val)
            .map(|s| Value::string(s, head))
            .map_err(|e| {
                LabeledError::new("invalid escape sequence").with_label(e.to_string(), head)
            }),
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

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrUnescape)
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{assert_error_contains, assert_rejects, eval_str};

    #[test]
    fn tab_and_quotes() {
        assert_eq!(eval_str(r#"'a\tb' | str unescape"#), "a\tb");
        assert_eq!(eval_str(r#"'say \"hi\"' | str unescape"#), r#"say "hi""#);
    }

    #[test]
    fn unchanged_without_escapes() {
        assert_eq!(eval_str(r#"'plain' | str unescape"#), "plain");
    }

    #[test]
    fn empty_string() {
        assert_eq!(eval_str(r#"'' | str unescape"#), "");
    }

    #[test]
    fn hex_escape() {
        assert_eq!(eval_str(r#"'A\x42C' | str unescape"#), "ABC");
    }

    #[test]
    fn invalid_escape_errors() {
        assert_error_contains(r#"'bad\u' | str unescape"#, "invalid escape");
    }

    #[test]
    fn rejects_non_string_input() {
        assert_rejects("42 | str unescape");
    }
}
