use rusqlite::{Connection, Result};
use std::{path::PathBuf, io::{BufReader, BufRead}, path::Path};
use uuid::Uuid;
use serde::Deserialize;
use crate::util::redact::redact_command;

pub const LOG_DIR: &str = "termstat";
pub const LOG_FILE_NAME: &str = "termstat.log";

#[derive(Deserialize, Debug)]
pub struct CommandEntry {
    #[serde(rename = "ts")]
    pub timestamp: i64,

    pub user: String,
    
    pub session: Uuid, 

    #[serde(rename = "shell")]
    pub shell_type: String,

    pub cmd: String,
    
    pub cwd: PathBuf, 

    #[serde(rename = "exit")]
    pub exit_code: i32,

    #[serde(rename = "dur")]
    pub duration_sec: i64,
}

pub fn connect_db() -> Result<Connection> {
    let path = dirs::data_dir().unwrap().join(LOG_DIR).join("termstat.db");
    let db = Connection::open(path)?;

    Ok(db)
}

pub fn insert_cmd_entry(cmd: &mut CommandEntry) -> Result<()> {
    let db = connect_db()?;

    if !db.table_exists(Some("main"), "commands")? {
        db.execute("CREATE TABLE IF NOT EXISTS commands (id INTEGER PRIMARY KEY AUTOINCREMENT, timestamp TEXT, user TEXT, session TEXT, shell_type TEXT, cmd TEXT, cwd TEXT, exit_code INTEGER, duration_ms INTEGER)", [])?;
    }

    cmd.cmd = redact_command(cmd.cmd.as_ref()).unwrap();

    db.execute("INSERT INTO commands 
        (timestamp, user, session, shell_type, cmd, cwd, exit_code, duration_ms)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        [format!("{:?}", cmd.timestamp),  cmd.user.clone(), cmd.session.to_string(), cmd.shell_type.clone(), cmd.cmd.clone(), cmd.cwd.clone().into_os_string().into_string().unwrap(), cmd.exit_code.to_string(), cmd.duration_sec.to_string()])?;

    Ok(())
}

pub fn parse_log_file(path: impl AsRef<Path>) -> Result<Vec<CommandEntry>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line_content = line?;
        // Skip empty lines
        if line_content.trim().is_empty() {
            continue;
        }

        // Parse the JSON string from the line
        match serde_json::from_str::<CommandEntry>(&line_content) {
            Ok(entry) => entries.push(entry),
            Err(e) => eprintln!("Failed to parse line: '{}'. Error: {}", line_content, e),
        }
    }

    Ok(entries)
}
