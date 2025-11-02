use crate::{sqlite::connect_db, table::create_cmd_table};
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
        Box::new(format!("{:.2} h", self.hours))
    }
}

pub struct PrintStat {
    pub table: Table,
    pub title: String,
}


fn query_statistic<T, F, R>(title: &str, sql: &str, row_mapper: F, params: R) -> Result<PrintStat>
where
    T: CommandStat,
    F: FnMut(&Row) -> Result<T>,
    R: Params,
{
    let db = connect_db()?;
    let mut stmt = db.prepare(sql)?;

    let rows = stmt.query_map(params, row_mapper)?;
    let commands: Vec<T> = rows.collect::<Result<Vec<_>, _>>()?;

    let print_stat = PrintStat {table: create_cmd_table(commands)?, title: title.to_string()};

    Ok(print_stat)
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

pub fn most_frequent_cmd() -> Result<PrintStat> {
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

pub fn cmd_runtimes(time_interval: Duration) -> Result<PrintStat> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time_interval = (current_time.as_millis() - time_interval.as_millis()) as i64;
    let sql = "SELECT cmd, (SUM(duration_ms) / 3600000.0) as runtime_hours 
               FROM commands 
               WHERE timestamp > ?1
               GROUP BY cmd 
               ORDER BY runtime_hours DESC 
               LIMIT 3";

    query_statistic(
        "Top 3 Commands by Runtime (hours)",
        sql,
        |row| {
            Ok(CommandRuntime {
                command: row.get(0)?,
                hours: row.get(1)?,
            })
        },
        params![time_interval],
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
        Box::new(format!("{:.2} h", self.avg_ms))
    }
}


pub fn cmd_avg_runtime(time_interval: Duration) -> Result<PrintStat> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time_interval = (current_time.as_millis() - time_interval.as_millis()) as i64;

    let sql = "SELECT cmd, (AVG(duration_ms) / 3600000.0) as avg_ms
               FROM commands 
               WHERE timestamp > ?1
               GROUP BY cmd 
               ORDER BY avg_ms DESC 
               LIMIT 3";

    query_statistic(
        "Top 3 Commands by Average Runtime (ms)",
        sql,
        |row| {
            Ok(CommandAvgRuntime {
                command: row.get(0)?,
                avg_ms: row.get(1)?,
            })
        },
        params![time_interval],
    )
}

pub fn most_used_command(time_interval: Duration) -> Result<PrintStat> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time_interval = (current_time.as_millis() - time_interval.as_millis()) as i64;

    let sql = "SELECT cmd as command, COUNT(*) AS count FROM commands WHERE timestamp > ?1 GROUP BY cmd ORDER BY count DESC LIMIT 3";

    query_statistic(
        "Top 3 Commands by timeinterval",
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
