use clap::{Command, CommandFactory};
use clap_complete::{Generator, Shell, generate};
use std::io;

use crate::Cli;

#[derive(clap::Args)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: Shell,
}

pub fn run(args: CompletionsArgs) {
    let mut cmd = Cli::command();
    print_completions(args.shell, &mut cmd);
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}
