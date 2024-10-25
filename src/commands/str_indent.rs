use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, ShellError, Signature, Span, SyntaxShape, Type, Value,
};
use textwrap::indent;

use crate::StrutilsPlugin;

pub struct StrIndent;

impl SimplePluginCommand for StrIndent {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str indent"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .required(
                "prefix",
                SyntaxShape::String,
                "Prefix used to indent each line with.",
            )
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Indent each line by the given prefix."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["convert", "ascii", "dedent", "tab"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Indent each line with a provided prefix",
            example: r#""First line.\nSecond line.\n" | str indent "1111""#,
            result: Some(Value::test_string("1111First line.\n1111Second line.\n")),
        }]
    }

    fn run(
        &self,
        _plugin: &StrutilsPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let prefix: String = call.req(0)?;

        Ok(do_indent(input, prefix, call.head))
    }
}

fn do_indent(input: &Value, prefix: String, head: Span) -> Value {
    match input {
        Value::String { val, .. } => Value::string(indent(val, &prefix), head),
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

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrIndent)
}
