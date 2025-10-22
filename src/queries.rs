use rusqlite::Result;
use crate::sqlite::connect_db;

#[derive(Debug)]
pub struct CommandCount {
    pub command: String,
    pub count: i32,
}

#[derive(Debug)]
pub struct CommandRuntime {
    pub command: String,
    pub hours: f32,
}

pub fn most_frequent_cmd() -> Result<()> {
    let db = connect_db()?;
    let mut stmt = db.prepare("SELECT cmd as command, COUNT(cmd) as count FROM commands GROUP BY cmd ORDER BY count DESC LIMIT 10")?;
    let rows = stmt.query_map([], |row|{
        Ok(
            CommandCount {
                command: row.get(0)?,
                count: row.get(1)?,
            }
        )
    })?;

    let commands: Vec<CommandCount> = rows.collect::<Result<Vec<_>, _>>()?;

    let longest_cmd_length = commands
        .iter()
        .map(|cmd| cmd.command.len())
        .max()
        .unwrap_or(10);

    for cmd in commands {
        println!(
            "{:<width$} | {}",
            cmd.command,
            cmd.count,
            width = longest_cmd_length
        );
    }

    Ok(())
}

pub fn cmd_runtimes() -> Result<()> {
    let db = connect_db()?;
    let mut stmt = db.prepare("SELECT cmd, (SUM(duration_ms)/3600.0) as runtime_hours FROM commands GROUP BY cmd ORDER BY runtime_hours DESC LIMIT 10")?;

    let rows = stmt.query_map([], |row|{
        Ok(
            CommandRuntime {
                command: row.get(0)?,
                hours: row.get(1)?,
            }
        )
    })?;

    let commands: Vec<CommandRuntime> = rows.collect::<Result<Vec<_>, _>>()?;

    let longest_cmd_length = commands
        .iter()
        .map(|cmd| cmd.command.len())
        .max()
        .unwrap_or(10);

    for cmd in commands {
        println!(
            "{:<width$} | {}",
            cmd.command,
            cmd.hours,
            width = longest_cmd_length
        );
    }

    Ok(())
}
