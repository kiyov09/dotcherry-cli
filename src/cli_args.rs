use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Dot file to parse
    pub dot_file: String,

    /// Id of the graph to update
    #[clap(short, long, value_parser, name = "graph_id")]
    pub update: Option<String>,

    /// Watch for changes to the dot file and re-run the program
    #[clap(short, long, value_parser, default_value = "false")]
    pub watch: bool,

    /// Generate a permanent link to the resulting graph
    #[clap(short, long, value_parser, default_value = "false")]
    pub permanent: bool,
}

pub fn get_cli_args() -> CliArgs {
    CliArgs::parse()
}
