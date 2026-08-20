use crate::StrutilsPlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, Signature, Span, Spanned, SyntaxShape, Value, record,
};
use std::vec;
use textdistance::{nstr, str};

pub struct StrSimilarity;

impl SimplePluginCommand for StrSimilarity {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str similarity"
    }

    fn description(&self) -> &str {
        "Compare strings to find similarity by algorithm"
    }
    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .optional("string", SyntaxShape::String, "String to compare with")
            .switch(
                "normalize",
                "Normalize the results between 0 and 1",
                Some('n'),
            )
            .switch("list", "List all available algorithms", Some('l'))
            .named(
                "algorithm",
                SyntaxShape::String,
                "Name of the algorithm to compute",
                Some('a'),
            )
            .switch("all", "Run all algorithms", Some('A'))
            .category(Category::Experimental)
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Compare two strings for similarity",
                example: "'nutshell' | str similarity 'nushell'",
                result: Some(Value::test_int(1)),
            },
            Example {
                description: "Compare two strings for similarity and normalize the output value",
                example: "'nutshell' | str similarity -n 'nushell'",
                result: Some(Value::test_float(0.125)),
            },
            Example {
                description: "Compare two strings for similarity using a specific algorithm",
                example: "'nutshell' | str similarity 'nushell' -a levenshtein",
                result: Some(Value::test_int(1)),
            },
            Example {
                description: "List all the included similarity algorithms",
                example: "str similarity --list",
                result: None,
            },
            Example {
                description: "Compare two strings for similarity with all algorithms",
                example: "'nutshell' | str similarity 'nushell' -A",
                result: None,
            },
            Example {
                description: "Compare two strings for similarity with all algorithms and normalize the output value",
                example: "'nutshell' | str similarity 'nushell' -A -n",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        _config: &StrutilsPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let list = call.has_flag("list")?;
        if list {
            return Ok(list_algorithms(call.head));
        }

        let compare_to_str: Spanned<String> = call.opt(0)?.ok_or_else(|| {
            LabeledError::new("missing required argument")
                .with_label("expected a string to compare with", call.head)
        })?;
        let normalize = call.has_flag("normalize")?;
        let sim = call
            .get_flag::<String>("algorithm")?
            .unwrap_or_else(|| "levenshtein".into());
        if compute(&sim, "", "", false).is_none() {
            return Err(LabeledError::new("unknown algorithm")
                .with_label(format!("{sim} is not a known algorithm"), call.head));
        }
        let all = call.has_flag("all")?;
        let input_span = input.span();

        let ret_val = match input {
            Value::String { val: input_val, .. } => {
                if all {
                    compute_all(&compare_to_str.item, input_val, normalize, input_span)?
                } else {
                    compare_strings(&sim, compare_to_str, normalize, input_val, input_span)?
                }
            }
            v => {
                return Err(LabeledError::new(format!(
                    "requires some input, got {}",
                    v.get_type()
                ))
                .with_label("Expected something from pipeline", call.head));
            }
        };

        Ok(ret_val)
    }
}

fn compute_all(s1: &str, s2: &str, norm: bool, span: Span) -> Result<Value, LabeledError> {
    let algos = vec![
        "bag",
        "cosine",
        "damerau_levenshtein",
        "entropy_ncd",
        "hamming",
        "jaccard",
        "jaro",
        "jaro_winkler",
        "levenshtein",
        "longest_common_subsequence",
        "longest_common_substring",
        "length",
        "lig3",
        "mlipns",
        "overlap",
        "prefix",
        "ratcliff_obershelp",
        "roberts",
        "sift4_common",
        "sift4_simple",
        "smith_waterman",
        "sorensen_dice",
        "suffix",
        "tversky",
        "yujian_bo",
    ];
    let mut rows = vec![];
    for algo in algos {
        let sim = Value::string(algo.to_string(), span);
        let val_comp = compute(algo, s1, s2, norm).unwrap_or(0.0);
        let val = if val_comp.fract() == 0.0 {
            Value::int(val_comp as i64, span)
        } else {
            Value::float(val_comp, span)
        };
        rows.push(Value::record(
            record! { "algorithm" => sim, "distance" => val },
            span,
        ));
    }

    Ok(Value::list(rows, span))
}

#[rustfmt::skip]
fn compute(a: &str, s1: &str, s2: &str, norm: bool) -> Option<f64> {
    let sim = a.to_lowercase();
    Some(match sim.as_str() {
        "bag" => if norm { nstr::bag(s1, s2) } else {str::bag(s1, s2) as f64},
        "cos" | "cosine" => if norm { nstr::cosine(s1, s2) } else {str::cosine(s1, s2)},
        "dlev" | "damerau_levenshtein" => if norm { nstr::damerau_levenshtein(s1, s2) } else {str::damerau_levenshtein(s1, s2) as f64},
        "entncd" | "entropy_ncd" => if norm { nstr::entropy_ncd(s1, s2) } else {str::entropy_ncd(s1, s2)},
        "ham" | "hamming" => if norm { nstr::hamming(s1, s2) } else {str::hamming(s1, s2) as f64},
        "jac" | "jaccard" => if norm { nstr::jaccard(s1, s2) } else {str::jaccard(s1, s2)},
        "jar" | "jaro" => if norm { nstr::jaro(s1, s2) } else {str::jaro(s1, s2)},
        "jarw" | "jaro_winkler" => if norm { nstr::jaro_winkler(s1, s2) } else {str::jaro_winkler(s1, s2)},
        "lev" | "levenshtein" => if norm { nstr::levenshtein(s1, s2) } else {str::levenshtein(s1, s2) as f64},
        "lcsubseq" | "longest_common_subsequence" => if norm { nstr::lcsseq(s1, s2) } else {str::lcsseq(s1, s2) as f64},
        "lcsubstr" | "longest_common_substring" => if norm { nstr::lcsstr(s1, s2) } else {str::lcsstr(s1, s2) as f64},
        "len" | "length" => if norm { nstr::length(s1, s2) } else {str::length(s1, s2) as f64},
        "lig" | "lig3" => if norm { nstr::lig3(s1, s2) } else {str::lig3(s1, s2)},
        "mli" | "mlipns" => if norm { nstr::mlipns(s1, s2) } else {str::mlipns(s1, s2) as f64},
        "olap" | "overlap" => if norm { nstr::overlap(s1, s2) } else {str::overlap(s1, s2)},
        "pre" | "prefix" => if norm { nstr::prefix(s1, s2) } else {str::prefix(s1, s2) as f64},
        "rat" | "ratcliff_obershelp" => if norm { nstr::ratcliff_obershelp(s1, s2) } else {str::ratcliff_obershelp(s1, s2)},
        "rob" | "roberts" => if norm { nstr::roberts(s1, s2) } else {str::roberts(s1, s2)},
        "scom" | "sift4_common" => if norm { nstr::sift4_common(s1, s2) } else {str::sift4_common(s1, s2) as f64},
        "ssim" | "sift4_simple" => if norm { nstr::sift4_simple(s1, s2) } else {str::sift4_simple(s1, s2) as f64},
        "smithw" | "smith_waterman" => if norm { nstr::smith_waterman(s1, s2) } else {str::smith_waterman(s1, s2) as f64},
        "soredice" | "sorensen_dice" => if norm { nstr::sorensen_dice(s1, s2) } else {str::sorensen_dice(s1, s2)},
        "suf" | "suffix" => if norm { nstr::suffix(s1, s2) } else {str::suffix(s1, s2) as f64},
        "tv" | "tversky" => if norm { nstr::tversky(s1, s2) } else {str::tversky(s1, s2)},
        "ybo" | "yujian_bo" => if norm { nstr::yujian_bo(s1, s2) } else {str::yujian_bo(s1, s2)},
        _ => return None,
    })
}

#[rustfmt::skip]
fn list_algorithms(span: Span) -> Value {
    let row = |algorithm: &str, short: &str| {
        Value::record(
            record! {
                "algorithm" => Value::string(algorithm, span),
                "short" => Value::string(short, span),
            },
            span,
        )
    };
    let rows = vec![
        row("bag", "bag"),
        row("cosine", "cos"),
        row("damerau_levenshtein", "dlev"),
        row("entropy_ncd", "entncd"),
        row("hamming", "ham"),
        row("jaccard", "jac"),
        row("jaro", "jar"),
        row("jaro_winkler", "jarw"),
        row("levenshtein", "lev"),
        row("longest_common_subsequence", "lcsubseq"),
        row("longest_common_substring", "lcsubstr"),
        row("length", "len"),
        row("lig3", "lig"),
        row("mlipns", "mli"),
        row("overlap", "olap"),
        row("prefix", "pre"),
        row("ratcliff_obershelp", "rat"),
        row("roberts", "rob"),
        row("sift4_common", "scom"),
        row("sift4_simple", "ssim"),
        row("smith_waterman", "smithw"),
        row("sorensen_dice", "soredice"),
        row("suffix", "suf"),
        row("tversky", "tv"),
        row("yujian_bo", "ybo"),
    ];

    Value::list(rows, span)
}

fn compare_strings(
    sim_algo: &str,
    compare_to_str: Spanned<String>,
    normalize: bool,
    input_val: &str,
    input_span: Span,
) -> Result<Value, LabeledError> {
    let compare_from = input_val;
    let compare_to = compare_to_str.item;

    let a_val = compute(sim_algo, compare_from, &compare_to, normalize).ok_or_else(|| {
        LabeledError::new("unknown algorithm")
            .with_label(format!("{sim_algo} is not a known algorithm"), input_span)
    })?;

    if a_val.fract() == 0.0 {
        Ok(Value::int(a_val as i64, input_span))
    } else {
        Ok(Value::float(a_val, input_span))
    }
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrSimilarity)
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{assert_error_contains, eval, eval_int, eval_list};

    #[test]
    fn alias_matches_full_name() {
        let full = eval_int("'nutshell' | str similarity 'nushell' --algorithm levenshtein");
        let alias = eval_int("'nutshell' | str similarity 'nushell' --algorithm lev");
        assert_eq!(full, alias);
        assert_eq!(full, 1);
    }

    #[test]
    fn list_returns_all_algorithms() {
        let rows = eval_list("str similarity --list");
        assert_eq!(rows.len(), 25);
        let first = rows[0].as_record().expect("record");
        assert!(first.contains("algorithm"));
        assert!(first.contains("short"));
    }

    #[test]
    fn all_returns_a_row_per_algorithm() {
        let rows = eval_list("'nutshell' | str similarity 'nushell' --all");
        assert_eq!(rows.len(), 25);
    }

    #[test]
    fn all_normalized_distances_are_between_zero_and_one() {
        let rows = eval_list("'nutshell' | str similarity 'nushell' --all --normalize");
        for row in rows {
            let rec = row.as_record().expect("record");
            let distance = rec.get("distance").expect("distance").clone();
            let n = match distance {
                nu_protocol::Value::Int { val, .. } => val as f64,
                nu_protocol::Value::Float { val, .. } => val,
                other => panic!("unexpected distance {other:?}"),
            };
            assert!((0.0..=1.0).contains(&n), "{n}");
        }
    }

    #[test]
    fn unknown_algorithm_errors() {
        assert_error_contains(
            "'nutshell' | str similarity 'nushell' --algorithm not-a-real-algo",
            "unknown algorithm",
        );
    }

    #[test]
    fn rejects_non_string_input() {
        assert_error_contains("42 | str similarity 'x'", "requires some input");
    }

    #[test]
    fn empty_strings_do_not_panic() {
        let _ = eval("'' | str similarity ''");
    }
}
