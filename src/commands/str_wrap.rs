use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, ShellError, Signature, Span, SyntaxShape, Type, Value,
};
use textwrap::{fill, Options, WrapAlgorithm};

use crate::StrutilsPlugin;

pub struct StrWrap;

impl SimplePluginCommand for StrWrap {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str wrap"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .switch(
                "optimal-fit",
                "Wrap words using an advanced algorithm with look-ahead.",
                Some('o'),
            )
            .named(
                "width",
                SyntaxShape::Int,
                "The width in columns at which the text will be wrapped. (default 80)",
                Some('w'),
            )
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Wrap text passed into pipeline."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["convert", "ascii"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Wrap text at 10 columns",
                example: r#""now is the time for all good men to come to the aid of their country" | str wrap --width 10"#,
                result: Some(Value::test_string(
                    "now is the\ntime for\nall good\nmen to\ncome to\nthe aid of\ntheir\ncountry",
                )),
            },
            Example {
                description: "Wrap text at 10 columns using optimal-fit",
                example: r#""now is the time for all good men to come to the aid of their country" | str wrap --width 10 --optimal-fit"#,
                result: Some(Value::test_string(
                    "now is\nthe time\nfor all\ngood men\nto come\nto the aid\nof their\ncountry",
                )),
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
        // dedent	Removes common leading whitespace from each line.
        // fill	    Fill a line of text at width characters.
        // indent	Add prefix to each non-empty line.
        // refill	Refill a paragraph of wrapped text with a new width.
        // unfill	Unpack a paragraph of already-wrapped text.
        // wrap	Wrap a line of text at width characters.

        let optimal = call.has_flag("optimal-fit")?;
        let width = call.get_flag("width")?.unwrap_or(80usize);

        Ok(do_wrap(input, optimal, width, call.head))
    }
}

fn do_wrap(input: &Value, optimal: bool, width: usize, head: Span) -> Value {
    let options = Options::new(width).wrap_algorithm(if optimal {
        WrapAlgorithm::new_optimal_fit()
    } else {
        WrapAlgorithm::FirstFit
    });

    match input {
        // fill returns a string with the text wrapped at the specified width
        // wrap returns a list of strings with the text wrapped at the specified width
        Value::String { val, .. } => Value::string(fill(val, options), head),
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

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrWrap)
}
