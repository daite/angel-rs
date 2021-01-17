#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format};
use reqwest;
use reqwest::header::USER_AGENT;
use select::document::Document;
use select::predicate::{Attr};
use std::error::Error;
use std::sync::{Mutex, Arc};
use regex::Regex;

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