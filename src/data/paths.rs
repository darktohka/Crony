use std::path::PathBuf;

#[derive(Clone)]
pub struct Paths {
    pub data_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub run_dir: PathBuf,
    pub tasks_dir: PathBuf,

    pub tasks_file: PathBuf,
    pub history_file: PathBuf,

    pub daemon_socket_file: PathBuf,
    pub daemon_log_file: PathBuf,
}

impl Paths {
    pub fn new(data_dir: PathBuf, logs_dir: PathBuf, run_dir: PathBuf) -> Self {
        Self {
            tasks_dir: logs_dir.join("tasks"),

            tasks_file: data_dir.join("tasks.json"),
            history_file: data_dir.join("history.json"),

            daemon_socket_file: run_dir.join("crony.sock"),
            daemon_log_file: logs_dir.join("crony.log"),

            data_dir,
            logs_dir,
            run_dir,
        }
    }

    pub fn task_log_file(&self, task_name: &str) -> PathBuf {
        self.tasks_dir.join(format!("{task_name}.log"))
    }
}
