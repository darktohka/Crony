use std::path::{Path, PathBuf};

use time::OffsetDateTime;

#[derive(Clone)]
pub struct Paths {
    pub data_dir: PathBuf,

    pub tasks_file: PathBuf,
    pub reload_request_file: PathBuf,

    pub tasks_dir: PathBuf,
    pub old_tasks_dir: PathBuf,
    pub daemon_dir: PathBuf,
}

impl Paths {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            tasks_file: data_dir.join("tasks.json"),
            reload_request_file: data_dir.join("reload-request.tmp"),

            tasks_dir: data_dir.join("tasks"),
            old_tasks_dir: data_dir.join("tasks.old"),
            daemon_dir: data_dir.join("daemon"),

            data_dir,
        }
    }

    pub fn task_paths(&self, task_name: &str) -> TaskPaths {
        TaskPaths {
            task_dir: self.tasks_dir.join(task_name),
        }
    }

    pub fn daemon_paths(&self) -> DaemonPaths {
        DaemonPaths {
            daemon_dir: self.daemon_dir.clone(),
        }
    }

    pub fn generate_old_task_dir_name(&self, task_name: &str) -> PathBuf {
        self.old_tasks_dir.join(format!(
            "{}-{}",
            OffsetDateTime::now_local()
                .expect("Failed to get current date/time")
                .unix_timestamp(),
            task_name
        ))
    }
}

pub struct TaskPaths {
    task_dir: PathBuf,
}

impl TaskPaths {
    pub fn dir(&self) -> &Path {
        &self.task_dir
    }

    pub fn history_file(&self) -> PathBuf {
        self.task_dir.join("history")
    }

    pub fn stdout_log_file(&self) -> PathBuf {
        self.task_dir.join("stdout.log")
    }

    pub fn stderr_log_file(&self) -> PathBuf {
        self.task_dir.join("stderr.log")
    }
}

pub struct DaemonPaths {
    daemon_dir: PathBuf,
}

impl DaemonPaths {
    pub fn dir(&self) -> &Path {
        &self.daemon_dir
    }

    pub fn socket_file(&self) -> PathBuf {
        self.daemon_dir.join("daemon.sock")
    }

    pub fn stdout_log_file(&self) -> PathBuf {
        self.daemon_dir.join("stdout.log")
    }

    pub fn stderr_log_file(&self) -> PathBuf {
        self.daemon_dir.join("stderr.log")
    }
}
