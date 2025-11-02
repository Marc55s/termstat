use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use crate::queries::PrintStat;

pub fn print_stats(tables: Vec<PrintStat>) -> Result<(), std::io::Error> {
    let mut layout_table = Table::new();
    let headers: Vec<_> = tables.iter().map(|e| e.title.as_str()).collect();

    layout_table.set_header(headers)
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    let tables_str : Vec<_> = tables.iter().map(|e| e.table.to_string()).collect();
    layout_table.add_row(tables_str);

    println!("{layout_table}");
    Ok(())
}
