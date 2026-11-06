//! Progress reports and cancel for long commands (Collaborators, Discover).

pub use tokio_util::sync::CancellationToken;

use crate::dto::Progress;

/// Receives progress reports. The Tauri app sends them to the screen as events.
pub trait ProgressSink: Send + Sync {
    fn report(&self, progress: Progress);
}

/// A sink that ignores all reports.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoProgress;

impl ProgressSink for NoProgress {
    fn report(&self, _progress: Progress) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_progress_accepts_reports() {
        NoProgress.report(Progress {
            task: "t".into(),
            done: 1,
            total: 2,
        });
    }
}
