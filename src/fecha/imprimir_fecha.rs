use chrono::{DateTime, Local};

pub(crate) fn imprimir_fecha() {
    let ahora: DateTime<Local> = Local::now();
    println!("{}", ahora);
}