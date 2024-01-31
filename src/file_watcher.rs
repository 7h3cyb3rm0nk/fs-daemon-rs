use notify::{Watcher, RecursiveMode, DeboundedEvent};
use std::sync::mpsc::{channel, Receiver};
use std::path::PathBuf;

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    rx: Receiver<DeboundedEvent>,
}

impl FileWatcher {
    pub fn new() -> Result<Self, notify::Error> {
        let (tx, rx) = channel();
        let watcher = notify::Watcher::new_immediate(move |res| {
            tx.send(res).expect("Failed to send file system event");
        })?;
        Ok(Self{watcher, rx})
    }

    pub fn watch_directory(&mut self, path: &str) -> Result<(), notify::Error> {
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn receive_event(&self) -> Result<DebouncedEvent, notify::Error> {
        self.rx.recv().map_err(|e| notify::Error::Generic(e.to_string()))
    }


}

impl Drop for FileWatcher {
    fn drop(&mut self) {
        let _ = self.watcher.unwatch_all();
    }
}
