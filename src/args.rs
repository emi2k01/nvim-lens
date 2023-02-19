use bpaf::{construct, short, Parser};

pub(crate) struct Args {
    pub(crate) out_dir: String,
    pub(crate) config: String,
}

pub(crate) fn parse() -> Args {
    let out_dir = out_dir();
    let config = config();
    construct!(Args { out_dir, config }).to_options().run()
}

pub(crate) fn out_dir() -> impl Parser<String> {
    short('o')
        .long("out-dir")
        .env("PLUGINS_OUT_DIR")
        .argument("PLUGINS_OUT_DIR")
}

pub(crate) fn config() -> impl Parser<String> {
    short('c')
        .long("config")
        .env("CONFIG_PATH")
        .argument("CONFIG_PATH")
}
