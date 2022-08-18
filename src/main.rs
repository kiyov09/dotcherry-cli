extern crate dotenv;
extern crate notify;

mod cli_args;
mod file_watcher;

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

use postgrest::Postgrest;
use std::{fs::read_to_string, path::Path, process::Command};


#[tokio::main]
async fn main() {

    let clap_args = cli_args::get_cli_args();
    let path = &clap_args.dot_file;
    let watch = clap_args.watch;

    // Create a path to a file.
    let path = Path::new(path);

    // Read the file and render the graph
    read_and_render(path);

    if watch {
        file_watcher::init_watcher(path, &read_and_render);
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
        name: "RUST IS COOL".to_string(),
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
