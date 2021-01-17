#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format};

pub mod sites;
mod consts;

pub fn print_table(p: Vec<(String, String)>) {
    let mut table = Table::new();
    table.set_titles(row!["Title", "Magnet"]);
    for zd in p {
        table.add_row(Row::new(vec![
             Cell::new(&zd.0),
             Cell::new(&zd.1),
        ]));
      }
      table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
      table.printstd();
}