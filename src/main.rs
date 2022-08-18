extern crate dotenv;
extern crate notify;

use futures::executor::block_on;

use dotenv::dotenv;
use serde::Serialize;
use std::env;

use graphviz_rust::{
    cmd::{CommandArg, Format},
    exec, parse,
    printer::PrinterContext,
};

use std::result::Result;

use notify::{watcher, RecursiveMode, Watcher};
use postgrest::Postgrest;
use std::{fs::read_to_string, path::Path, process::Command, sync::mpsc::channel, time::Duration};

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Dot file to parse
    dot_file: String,

    /// Watch for changes to the dot file and re-run the program
    #[clap(short, long, value_parser, default_value = "false")]
    watch: bool,

    /// Generate a permanent link to the resulting graph
    #[clap(short, long, value_parser, default_value = "false")]
    permanent: bool,
}

#[tokio::main]
async fn main() {

    let clap_args = CliArgs::parse();
    let path = &clap_args.dot_file;
    let watch = clap_args.watch;

    // Create a path to a file.
    let path = Path::new(path);

    // Read the file and render the graph
    read_and_render(path);

    if watch {
        init_watcher(path);
    }
}

fn init_watcher(path: &Path) {
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
                    read_and_render(&path);
                }
                notify::DebouncedEvent::Write(path) => {
                    println!("The file {:?} has been written to", &path);
                    read_and_render(&path);
                }
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn read_and_render(path: &Path) {
    let content = read_to_string(path).unwrap();
    render_dot(&content);

    // Save on db
    let _result = block_on(insert_on_db(&content));
}

fn render_dot(str: &str) {
    // let g: Graph = parse(str).unwrap();

    let parse_result = parse(str);

    // If parsing failed, print the error and skip rendering
    if let Err(e) = parse_result {
        println!("{}", e);
        return;
    }

    let g = parse_result.unwrap();

    // Generate the svg file
    let result = exec(
        g,
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

fn get_env_var(var_name: &str) -> Option<String> {
    dotenv().ok();

    for (key, value) in env::vars() {
        if key == var_name {
            return Some(value);
        }
    }

    None
}

fn get_supabase_key() -> Option<String> {
    get_env_var("SUPABASE_API_KEY")
}

fn get_supabase_url() -> Option<String> {
    get_env_var("SUPABASE_API_URL")
}

fn get_postgrest_client() -> postgrest::Postgrest {
    let supabase_api_key = get_supabase_key().unwrap();
    let supabase_api_url = get_supabase_url().unwrap();

    Postgrest::new(supabase_api_url).insert_header("apikey", supabase_api_key)
}

#[derive(Debug, Serialize)]
struct Gragh {
    name: String,
    user_id: String,
    code: String
}

async fn insert_on_db(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_postgrest_client();

    let graph = Gragh {
        name: "Testing from rust".to_string(),
        user_id: "7febcbe7-a9d4-48b4-99c5-8c1f290ae934".to_string(),
        code: code.to_string()
    };

    let resp = client
        .from("graphs")
        .insert(
            serde_json::to_string(&graph).unwrap()
        )
        .execute()
        .await?;

    let body = resp.text().await?;

    println!("{:}", body);

    Ok(())
}
