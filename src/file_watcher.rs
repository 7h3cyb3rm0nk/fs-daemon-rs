use notify::{Watcher, RecursiveMode, event::Event};
use std::sync::mpsc::{channel, Receiver};
use std::path::{Path,PathBuf};
use notify::Error;
pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    rx: Receiver<Result<Event, notify::Error>>,
}

impl FileWatcher {
    pub fn new() -> Result<Self, notify::Error> {
        let (tx, rx) = channel();
        let watcher = notify::RecommendedWatcher::new(move |res| {
            tx.send(res).expect("Failed to send file system event");
        }, notify::Config::default())?;
        Ok(Self{watcher, rx})
    }

    pub fn watch_directory(&mut self, path: &str) -> Result<(), notify::Error> {
        self.watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn receive_event(&self) -> Result<Event, notify::Error> {
        self.rx.recv().map_err(|e| notify::Error::generic(&e.to_string())).unwrap()
    }


}


