use crate::queries::CommandStat;
use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use rusqlite::{Result};

pub fn create_table<T>(cmd_stats: Vec<T>) -> Result<Table> where T: CommandStat {
    let mut table = Table::new();

    if cmd_stats.is_empty() {
        return Ok(table);        
    }

    table.set_header(vec!["Cmd", "Value"])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    for stat in cmd_stats {
        let x = stat.value().to_string();
        table.add_row(vec![&stat.command(), x.as_str()]);
    }

    Ok(table)
}
