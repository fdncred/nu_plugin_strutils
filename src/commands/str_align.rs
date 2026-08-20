use std::io::Write;

use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, ShellError, Signature, Span, SyntaxShape, Type, Value,
};
use tabwriter::TabWriter;

use crate::StrutilsPlugin;

pub struct StrAlign;

impl SimplePluginCommand for StrAlign {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str align"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .named(
                "separator",
                SyntaxShape::String,
                "Column separator in the input (default tab).",
                Some('s'),
            )
            .named(
                "padding",
                SyntaxShape::Int,
                "Spaces between columns (default 2).",
                Some('p'),
            )
            .named(
                "min-width",
                SyntaxShape::Int,
                "Minimum column width (default 2).",
                Some('m'),
            )
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Align delimited columns using elastic tabstops."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["column", "table", "tab", "pad"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Align tab-separated columns",
                example: r#""name\tcount\nalice\t12\nbob\t3" | str align"#,
                result: Some(Value::test_string("name   count\nalice  12\nbob    3")),
            },
            Example {
                description: "Align comma-separated columns",
                example: r#""a,bb,ccc\n1,2,3" | str align --separator ",""#,
                result: Some(Value::test_string("a   bb  ccc\n1   2   3")),
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
        let separator: String = call.get_flag("separator")?.unwrap_or_else(|| "\t".into());
        if separator.is_empty() {
            return Err(LabeledError::new("separator cannot be empty")
                .with_label("provide a non-empty column separator", call.head));
        }
        let padding: usize = call.get_flag("padding")?.unwrap_or(2);
        let min_width: usize = call.get_flag("min-width")?.unwrap_or(2);

        do_align(input, &separator, padding, min_width, call.head)
    }
}

fn do_align(
    input: &Value,
    separator: &str,
    padding: usize,
    min_width: usize,
    head: Span,
) -> Result<Value, LabeledError> {
    match input {
        Value::String { val, .. } => align_text(val, separator, padding, min_width)
            .map(|s| Value::string(s, head))
            .map_err(|e| LabeledError::new("failed to align text").with_label(e.to_string(), head)),
        Value::Error { .. } => Ok(input.clone()),
        _ => Ok(Value::error(
            ShellError::OnlySupportsThisInputType {
                exp_input_type: "string".into(),
                wrong_type: input.get_type().to_string(),
                dst_span: head,
                src_span: input.span(),
            },
            head,
        )),
    }
}

fn align_text(
    s: &str,
    separator: &str,
    padding: usize,
    min_width: usize,
) -> Result<String, std::io::Error> {
    let prepared = if separator == "\t" {
        s.to_string()
    } else {
        s.replace(separator, "\t")
    };

    let mut tw = TabWriter::new(Vec::new())
        .padding(padding)
        .minwidth(min_width);
    tw.write_all(prepared.as_bytes())?;
    tw.flush()?;
    let bytes = tw
        .into_inner()
        .map_err(|e| std::io::Error::other(e.error().to_string()))?;
    // TabWriter always ends with a newline; drop it when the input had none.
    let mut out = String::from_utf8(bytes).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "aligned output was not utf-8",
        )
    })?;
    if !s.ends_with('\n') {
        while out.ends_with('\n') {
            out.pop();
            if out.ends_with('\r') {
                out.pop();
            }
        }
    }
    Ok(out)
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrAlign)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligns_tabs() {
        let out = align_text("name\tcount\nalice\t12\nbob\t3", "\t", 2, 2).unwrap();
        assert_eq!(out, "name   count\nalice  12\nbob    3");
    }

    #[test]
    fn aligns_commas() {
        let out = align_text("a,bb,ccc\n1,2,3", ",", 2, 2).unwrap();
        assert_eq!(out, "a   bb  ccc\n1   2   3");
    }

    #[test]
    fn preserves_trailing_newline() {
        let out = align_text("a\tb\n", "\t", 2, 2).unwrap();
        assert!(out.ends_with('\n'), "{out:?}");
    }

    #[test]
    fn empty_input() {
        assert_eq!(align_text("", "\t", 2, 2).unwrap(), "");
    }

    #[test]
    fn custom_padding() {
        let out = super::super::test_support::eval_str(r#""a\tb\n1\t2" | str align --padding 4"#);
        assert!(out.contains("a"), "{out}");
        assert!(out.contains("b"), "{out}");
    }

    #[test]
    fn rejects_non_string_input() {
        super::super::test_support::assert_rejects("42 | str align");
    }

    #[test]
    fn empty_separator_errors() {
        super::super::test_support::assert_error_contains(
            r#""a,b" | str align --separator """#,
            "separator cannot be empty",
        );
    }
}
