mod cli_args;
mod database;
mod file_watcher;

use std::{
    fs::read_to_string,
    path::Path,
};

#[tokio::main]
async fn main() {
    // Get CLI args
    let clap_args = cli_args::get_cli_args();
    let path = &clap_args.dot_file;
    let watch = clap_args.watch;
    let graph_id = clap_args.update;

    // Create a path to a file.
    let path = Path::new(path);

    // Init graph
    let mut graph = database::Graph::new();
    graph.set_id(graph_id);

    let (graph_name, graph_code) = get_graph_info_from_dot_file(&path);
    update_graph_and_save(&mut graph, &graph_name, &graph_code).await;

    if let Some(graph_id) = &graph.id {
        println!("Your graph is ready! Access it from:\n\n    http://localhost:5004/graph/{}\n", graph_id);
    }

    if watch {
        // Init the watch mode
        println!("Watching for changes...");
        file_watcher::init_watcher(path, &mut graph).await;
    };
}

async fn update_graph_and_save(graph: &mut database::Graph, graph_name: &str, graph_code: &str) {
    graph.set_name(graph_name);
    graph.set_code(graph_code);
    graph.save().await.unwrap();
}

fn get_graph_info_from_dot_file(path: &Path) -> (String, String) {
    let file_name = filename_from_path(path);
    let graph_code = read_to_string(path).unwrap();
    (file_name, graph_code)
}

fn filename_from_path(path: &Path) -> String {
    path.file_stem().unwrap().to_str().unwrap().to_string()
}


#[cfg(test)]
mod tests {
    #[test]
    fn filename_from_path_works() {
        let path = super::Path::new("/home/user/graph.dot");
        let file_name = super::filename_from_path(path);
        assert_eq!(file_name, "graph");
    }
}
