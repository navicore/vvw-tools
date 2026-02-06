//! Progress indicators for long-running operations.
//!
//! Respects verbosity settings - no progress in quiet mode.
//! Only shows progress for operations with enough work to warrant it.

use crate::Verbosity;
use indicatif::{ProgressBar, ProgressStyle};

/// Minimum number of items before showing a progress bar.
/// Below this threshold, progress is not shown to avoid UI flicker for fast operations.
const MIN_ITEMS_FOR_PROGRESS: u64 = 25;

/// A progress bar that respects verbosity settings.
pub struct Progress {
    bar: Option<ProgressBar>,
}

impl Progress {
    /// Create a new progress bar with known total and initial message.
    ///
    /// Progress is only shown if:
    /// - Verbosity is not Quiet
    /// - Total items >= MIN_ITEMS_FOR_PROGRESS (to avoid flicker on fast operations)
    pub fn new(total: u64, message: &str, verbosity: Verbosity) -> Self {
        let bar = if verbosity.show_status() && total >= MIN_ITEMS_FOR_PROGRESS {
            let pb = ProgressBar::new(total);
            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.green} [{bar:30.cyan/blue}] {pos}/{len} {msg}",
                )
                .expect("hardcoded progress template should be valid")
                .progress_chars("#>-"),
            );
            pb.set_message(message.to_string());
            Some(pb)
        } else {
            None
        };
        Self { bar }
    }

    /// Increment the progress bar by one.
    pub fn inc(&self, delta: u64) {
        if let Some(ref bar) = self.bar {
            bar.inc(delta);
        }
    }

    /// Set the message shown on the progress bar.
    #[allow(dead_code)]
    pub fn set_message(&self, msg: impl Into<String>) {
        if let Some(ref bar) = self.bar {
            bar.set_message(msg.into());
        }
    }

    /// Finish and clear the progress bar.
    pub fn finish_and_clear(&self) {
        if let Some(ref bar) = self.bar {
            bar.finish_and_clear();
        }
    }
}
