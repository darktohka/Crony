use std::{
    fs::OpenOptions,
    process::{Command, Stdio},
};

use crate::{
    datetime::{get_now, human_datetime},
    history::{HistoryEntry, TaskResult},
    info,
    paths::TaskPaths,
    save::append_to_history,
    task::Task,
    warn,
};
use anyhow::{bail, Context, Result};

pub static DEFAULT_SHELL_CMD: &str = "/bin/sh -c";

pub fn runner(
    task: &Task,
    paths: &TaskPaths,
    use_log_files: bool,
    // was_task_removed: impl FnOnce() -> bool,
) -> Result<HistoryEntry> {
    if !paths.dir().exists() {
        bail!("Task's directory was not found!");
    }

    let started_at = get_now();

    info!(
        "Starting task '{}' on {}...",
        task.name.bright_yellow(),
        human_datetime(started_at).bright_magenta()
    );

    let shell_cmd = task
        .shell
        .clone()
        .unwrap_or_else(|| DEFAULT_SHELL_CMD.to_string());

    let mut shell_cmd_parts = shell_cmd.split(' ');

    let mut cmd = Command::new(shell_cmd_parts.next().unwrap());

    for part in shell_cmd_parts {
        cmd.arg(part);
    }

    cmd.arg(&task.cmd);

    if use_log_files {
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(paths.log_file())
            .context("Failed to open the task's log file")?;

        cmd.stdout(Stdio::from(log_file.try_clone().unwrap()));
        cmd.stderr(Stdio::from(log_file));
    }

    let status = cmd.status().context("Failed to run the task's command")?;

    let ended_at = get_now();

    let result = if status.success() {
        TaskResult::Success
    } else {
        TaskResult::Failed {
            code: status.code(),
        }
    };

    info!(
        "Task '{}' finished running on {} ({})",
        task.name.bright_yellow(),
        human_datetime(ended_at).bright_magenta(),
        match result {
            TaskResult::Success => format!("{}", result).bright_green(),
            TaskResult::Failed { code: _ } => format!("{}", result).bright_red(),
        }
    );

    let entry = HistoryEntry {
        task_id: task.id,
        task_name: task.name.clone(),
        started_at,
        ended_at,
        result,
    };

    if !paths.dir().exists() {
        warn!(
            "Task '{}' was removed during its execution, skipping history update.",
            task.name
        );
    } else {
        append_to_history(paths, &entry)?;
    }

    Ok(entry)
}
