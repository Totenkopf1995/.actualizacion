use chrono::{Local, DateTime};
use std::io::{self, Write};
use std::env;

fn main() {

    let mut input:char = ' ';

    while input != 's' || input != 'n' {

    }
}

fn imprimir_fecha() {
    let ahora: DateTime<Local> = Local::now();
    println!("{}", ahora);
}

fn os() {
    match env::consts::OS {
        "android" => println!("Estás ejecutando Android!"),
        "linux" => println!("Estás ejecutando Linux!"),
        _ => println!("No estoy seguro de qué sistema operativo estás ejecutando."),
    }
}