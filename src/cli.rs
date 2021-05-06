use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the todo file.
    Add {
        /// The task description text.
        #[structopt()]
        task: String,
    },
    /// Remove an entry from the todo file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the todo file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Supa Todo", about = "A command line to-do app written in Rust")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different todo file.
    #[structopt(parse(from_os_str), short, long)]
    pub todo_file: Option<PathBuf>,
}
