use anyhow::Result;
use clap::{Parser, Subcommand};

mod audio;
mod commands;
mod crypto;
mod format;
mod progress;
mod stego;
mod verbosity;
mod wav;

pub use progress::Progress;
pub use verbosity::Verbosity;

#[derive(Parser)]
#[command(name = "zimhide")]
#[command(about = "Zim Steganography Toolkit - embed and extract encrypted content in WAV files")]
#[command(version)]
pub struct Cli {
    /// Suppress all output except errors and requested content
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Show detailed output
    #[arg(short, long, global = true, conflicts_with = "quiet")]
    pub verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Embed text or audio into a WAV file
    Encode(commands::encode::EncodeArgs),

    /// Extract text content from a WAV file
    Decode(commands::decode::DecodeArgs),

    /// Extract and play embedded audio from a WAV file
    Play(commands::play::PlayArgs),

    /// Generate a keypair for encryption and signing
    Keygen(commands::keygen::KeygenArgs),

    /// Inspect embedded content metadata without decrypting
    Inspect(commands::inspect::InspectArgs),

    /// Generate shell completions
    Completions(commands::completions::CompletionsArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbosity = Verbosity::from_flags(cli.quiet, cli.verbose);

    match cli.command {
        Commands::Encode(args) => commands::encode::run(args, verbosity),
        Commands::Decode(args) => commands::decode::run(args, verbosity),
        Commands::Play(args) => commands::play::run(args, verbosity),
        Commands::Keygen(args) => commands::keygen::run(args, verbosity),
        Commands::Inspect(args) => commands::inspect::run(args, verbosity),
        Commands::Completions(args) => {
            commands::completions::run(args);
            Ok(())
        }
    }
}
