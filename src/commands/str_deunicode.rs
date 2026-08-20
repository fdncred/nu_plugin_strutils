use deunicode::deunicode;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, ShellError, Signature, Span, Type, Value};

use crate::StrutilsPlugin;

pub struct StrDeunicode;

impl SimplePluginCommand for StrDeunicode {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str deunicode"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Convert Unicode string to pure ASCII."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["convert", "ascii"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![Example {
            description: "deunicode a string",
            example: "'A…C' | str deunicode",
            result: Some(Value::test_string("A...C")),
        }]
    }

    fn run(
        &self,
        _plugin: &StrutilsPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        Ok(do_deunicode(input, call.head))
    }
}

fn do_deunicode(input: &Value, head: Span) -> Value {
    match input {
        Value::String { val, .. } => Value::string(deunicode(val), head),
        Value::Error { .. } => input.clone(),
        _ => Value::error(
            ShellError::OnlySupportsThisInputType {
                exp_input_type: "string".into(),
                wrong_type: input.get_type().to_string(),
                dst_span: head,
                src_span: input.span(),
            },
            head,
        ),
    }
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrDeunicode)
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{assert_rejects, eval_str};

    #[test]
    fn ascii_is_unchanged() {
        assert_eq!(eval_str(r#"'hello' | str deunicode"#), "hello");
    }

    #[test]
    fn accented_letters() {
        assert_eq!(eval_str(r#"'Café' | str deunicode"#), "Cafe");
    }

    #[test]
    fn empty_string() {
        assert_eq!(eval_str(r#"'' | str deunicode"#), "");
    }

    #[test]
    fn rejects_non_string_input() {
        assert_rejects("42 | str deunicode");
    }
}
