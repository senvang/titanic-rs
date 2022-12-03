use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// start work on a project and track your working times
    Start {
        #[arg(long)]
        customer: String,

        #[arg(long)]
        project: String,

        #[arg(long)]
        comment: Option<String>,
    },
    /// stop working on a project
    Stop {
    },
}



fn main() {
    let _cli = Cli::parse();
}
