// module schema

use clap::Parser;

/// rust-container-tool cli struct
#[derive(Parser, Debug)]
#[command(name = "rust-ragllm-tui")]
#[command(author = "Luigi Mario Zuccarelli <luzuccar@redhat.com>")]
#[command(version = "0.1.0")]
#[command(about = "Front TUI for the RAG Retrieval augmented generation tool JSON web service", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// config file to use
    #[arg(
        short,
        long,
        value_name = "url",
        default_value = "http://localhost:7000/query"
    )]
    pub url: String,

    /// set the loglevel. Valid arguments are info, debug, trace
    #[arg(value_enum, long, value_name = "loglevel", default_value = "info")]
    pub loglevel: Option<String>,
}
