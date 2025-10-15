use crate::StrutilsPlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, ShellError, Signature, Span, Type, Value};
use shell_words::quote;

pub struct StrShlQuote;

impl SimplePluginCommand for StrShlQuote {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str shl-quote"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Escapes special characters in a string, so that it will retain its literal meaning when used as a part of command in Unix shell."
    }

    fn extra_description(&self) -> &str {
        r"It tries to avoid introducing any unnecessary quotes or escape characters, but specifics regarding quoting style are left unspecified."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["args", "posix"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Quote the string",
                example: r#""" | str shl-quote"#,
                result: Some(Value::test_string("''")),
            },
            Example {
                description: "Quote the string",
                example: r#""'" | str shl-quote"#,
                result: Some(Value::test_string("''\\'''")),
            },
            Example {
                description: "Quote the string",
                example: r#""~root" | str shl-quote"#,
                result: Some(Value::test_string("'~root'")),
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
        Ok(do_quote(input, call.head))
    }
}

fn do_quote(input: &Value, head: Span) -> Value {
    match input {
        Value::String { val, .. } => Value::string(quote(val), head),
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

    // This will automatically run the examples specified in your command and compare their actual
    // output against what was specified in the example.
    //
    // We recommend you add this test to any other commands you create, or remove it if the examples
    // can't be tested this way.

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrShlQuote)
}
