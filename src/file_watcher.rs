use std::{path::Path, sync::mpsc::channel, time::Duration};
use notify::{watcher, RecursiveMode, Watcher, DebouncedEvent};
use crate::database;

pub async fn init_watcher(path: &Path, graph: &mut database::Graph) {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(0)).unwrap();
    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path) | DebouncedEvent::Write(path)  => {
                    println!("Change detected. Saving graph...");
                    let (graph_name, graph_code) = crate::get_graph_info_from_dot_file(&path);
                    update_graph_and_save(graph, &graph_name, &graph_code).await;
                }
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

async fn update_graph_and_save(graph: &mut database::Graph, graph_name: &str, graph_code: &str) {
    graph.set_name(graph_name);
    graph.set_code(graph_code);
    graph.save().await.unwrap();
}
