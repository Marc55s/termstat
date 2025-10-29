use comfy_table::Table;

pub fn print_stats(tables: Vec<Table>) -> Result<(), std::io::Error> {
    let mut layout_table = Table::new();
    layout_table.set_header(vec!["Most used command", "avg runtime", "runtime"]);


    let tables_str : Vec<_> = tables.iter().map(|e| e.to_string()).collect();
    layout_table.add_row(tables_str);

    println!("{layout_table}");
    Ok(())
}
