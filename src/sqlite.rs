use uuid::Uuid;
use std::{path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

enum ShellType {
    Bash,
    Zsh
}

struct CommandEntry {
    timestamp: SystemTime,
    user: String,
    session: Uuid,
    shell_type: ShellType,
    cmd: String,
    cwd: PathBuf,
    exit_code: i32,
    duration_sec: i32
}

// fn init_table();
// fn insert_command_entry(entry: CommandEntry);
