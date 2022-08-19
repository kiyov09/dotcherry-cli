extern crate notify;

mod cli_args;
mod file_watcher;
mod database;

use std::{
    fs::read_to_string,
    path::Path,
    process::Command,
};
use futures::executor::block_on;
use graphviz_rust::{
    cmd::{CommandArg, Format},
    exec, parse,
    printer::PrinterContext,
};

#[tokio::main]
async fn main() {
    // Get CLI args
    let clap_args = cli_args::get_cli_args();
    let path = &clap_args.dot_file;
    let watch = clap_args.watch;

    // Create a path to a file.
    let path = Path::new(path);

    // Read the file and render the graph
    read_and_render(path);

    if watch {
        // Init the watch mode
        file_watcher::init_watcher(path, &read_and_render);
    }
}

fn filename_from_path(path: &Path) -> String {
    path.file_stem().unwrap()
        .to_str().unwrap()
        .to_string()
}

fn read_and_render(path: &Path) {
    let file_name = filename_from_path(path);
    let content = read_to_string(path).unwrap();

    // This line is for local testing purposes
    render_dot(&content);

    // Save on db
    let _result = block_on(database::save_graph(&file_name, &content));
}

fn render_dot(str: &str) {
    let parse_result = parse(str);

    // If parsing failed, print the error and skip rendering
    if let Err(e) = parse_result {
        println!("{}", e);
        return;
    }

    let parsed_code = parse_result.unwrap();

    // Generate the svg file
    let result = exec(
        parsed_code,
        &mut PrinterContext::default(),
        vec![
            CommandArg::Format(Format::Svg),
            CommandArg::Output("path_to_file.svg".to_string()),
        ],
    );

    // If rendering failed, print the error and skip reload the chrome tab
    if let Err(e) = result {
        println!("{}", e);
        return;
    }

    // Reload the active chrome tab (this is jsut for testing)
    reload_active_tab_on_chrome();
}

// This is just during testing
fn reload_active_tab_on_chrome() {
    Command::new("chrome-cli").arg("reload").output().unwrap();
}

// Test section start here

#[cfg(test)]
mod tests {
    #[test]
    fn filename_from_path_works() {
        let path = super::Path::new("/home/user/graph.dot");
        let file_name = super::filename_from_path(path);
        assert_eq!(file_name, "graph");
    }
}
