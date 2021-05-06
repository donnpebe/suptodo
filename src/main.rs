mod cli;
mod tasks;

use std::path::PathBuf;

use anyhow::anyhow;
use structopt::StructOpt;

use cli::{
    Action::{Add, Done, List},
    CommandLineArgs,
};
use tasks::Task;

fn find_default_todo_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".suptodo.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action, todo_file } = cli::CommandLineArgs::from_args();

    let todo_file = todo_file
        .or_else(find_default_todo_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    println!("{:?}", todo_file);

    match action {
        Add { task } => tasks::add_task(&todo_file, Task::new(task)),
        Done { position } => tasks::complete_task(&todo_file, position),
        List => tasks::list_tasks(&todo_file),
    }?;

    Ok(())
}
