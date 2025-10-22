use crate::sqlite::connect_db;
use rusqlite::{Result, Row};
use std::fmt::Display;

trait CommandStat {
    fn command(&self) -> &str;
    fn value(&self) -> Box<dyn Display>;
}

#[derive(Debug)]
pub struct CommandRuntime {
    pub command: String,
    pub hours: f32,
}

impl CommandStat for CommandRuntime {
    fn command(&self) -> &str {
        &self.command
    }

    // We can even include formatting logic right in the trait implementation
    fn value(&self) -> Box<dyn Display> {
        Box::new(format!("{:.2}", self.hours))
    }
}

fn query_and_print_stats<T, F>(title: &str, sql: &str, row_mapper: F) -> Result<()>
where
    T: CommandStat,
    F: FnMut(&Row) -> Result<T>,
{
    println!("\n## {} ##", title);

    let db = connect_db()?;
    let mut stmt = db.prepare(sql)?;

    let rows = stmt.query_map([], row_mapper)?;
    let commands: Vec<T> = rows.collect::<Result<Vec<_>, _>>()?;

    if commands.is_empty() {
        println!("No data found.");
        return Ok(());
    }

    let longest_cmd_length = commands
        .iter()
        .map(|cmd| cmd.command().len())
        .max()
        .unwrap_or(10);

    println!("{:<width$} | Value", "Command", width = longest_cmd_length);
    println!("{:-<width$}-|-{:-<5}", "", "", width = longest_cmd_length);

    for cmd in commands {
        println!(
            "{:<width$} | {}",
            cmd.command(),
            cmd.value(),
            width = longest_cmd_length
        );
    }

    Ok(())
}

#[derive(Debug)]
pub struct CommandCount {
    pub command: String,
    pub count: i32,
}

impl CommandStat for CommandCount {
    fn command(&self) -> &str {
        &self.command
    }

    fn value(&self) -> Box<dyn Display> {
        Box::new(self.count)
    }
}

pub fn most_frequent_cmd() -> Result<()> {
    let sql = "SELECT cmd as command, COUNT(cmd) as count 
               FROM commands 
               GROUP BY cmd 
               ORDER BY count DESC 
               LIMIT 10";

    query_and_print_stats("Top 10 Most Frequent Commands", sql, |row| {
        Ok(CommandCount {
            command: row.get(0)?,
            count: row.get(1)?,
        })
    })
}

pub fn cmd_runtimes() -> Result<()> {
    let sql = "SELECT cmd, (SUM(duration_ms) / 3600000.0) as runtime_hours 
               FROM commands 
               GROUP BY cmd 
               ORDER BY runtime_hours DESC 
               LIMIT 10";

    query_and_print_stats("Top 10 Commands by Runtime (hours)", sql, |row| {
        Ok(CommandRuntime {
            command: row.get(0)?,
            hours: row.get(1)?,
        })
    })
}

#[derive(Debug)]
pub struct CommandAvgRuntime {
    pub command: String,
    pub avg_ms: f64,
}

impl CommandStat for CommandAvgRuntime {
    fn command(&self) -> &str {
        &self.command
    }
    fn value(&self) -> Box<dyn Display> {
        Box::new(format!("{:.0} ms", self.avg_ms))
    }
}

pub fn cmd_avg_runtime() -> Result<()> {
    let sql = "SELECT cmd, AVG(duration_ms) as avg_ms
               FROM commands 
               GROUP BY cmd 
               ORDER BY avg_ms DESC 
               LIMIT 10";

    query_and_print_stats("Top 10 Commands by Average Runtime (ms)", sql, |row| {
        Ok(CommandAvgRuntime {
            command: row.get(0)?,
            avg_ms: row.get(1)?,
        })
    })
}
