use alacritty_terminal::{
    ansi::Processor, config::Config, event::VoidListener, term::test::TermSize, Term,
};

pub(crate) fn term() -> Term<VoidListener> {
    let config = Config::default();
    let term_size = TermSize::new(super::COLUMNS, super::LINES);

    Term::new(&config, &term_size, VoidListener)
}

pub(crate) fn processor() -> Processor {
    Processor::new()
}
