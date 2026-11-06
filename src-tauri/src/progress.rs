//! Sends progress reports of long commands to the screen through a Tauri channel.

use rate_it_core::ProgressSink;
use rate_it_core::dto::Progress;
use tauri::ipc::Channel;

/// A [`ProgressSink`] that writes to a Tauri channel.
pub(crate) struct ChannelProgress(pub Channel<Progress>);

impl ProgressSink for ChannelProgress {
    fn report(&self, progress: Progress) {
        // The screen can be closed; a lost progress report is not an error.
        self.0.send(progress).ok();
    }
}
