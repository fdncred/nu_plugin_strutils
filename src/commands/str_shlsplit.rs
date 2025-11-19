use crate::StrutilsPlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, ShellError, Signature, Span, Type, Value};
use shell_words::split;

pub struct StrShlSplit;

impl SimplePluginCommand for StrShlSplit {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str shl-split"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![
                (Type::String, Type::String),
                (Type::String, Type::list(Type::String)),
            ])
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Parse an argument string with Unix rules similar to Python's shlex.split and GLib's g_shell_parse_argv."
    }

    fn extra_description(&self) -> &str {
        r"Splits command line into separate arguments, in much the same way Unix shell would, but without many of expansion the shell would perform.

The split functionality is compatible with behaviour of Unix shell, but with word expansions limited to quote removal, and without special token recognition rules for operators.

The result is exactly the same as one obtained from Unix shell as long as those unsupported features are not present in input: no operators, no variable assignments, no tilde expansion, no parameter expansion, no command substitution, no arithmetic expansion, no pathname expansion.

In case those unsupported shell features are present, the syntax that introduce them is interpreted literally."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["args", "split", "posix", "argv"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Parse the arguments and ignore the comments",
                example: r#"" a # very long comment \n b # another comment" | str shl-split"#,
                result: Some(Value::test_list(vec![
                    Value::test_string("a"),
                    Value::test_string("b"),
                ])),
            },
            Example {
                description: "Parse the arguments",
                example: r#""-c a.c -o a.out" | str shl-split"#,
                result: Some(Value::test_list(vec![
                    Value::test_string("-c"),
                    Value::test_string("a.c"),
                    Value::test_string("-o"),
                    Value::test_string("a.out"),
                ])),
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
        Ok(do_split(input, call.head))
    }
}

fn do_split(input: &Value, head: Span) -> Value {
    match input {
        Value::String { val, .. } => {
            let args = match split(val) {
                Ok(args) => args.into_iter().map(|s| Value::string(s, head)).collect(),
                Err(err) => {
                    return Value::error(
                        ShellError::GenericError {
                            msg: "Failed to parse string".into(),
                            error: err.to_string(),
                            span: Some(head),
                            help: None,
                            inner: Vec::new(),
                        },
                        head,
                    );
                }
            };
            Value::list(args, head)
        }
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

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrShlSplit)
}
