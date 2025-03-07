use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, ShellError, Signature, Span, Type, Value};
use slug::slugify;

use crate::StrutilsPlugin;

pub struct StrSlug;

impl SimplePluginCommand for StrSlug {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str slug"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Convert a string to a slug (URL/filename friendly)."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["convert", "slug", "url", "filename"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Slugify a string",
            example: r#""Hello, World!" | str slug"#,
            result: Some(Value::test_string("hello-world")),
        }]
    }

    fn run(
        &self,
        _plugin: &StrutilsPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        Ok(do_slug(input, call.head))
    }
}

fn do_slug(input: &Value, head: Span) -> Value {
    match input {
        Value::String { val, .. } => Value::string(slugify(val), head),
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

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrSlug)
}
