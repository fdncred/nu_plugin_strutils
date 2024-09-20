use crate::StrutilsPlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, Type, Value};
use std::io::{Cursor, Read};

const BUFFER_SIZE: usize = 65536;

pub struct StrDecompress;

impl SimplePluginCommand for StrDecompress {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str decompress"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::String)
            .switch("brotli", "Use brotli decompression", Some('b'))
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Convert brotli-compressed data into a string."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["convert", "ascii"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Decompress a json string",
            example: "ls | to json | str compress --brotli | str decompress --brotli",
            result: None,
        }]
    }

    fn run(
        &self,
        _plugin: &StrutilsPlugin,
        _engine: &EngineInterface,
        _call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match input {
            Value::Binary { val: bytes, .. } => {
                let mut reader = brotli::Decompressor::new(Cursor::new(bytes), BUFFER_SIZE);

                let mut decompressed = Vec::new();
                reader.read_to_end(&mut decompressed).map_err(|err| {
                    LabeledError::new("Decompression error")
                        .with_label(err.to_string(), input.span())
                })?;

                Ok(Value::string(
                    String::from_utf8_lossy(decompressed.as_slice()).to_string(),
                    input.span(),
                ))
            }
            _ => Err(LabeledError::new("Type mismatch")
                .with_label(
                    format!("expected binary, found {}", input.get_type()),
                    input.span(),
                )
                .with_help("Only binary nushell values are supported.")),
        }
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

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrDecompress)
}
