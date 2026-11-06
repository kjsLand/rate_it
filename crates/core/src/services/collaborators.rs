//! Collaborators: the artists who made songs with the current artist.

use std::collections::HashMap;

use crate::App;
use crate::dto::Collaborator;
use crate::error::CoreResult;
use crate::progress::{CancellationToken, ProgressSink};

impl App {
    /// The artists on the songs of the current artist (or `artist_id`), most songs first.
    pub async fn collaborators(
        &self,
        artist_id: Option<&str>,
        progress: &dyn ProgressSink,
        cancel: &CancellationToken,
    ) -> CoreResult<Vec<Collaborator>> {
        let artist = self.target_artist(artist_id).await?;
        let tracks = self
            .discography_tracks(&artist.id, "collaborators", progress, cancel)
            .await?;
        let mut found: HashMap<String, Collaborator> = HashMap::new();
        let mut seen_tracks = std::collections::HashSet::new();
        for track in tracks
            .into_iter()
            .filter(|t| seen_tracks.insert(t.id.clone()))
        {
            for person in track.artists.into_iter().filter(|a| a.id != artist.id) {
                found
                    .entry(person.id.clone())
                    .or_insert_with(|| Collaborator {
                        id: person.id,
                        name: person.name,
                        track_count: 0,
                    })
                    .track_count += 1;
            }
        }
        let mut list: Vec<Collaborator> = found.into_values().collect();
        list.sort_by(|a, b| {
            b.track_count
                .cmp(&a.track_count)
                .then_with(|| a.name.cmp(&b.name))
        });
        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use crate::dto::Progress;
    use crate::fakes::{album, app};
    use crate::progress::{CancellationToken, ProgressSink};

    #[derive(Default)]
    struct Recorder(Mutex<Vec<Progress>>);

    impl ProgressSink for Recorder {
        fn report(&self, progress: Progress) {
            self.0.lock().unwrap().push(progress);
        }
    }

    #[tokio::test]
    async fn collaborators_are_counted_and_progress_is_reported() {
        let (app, spotify) = app().await;
        spotify.add_album(
            "a1",
            album("al1", &[("t1", &["a1", "b1"]), ("t2", &["a1", "c1"])]),
        );
        spotify.add_album(
            "a1",
            album("al2", &[("t3", &["a1", "b1"]), ("t1", &["a1", "b1"])]),
        );
        let recorder = Recorder::default();
        let list = app
            .collaborators(Some("a1"), &recorder, &CancellationToken::new())
            .await
            .unwrap();
        let ids: Vec<(&str, u32)> = list
            .iter()
            .map(|c| (c.id.as_str(), c.track_count))
            .collect();
        assert_eq!(ids, vec![("b1", 2), ("c1", 1)]);
        let reports = recorder.0.lock().unwrap();
        assert_eq!(reports.first().map(|p| (p.done, p.total)), Some((0, 2)));
        assert_eq!(reports.last().map(|p| (p.done, p.total)), Some((2, 2)));
    }
}
