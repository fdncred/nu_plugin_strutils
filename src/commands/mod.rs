// Command modules should be added here
mod extract;
mod str_after;
mod str_align;
mod str_before;
mod str_between;
mod str_common_prefix;
mod str_compress;
mod str_decompress;
mod str_dedent;
mod str_deunicode;
mod str_increment;
mod str_indent;
mod str_shlquote;
mod str_shlsplit;
mod str_similarity;
mod str_slug;
mod str_truncate;
mod str_unescape;
mod str_wrap;

#[cfg(test)]
mod test_support;

// Command structs should be exported here
pub use str_after::StrAfter;
pub use str_align::StrAlign;
pub use str_before::StrBefore;
pub use str_between::StrBetween;
pub use str_common_prefix::StrCommonPrefix;
pub use str_compress::StrCompress;
pub use str_decompress::StrDecompress;
pub use str_dedent::StrDedent;
pub use str_deunicode::StrDeunicode;
pub use str_increment::StrIncrement;
pub use str_indent::StrIndent;
pub use str_shlquote::StrShlQuote;
pub use str_shlsplit::StrShlSplit;
pub use str_similarity::StrSimilarity;
pub use str_slug::StrSlug;
pub use str_truncate::StrTruncate;
pub use str_unescape::StrUnescape;
pub use str_wrap::StrWrap;
