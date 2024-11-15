use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, ShellError, Signature, Span, Type, Value};
use textwrap::dedent;

use crate::StrutilsPlugin;

pub struct StrDedent;

impl SimplePluginCommand for StrDedent {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str dedent"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Removes common leading whitespace from each line."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["convert", "ascii", "indent", "untab"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Dedent string",
            example: "'     1st line\r\n       2nd line\r\n     3rd line\r\n' | str dedent",
            result: Some(Value::test_string("1st line\n  2nd line\n3rd line\n")),
        }]
    }

    fn run(
        &self,
        _plugin: &StrutilsPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        Ok(do_dedent(input, call.head))
    }
}

fn do_dedent(input: &Value, head: Span) -> Value {
    match input {
        Value::String { val, .. } => Value::string(dedent(val), head),
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

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrDedent)
}
