
use std::fs::File;
use std::io::{self, Write};
use chrono::{DateTime, Local};

pub(crate) fn imprimir_fecha() -> io::Result<()> {
    // Crear el archivo
    let mut file = File::create("fecha.txt").expect("No se pudo crear el archivo");

    // Obtener la hora actual
    let hora: DateTime<Local> = Local::now();

    // Convertir la hora a una cadena
    let hora_str = hora.to_string();

    // Escribir la cadena en el archivo
    file.write_all(hora_str.as_bytes()).expect("Error al escribir en el archivo");

    Ok(()) // todo salió bien
}