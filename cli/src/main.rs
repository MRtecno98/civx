use clap::{Parser, builder::{Styles, styling::{AnsiColor, Effects}}};

mod args;

pub(crate) fn clap_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
}

#[tokio::main]
async fn main() {
	let _cli = args::Args::parse();

}
