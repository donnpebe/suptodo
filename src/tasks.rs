use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::Path;
use std::{
    fmt,
    fs::{File, OpenOptions},
};

use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Self {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

fn collect_task(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;

    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}

pub fn add_task(filepath: &Path, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filepath)?;

    let mut tasks: Vec<Task> = collect_task(&file)?;

    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(filepath: &Path, task_position: usize) -> Result<()> {
    let file = OpenOptions::new().read(true).write(true).open(filepath)?;

    let mut tasks = collect_task(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }

    tasks.remove(task_position - 1);

    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(filepath: &Path) -> Result<()> {
    let file = OpenOptions::new().read(true).open(filepath)?;

    let tasks = collect_task(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}
