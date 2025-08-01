// Command modules should be added here
mod str_compress;
mod str_decompress;
mod str_dedent;
mod str_deunicode;
mod str_indent;
mod str_shlquote;
mod str_shlsplit;
mod str_similarity;
mod str_slug;
mod str_wrap;

// Command structs should be exported here
pub use str_compress::StrCompress;
pub use str_decompress::StrDecompress;
pub use str_dedent::StrDedent;
pub use str_deunicode::StrDeunicode;
pub use str_indent::StrIndent;
pub use str_shlquote::StrShlQuote;
pub use str_shlsplit::StrShlSplit;
pub use str_similarity::StrSimilarity;
pub use str_slug::StrSlug;
pub use str_wrap::StrWrap;
