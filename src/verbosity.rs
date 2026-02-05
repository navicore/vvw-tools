//! Verbosity control for CLI output.

/// Verbosity level for command output.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Verbosity {
    /// Suppress all non-essential output (errors and requested content only)
    Quiet,
    /// Normal output (status messages)
    #[default]
    Normal,
    /// Verbose output (detailed information)
    Verbose,
}

impl Verbosity {
    /// Create from quiet/verbose flags.
    pub fn from_flags(quiet: bool, verbose: bool) -> Self {
        if quiet {
            Self::Quiet
        } else if verbose {
            Self::Verbose
        } else {
            Self::Normal
        }
    }

    /// Returns true if status messages should be shown.
    pub fn show_status(self) -> bool {
        !matches!(self, Self::Quiet)
    }

    /// Returns true if verbose/debug messages should be shown.
    pub fn show_verbose(self) -> bool {
        matches!(self, Self::Verbose)
    }
}

/// Print a status message to stderr (suppressed in quiet mode).
#[macro_export]
macro_rules! status {
    ($verbosity:expr, $($arg:tt)*) => {
        if $verbosity.show_status() {
            eprintln!($($arg)*);
        }
    };
}

/// Print a verbose message to stderr (only in verbose mode).
#[macro_export]
macro_rules! verbose {
    ($verbosity:expr, $($arg:tt)*) => {
        if $verbosity.show_verbose() {
            eprintln!($($arg)*);
        }
    };
}
