use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, ShellError, Signature, Span, SyntaxShape, Type, Value,
};
use textwrap::{Options, WrapAlgorithm, fill};

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

    fn examples(&self) -> Vec<Example<'_>> {
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

#[cfg(test)]
mod tests {
    use super::super::test_support::{assert_rejects, eval_str};

    #[test]
    fn short_string_is_unchanged() {
        assert_eq!(eval_str(r#""hello" | str wrap --width 80"#), "hello");
    }

    #[test]
    fn empty_string() {
        assert_eq!(eval_str(r#""" | str wrap --width 10"#), "");
    }

    #[test]
    fn default_width_wraps_long_line() {
        let out = eval_str(
            r#""aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" | str wrap"#,
        );
        assert!(out.contains('\n'), "{out}");
    }

    #[test]
    fn rejects_non_string_input() {
        assert_rejects("42 | str wrap");
    }
}
