// Command modules should be added here
mod str_compress;
mod str_decompress;
mod str_deunicode;
mod str_similarity;
mod str_wrap;

// Command structs should be exported here
pub use str_compress::StrCompress;
pub use str_decompress::StrDecompress;
pub use str_deunicode::StrDeunicode;
pub use str_similarity::StrSimilarity;
pub use str_wrap::StrWrap;
