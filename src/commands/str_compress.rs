use crate::StrutilsPlugin;
use flate2::write::{DeflateEncoder, ZlibEncoder};
use flate2::Compression;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    Category, ErrSpan, Example, IntoSpanned, LabeledError, ShellError, Signature, Span, Spanned,
    SyntaxShape, Type, Value,
};
use std::io::Write;

const BUFFER_SIZE: usize = 65536;
const DEFAULT_QUALITY: u32 = 3;
const DEFAULT_WINDOW_SIZE: u32 = 20;

pub struct StrCompress;

impl SimplePluginCommand for StrCompress {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str compress"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Any, Type::Binary)
            .switch("brotli", "Use brotli compression", Some('b'))
            .switch("flate", "Use flate compression", Some('f'))
            .switch("zlib", "Use zlib compression", Some('z'))
            .named(
                "quality",
                SyntaxShape::Int,
                "Quality between 0 and 11. 11 is smallest but takes longest to encode (default 3)",
                Some('q'),
            )
            .named(
                "window-size",
                SyntaxShape::Int,
                "Log of how big the ring buffer should be for copying prior data. Window size for brotli compression (default 20)",
                Some('w'),
            )
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Compress string to compressed format."
    }

    fn extra_description(&self) -> &str {
        "All nushell value types are converted to strings first before compressing."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["convert", "ascii", "decompress", "brotli", "flate", "zlib"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Compress a json string using brotli",
                example: "ls | to json | str compress --brotli",
                result: None,
            },
            Example {
                description: "Compress a json string using flate",
                example: "ls | to json | str compress --flate",
                result: None,
            },
            Example {
                description: "Compress a json string using zlib",
                example: "ls | to json | str compress --zlib",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        _plugin: &StrutilsPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        fn to_u32(n: Spanned<i64>) -> Result<Spanned<u32>, ShellError> {
            u32::try_from(n.item)
                .map_err(|err| ShellError::CantConvert {
                    to_type: "u32".into(),
                    from_type: "int".into(),
                    span: n.span,
                    help: Some(err.to_string()),
                })
                .map(|o| o.into_spanned(n.span))
        }
        let config = engine.get_config()?;
        let quality = call.get_flag("quality")?.map(to_u32).transpose()?;
        let window_size = call.get_flag("window-size")?.map(to_u32).transpose()?;

        let use_brotli = call.has_flag("brotli")?;
        let use_flate = call.has_flag("flate")?;
        let use_zlib = call.has_flag("zlib")?;

        match (use_brotli, use_flate, use_zlib) {
            (true, false, false) => do_brotli(input, quality, window_size, config, call.head),
            (false, true, false) => do_flate(input, quality, config, call.head),
            (false, false, true) => do_zlib(input, quality, config, call.head),
            (false, false, false) => do_brotli(input, quality, window_size, config, call.head), // default to brotli
            _ => Err(LabeledError::new(
                "Only one compression method can be used at a time",
            )),
        }
    }
}

fn do_flate(
    input: &Value,
    quality: Option<Spanned<u32>>,
    config: std::sync::Arc<nu_protocol::Config>,
    head: Span,
) -> Result<Value, LabeledError> {
    let value_span = input.span();
    let value = input.to_expanded_string("", &config);
    let mut out_buf = vec![];
    let compression_level = quality.map(|q| q.item).unwrap_or(DEFAULT_QUALITY);
    let mut writer = DeflateEncoder::new(&mut out_buf, Compression::new(compression_level.into()));

    write_value(&mut writer, value, value_span)?;
    let _ = writer
        .finish()
        .err_span(head)
        .map_err(|err| ShellError::GenericError {
            error: err.item.to_string(),
            msg: "Error writing to flate compressor".to_string(),
            span: Some(value_span),
            help: None,
            inner: vec![],
        })?;

    Ok(Value::binary(out_buf, value_span))
}

fn do_zlib(
    input: &Value,
    quality: Option<Spanned<u32>>,
    config: std::sync::Arc<nu_protocol::Config>,
    head: Span,
) -> Result<Value, LabeledError> {
    let value_span = input.span();
    let value = input.to_expanded_string("", &config);
    let mut out_buf = vec![];
    let compression_level = quality.map(|q| q.item).unwrap_or(DEFAULT_QUALITY);
    let mut writer = ZlibEncoder::new(&mut out_buf, Compression::new(compression_level.into()));

    write_value(&mut writer, value, value_span)?;
    let _ = writer
        .finish()
        .err_span(head)
        .map_err(|err| ShellError::GenericError {
            error: err.item.to_string(),
            msg: "Error writing to zlib compressor".to_string(),
            span: Some(value_span),
            help: None,
            inner: vec![],
        })?;

    Ok(Value::binary(out_buf, value_span))
}

fn do_brotli(
    input: &Value,
    quality: Option<Spanned<u32>>,
    window_size: Option<Spanned<u32>>,
    config: std::sync::Arc<nu_protocol::Config>,
    head: Span,
) -> Result<Value, LabeledError> {
    let value_span = input.span();
    let value = input.to_expanded_string("", &config);
    let mut out_buf = vec![];
    let mut writer = brotli::CompressorWriter::new(
        &mut out_buf,
        BUFFER_SIZE,
        quality.map(|q| q.item).unwrap_or(DEFAULT_QUALITY),
        window_size.map(|w| w.item).unwrap_or(DEFAULT_WINDOW_SIZE),
    );

    write_value(&mut writer, value, value_span)?;
    let _ = writer
        .flush()
        .err_span(head)
        .map_err(|err| ShellError::GenericError {
            error: err.item.to_string(),
            msg: "Error writing to brotli compressor".to_string(),
            span: Some(value_span),
            help: None,
            inner: vec![],
        });
    drop(writer);

    Ok(Value::binary(out_buf, value_span))
}

fn write_value(out: &mut impl std::io::Write, value: String, span: Span) -> Result<(), ShellError> {
    out.write_all(value.as_bytes())
        .map_err(|err| ShellError::GenericError {
            error: err.to_string(),
            msg: "Error writing to brotli compressor".to_string(),
            span: Some(span),
            help: None,
            inner: vec![],
        })?;

    Ok(())
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    // This will automatically run the examples specified in your command and compare their actual
    // output against what was specified in the example.
    //
    // We recommend you add this test to any other commands you create, or remove it if the examples
    // can't be tested this way.

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrCompress)
}
