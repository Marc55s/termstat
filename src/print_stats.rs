use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;

pub fn print_stats(tables: Vec<Table>) -> Result<(), std::io::Error> {
    let mut layout_table = Table::new();
    layout_table.set_header(vec!["Most used command", "average runtime", "runtime"])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    let tables_str : Vec<_> = tables.iter().map(|e| e.to_string()).collect();
    layout_table.add_row(tables_str);

    println!("{layout_table}");
    Ok(())
}
