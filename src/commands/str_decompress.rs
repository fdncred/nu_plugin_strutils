use crate::StrutilsPlugin;
use flate2::read::{DeflateDecoder, ZlibDecoder};
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
            .switch("flate", "Use flate decompression", Some('f'))
            .switch("zlib", "Use zlib decompression", Some('z'))
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Convert compressed data into a string."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["convert", "ascii", "compress", "brotli", "flate", "zlib"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Decompress a brotli-compressed string",
                example: "ls | to json | str compress --brotli | str decompress --brotli",
                result: None,
            },
            Example {
                description: "Decompress a flate-compressed string",
                example: "ls | to json | str compress --flate | str decompress --flate",
                result: None,
            },
            Example {
                description: "Decompress a zlib-compressed string",
                example: "ls | to json | str compress --zlib | str decompress --zlib",
                result: None,
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
        match input {
            Value::Binary { val: bytes, .. } => {
                let use_brotli = call.has_flag("brotli")?;
                let use_flate = call.has_flag("flate")?;
                let use_zlib = call.has_flag("zlib")?;

                let decompressed = match (use_brotli, use_flate, use_zlib) {
                    (true, false, false) => decompress_brotli(bytes, input.span())?,
                    (false, true, false) => decompress_flate(bytes, input.span())?,
                    (false, false, true) => decompress_zlib(bytes, input.span())?,
                    (false, false, false) => decompress_brotli(bytes, input.span())?,
                    _ => {
                        return Err(LabeledError::new(
                            "Only one decompression method can be used at a time",
                        )
                        .with_label("Multiple compression flags specified", call.head));
                    }
                };

                decode_utf8(decompressed, input.span())
            }
            Value::Error { .. } => Ok(input.clone()),
            _ => Err(LabeledError::new("Type mismatch")
                .with_label(
                    format!("expected binary, found {}", input.get_type()),
                    input.span(),
                )
                .with_help("Only binary nushell values are supported.")),
        }
    }
}

fn decode_utf8(bytes: Vec<u8>, span: nu_protocol::Span) -> Result<Value, LabeledError> {
    match String::from_utf8(bytes) {
        Ok(s) => Ok(Value::string(s, span)),
        Err(_) => Err(LabeledError::new("decompressed data is not valid utf-8")
            .with_label("output is not utf-8", span)),
    }
}

fn decompress_brotli(bytes: &[u8], span: nu_protocol::Span) -> Result<Vec<u8>, LabeledError> {
    let mut reader = brotli::Decompressor::new(Cursor::new(bytes), BUFFER_SIZE);
    let mut decompressed = Vec::new();
    reader.read_to_end(&mut decompressed).map_err(|err| {
        LabeledError::new("Brotli decompression error").with_label(err.to_string(), span)
    })?;
    Ok(decompressed)
}

fn decompress_flate(bytes: &[u8], span: nu_protocol::Span) -> Result<Vec<u8>, LabeledError> {
    let mut reader = DeflateDecoder::new(Cursor::new(bytes));
    let mut decompressed = Vec::new();
    reader.read_to_end(&mut decompressed).map_err(|err| {
        LabeledError::new("Flate decompression error").with_label(err.to_string(), span)
    })?;
    Ok(decompressed)
}

fn decompress_zlib(bytes: &[u8], span: nu_protocol::Span) -> Result<Vec<u8>, LabeledError> {
    let mut reader = ZlibDecoder::new(Cursor::new(bytes));
    let mut decompressed = Vec::new();
    reader.read_to_end(&mut decompressed).map_err(|err| {
        LabeledError::new("Zlib decompression error").with_label(err.to_string(), span)
    })?;
    Ok(decompressed)
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrDecompress)
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{assert_error_contains, assert_rejects};

    #[test]
    fn rejects_non_binary_input() {
        assert_rejects(r#""not binary" | str decompress --brotli"#);
    }

    #[test]
    fn multiple_methods_error() {
        assert_error_contains(
            r#""ABCDEFG" | str compress --brotli | str decompress --brotli --zlib"#,
            "Only one decompression method",
        );
    }

    #[test]
    fn garbage_bytes_error() {
        assert_error_contains(
            r#"0x[01 02 03 04] | str decompress --zlib"#,
            "decompression error",
        );
    }

    #[test]
    fn invalid_utf8_errors() {
        let err = super::decode_utf8(vec![0xff], nu_protocol::Span::test_data());
        assert!(err.is_err());
    }
}
