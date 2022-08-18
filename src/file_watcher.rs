use std::{path::Path, sync::mpsc::channel, time::Duration};

use notify::{watcher, RecursiveMode, Watcher};

pub fn init_watcher(path: &Path, on_change: &dyn Fn(&Path)) {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(0)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                notify::DebouncedEvent::Create(path) => {
                    println!("The file {:?} has been created", &path);
                    on_change(&path);
                }
                notify::DebouncedEvent::Write(path) => {
                    println!("The file {:?} has been written to", &path);
                    on_change(&path);
                }
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
