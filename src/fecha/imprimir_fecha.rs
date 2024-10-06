use chrono::{DateTime, Local};
use std::env;
use std::fs::File;
use std::io::{self, Write};

pub(crate) fn imprimir_fecha() -> io::Result<()> {
    // Obtener la ruta al directorio home
    let home_dir = env::var("HOME").expect("No se pudo obtener el directorio home");
    let nombre_archivo = format!("{}/.fecha.txt", home_dir);

    // Crear el archivo
    let mut file = File::create(nombre_archivo).expect("No se pudo crear el archivo");

    // Obtener la hora actual
    let hora: DateTime<Local> = Local::now();

    // Convertir la hora a una cadena
    let hora_str = hora.to_string();

    // Escribir la cadena en el archivo
    file.write_all(hora_str.as_bytes())
        .expect("Error al escribir en el archivo");

    Ok(()) // todo salió bien
}
