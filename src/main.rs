use chrono::{Local, DateTime};
use std::io::{self, Write};

fn main() {

    let mut input:char = ' ';

    while input != 's' || input != 'n' {

    }
}

fn imprimir_fecha() {
    let ahora: DateTime<Local> = Local::now();
    println!("{}", ahora);
}
