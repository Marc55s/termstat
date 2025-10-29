use crate::{sqlite::connect_db, table::create_table};
use rusqlite::{Params, Result, Row, params};
use std::fmt::Display;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use comfy_table::Table;

pub trait CommandStat {
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

    fn value(&self) -> Box<dyn Display> {
        Box::new(format!("{:.2}", self.hours))
    }
}

fn query_statistic<T, F, R>(title: &str, sql: &str, row_mapper: F, params: R) -> Result<Table>
where
    T: CommandStat,
    F: FnMut(&Row) -> Result<T>,
    R: Params,
{
    let db = connect_db()?;
    let mut stmt = db.prepare(sql)?;

    let rows = stmt.query_map(params, row_mapper)?;
    let commands: Vec<T> = rows.collect::<Result<Vec<_>, _>>()?;

    create_table(commands)
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

pub fn most_frequent_cmd() -> Result<Table> {
    let sql = "SELECT cmd as command, COUNT(cmd) as count 
               FROM commands 
               GROUP BY cmd 
               ORDER BY count DESC 
               LIMIT 3";

    query_statistic(
        "Top 10 Most Frequent Commands",
        sql,
        |row| {
            Ok(CommandCount {
                command: row.get(0)?,
                count: row.get(1)?,
            })
        },
        [],
    )
}

pub fn cmd_runtimes() -> Result<Table> {
    let sql = "SELECT cmd, (SUM(duration_ms) / 3600000.0) as runtime_hours 
               FROM commands 
               GROUP BY cmd 
               ORDER BY runtime_hours DESC 
               LIMIT 3";

    query_statistic(
        "Top 10 Commands by Runtime (hours)",
        sql,
        |row| {
            Ok(CommandRuntime {
                command: row.get(0)?,
                hours: row.get(1)?,
            })
        },
        [],
    )
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

pub fn cmd_avg_runtime() -> Result<Table> {
    let sql = "SELECT cmd, AVG(duration_ms) as avg_ms
               FROM commands 
               GROUP BY cmd 
               ORDER BY avg_ms DESC 
               LIMIT 3";

    query_statistic(
        "Top 10 Commands by Average Runtime (ms)",
        sql,
        |row| {
            Ok(CommandAvgRuntime {
                command: row.get(0)?,
                avg_ms: row.get(1)?,
            })
        },
        [],
    )
}

pub fn most_used_command(time_interval: Duration) -> Result<Table> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time_interval = (current_time.as_millis() - time_interval.as_millis()) as i64;

    println!("time_interval: {}", time_interval);

    let sql = "SELECT cmd as command, COUNT(*) AS count FROM commands WHERE timestamp > ?1 GROUP BY cmd ORDER BY count DESC LIMIT 3";

    query_statistic(
        "Top 10 Commands by timeinterval",
        sql,
        |row| {
            Ok(CommandCount {
                command: row.get(0)?,
                count: row.get(1)?,
            })
        },
        params![time_interval],
    )
}
