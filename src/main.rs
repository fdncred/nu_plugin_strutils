use nu_plugin::{MsgPackSerializer, Plugin, PluginCommand, serve_plugin};

mod commands;
pub use commands::*;

pub struct StrutilsPlugin;

impl Plugin for StrutilsPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(StrDeunicode),
            Box::new(StrSimilarity),
            Box::new(StrCompress),
            Box::new(StrDecompress),
            Box::new(StrWrap),
            Box::new(StrDedent),
            Box::new(StrIndent),
            Box::new(StrSlug),
            Box::new(StrShlSplit),
            Box::new(StrShlQuote),
            Box::new(StrBefore),
            Box::new(StrAfter),
            Box::new(StrBetween),
            Box::new(StrIncrement),
            Box::new(StrTruncate),
            Box::new(StrAlign),
            Box::new(StrCommonPrefix),
            Box::new(StrUnescape),
        ]
    }
}

fn main() {
    serve_plugin(&StrutilsPlugin, MsgPackSerializer);
}
