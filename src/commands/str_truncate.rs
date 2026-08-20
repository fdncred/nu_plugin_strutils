use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, ShellError, Signature, Span, SyntaxShape, Type, Value,
};
use unicode_truncate::{Alignment, UnicodeTruncateStr};

use crate::StrutilsPlugin;

pub struct StrTruncate;

impl SimplePluginCommand for StrTruncate {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str truncate"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .named(
                "width",
                SyntaxShape::Int,
                "Maximum display width in columns.",
                Some('w'),
            )
            .named(
                "ellipsis",
                SyntaxShape::String,
                "String appended when truncated (default '…').",
                Some('e'),
            )
            .switch(
                "pad",
                "Pad to --width when the input is shorter.",
                Some('p'),
            )
            .named(
                "align",
                SyntaxShape::String,
                "Padding alignment: left, right, or center (default left).",
                Some('a'),
            )
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Truncate (and optionally pad) a string by terminal display width."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["ellipsis", "pad", "width", "clip"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Truncate to 10 columns with an ellipsis",
                example: r#""now is the time" | str truncate --width 10"#,
                result: Some(Value::test_string("now is th…")),
            },
            Example {
                description: "Pad a short string to 6 columns",
                example: r#""hi" | str truncate --width 6 --pad"#,
                result: Some(Value::test_string("hi    ")),
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
        let width: usize = call.get_flag("width")?.ok_or_else(|| {
            LabeledError::new("missing required flag").with_label("--width is required", call.head)
        })?;
        let ellipsis: String = call.get_flag("ellipsis")?.unwrap_or_else(|| "…".into());
        let pad = call.has_flag("pad")?;
        let align = match call
            .get_flag::<String>("align")?
            .as_deref()
            .unwrap_or("left")
        {
            "left" => Alignment::Left,
            "right" => Alignment::Right,
            "center" => Alignment::Center,
            other => {
                return Err(LabeledError::new("invalid align")
                    .with_label(format!("{other} is not left, right, or center"), call.head));
            }
        };

        Ok(do_truncate(input, width, &ellipsis, pad, align, call.head))
    }
}

fn do_truncate(
    input: &Value,
    width: usize,
    ellipsis: &str,
    pad: bool,
    align: Alignment,
    head: Span,
) -> Value {
    match input {
        Value::String { val, .. } => {
            Value::string(truncate_to_width(val, width, ellipsis, pad, align), head)
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

fn display_width(s: &str) -> usize {
    s.unicode_truncate(usize::MAX).1
}

fn truncate_to_width(s: &str, width: usize, ellipsis: &str, pad: bool, align: Alignment) -> String {
    if width == 0 {
        return String::new();
    }

    let s_width = display_width(s);
    if s_width <= width {
        if pad {
            return s.unicode_pad(width, align, false).into_owned();
        }
        return s.to_string();
    }

    let e_width = display_width(ellipsis);
    if e_width >= width {
        return ellipsis.unicode_truncate(width).0.to_string();
    }

    let inner = width.saturating_sub(e_width);
    let truncated = s.unicode_truncate(inner).0;
    let mut out = String::with_capacity(truncated.len().saturating_add(ellipsis.len()));
    out.push_str(truncated);
    out.push_str(ellipsis);
    if pad {
        out.unicode_pad(width, align, false).into_owned()
    } else {
        out
    }
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrTruncate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_ascii_with_ellipsis() {
        assert_eq!(
            truncate_to_width("now is the time", 10, "…", false, Alignment::Left),
            "now is th…"
        );
    }

    #[test]
    fn pads_short_string() {
        assert_eq!(
            truncate_to_width("hi", 6, "…", true, Alignment::Left),
            "hi    "
        );
    }

    #[test]
    fn wide_chars_do_not_split_graphemes() {
        let out = truncate_to_width("你好吗", 3, "…", false, Alignment::Left);
        assert!(display_width(&out) <= 3);
    }

    #[test]
    fn shorter_than_width_is_unchanged_without_pad() {
        assert_eq!(
            truncate_to_width("hi", 10, "…", false, Alignment::Left),
            "hi"
        );
    }

    #[test]
    fn width_zero_is_empty() {
        assert_eq!(
            truncate_to_width("hello", 0, "…", false, Alignment::Left),
            ""
        );
    }

    #[test]
    fn custom_ellipsis() {
        assert_eq!(
            super::super::test_support::eval_str(
                r#""now is the time" | str truncate --width 10 --ellipsis "...""#
            ),
            "now is ..."
        );
    }

    #[test]
    fn pad_right() {
        assert_eq!(
            super::super::test_support::eval_str(
                r#""hi" | str truncate --width 6 --pad --align right"#
            ),
            "    hi"
        );
    }

    #[test]
    fn pad_center() {
        let out = super::super::test_support::eval_str(
            r#""hi" | str truncate --width 6 --pad --align center"#,
        );
        assert_eq!(out.len(), 6);
        assert!(out.contains("hi"));
    }

    #[test]
    fn missing_width_errors() {
        super::super::test_support::assert_error_contains(
            r#""hi" | str truncate"#,
            "missing required flag",
        );
    }

    #[test]
    fn invalid_align_errors() {
        super::super::test_support::assert_error_contains(
            r#""hi" | str truncate --width 6 --align sideways"#,
            "invalid align",
        );
    }

    #[test]
    fn rejects_non_string_input() {
        super::super::test_support::assert_rejects(r#"42 | str truncate --width 4"#);
    }
}
