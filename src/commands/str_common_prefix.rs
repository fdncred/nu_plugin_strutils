use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, ShellError, Signature, Span, SyntaxShape, Type, Value,
};

use crate::StrutilsPlugin;

pub struct StrCommonPrefix;

impl SimplePluginCommand for StrCommonPrefix {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str common-prefix"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![
                (Type::String, Type::String),
                (Type::List(Box::new(Type::String)), Type::String),
            ])
            .optional(
                "other",
                SyntaxShape::String,
                "Second string when the input is a single string.",
            )
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Return the longest common prefix of two strings or a list of strings."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["shared", "path", "prefix", "lcp"]
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Common prefix of a list of paths",
                example: r#"["src/commands/mod.rs" "src/commands/str_slug.rs"] | str common-prefix"#,
                result: Some(Value::test_string("src/commands/")),
            },
            Example {
                description: "Common prefix of two strings",
                example: r#""foobar" | str common-prefix "foobaz""#,
                result: Some(Value::test_string("fooba")),
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
        let other: Option<String> = call.opt(0)?;
        do_common_prefix(input, other, call.head)
    }
}

fn do_common_prefix(
    input: &Value,
    other: Option<String>,
    head: Span,
) -> Result<Value, LabeledError> {
    match input {
        Value::String { val, .. } => {
            let other = other.ok_or_else(|| {
                LabeledError::new("missing required argument")
                    .with_label("provide a second string, or pipe a list of strings", head)
            })?;
            Ok(Value::string(shared_prefix(val, &other), head))
        }
        Value::List { vals, .. } => {
            if other.is_some() {
                return Err(LabeledError::new("unexpected argument")
                    .with_label("do not pass a second string when the input is a list", head));
            }
            let mut strings = Vec::with_capacity(vals.len());
            for v in vals {
                match v {
                    Value::String { val, .. } => strings.push(val.as_str()),
                    Value::Error { .. } => return Ok(v.clone()),
                    _ => {
                        return Ok(Value::error(
                            ShellError::OnlySupportsThisInputType {
                                exp_input_type: "string".into(),
                                wrong_type: v.get_type().to_string(),
                                dst_span: head,
                                src_span: v.span(),
                            },
                            head,
                        ));
                    }
                }
            }
            Ok(Value::string(common_prefix(&strings), head))
        }
        Value::Error { .. } => Ok(input.clone()),
        _ => Ok(Value::error(
            ShellError::OnlySupportsThisInputType {
                exp_input_type: "string or list<string>".into(),
                wrong_type: input.get_type().to_string(),
                dst_span: head,
                src_span: input.span(),
            },
            head,
        )),
    }
}

fn shared_prefix<'a>(a: &'a str, b: &str) -> &'a str {
    let n = a.bytes().zip(b.bytes()).take_while(|(x, y)| x == y).count();
    let n = (0..=n).rev().find(|&i| a.is_char_boundary(i)).unwrap_or(0);
    &a[..n]
}

fn common_prefix(strings: &[&str]) -> String {
    let Some(first) = strings.first() else {
        return String::new();
    };
    let mut prefix = *first;
    for s in &strings[1..] {
        prefix = shared_prefix(prefix, s);
        if prefix.is_empty() {
            break;
        }
    }
    prefix.to_string()
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrCommonPrefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_strings() {
        assert_eq!(shared_prefix("foobar", "foobaz"), "fooba");
    }

    #[test]
    fn list_of_paths() {
        assert_eq!(
            common_prefix(&["src/commands/mod.rs", "src/commands/str_slug.rs"]),
            "src/commands/"
        );
    }

    #[test]
    fn empty_list() {
        assert_eq!(common_prefix(&[]), "");
    }

    #[test]
    fn snaps_to_char_boundary() {
        assert_eq!(shared_prefix("é", "e"), "");
    }

    #[test]
    fn no_common_prefix_is_empty() {
        assert_eq!(common_prefix(&["abc", "xyz"]), "");
    }

    #[test]
    fn single_item_list_is_itself() {
        assert_eq!(
            super::super::test_support::eval_str(r#"["only"] | str common-prefix"#),
            "only"
        );
    }

    #[test]
    fn empty_list_is_empty_string() {
        assert_eq!(
            super::super::test_support::eval_str("[] | str common-prefix"),
            ""
        );
    }

    #[test]
    fn string_without_other_errors() {
        super::super::test_support::assert_error_contains(
            r#""foobar" | str common-prefix"#,
            "missing required argument",
        );
    }

    #[test]
    fn list_with_extra_arg_errors() {
        super::super::test_support::assert_error_contains(
            r#"["a" "b"] | str common-prefix "c""#,
            "unexpected argument",
        );
    }

    #[test]
    fn rejects_non_string_list_items() {
        super::super::test_support::assert_rejects(r#"["a" 1] | str common-prefix"#);
    }
}
