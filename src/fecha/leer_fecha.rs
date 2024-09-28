use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;
use crate::fecha::imprimir_fecha::imprimir_fecha;

pub(crate) fn leer_fecha() -> io::Result<()> {

    let nombre_archivo = "fecha.txt";

    if !Path::new(nombre_archivo).exists() {
        imprimir_fecha()?;
    } else {
        let file = File::open(nombre_archivo)?; //abre archivo en modo lectura
        let mut buf_reader = BufReader::new(file); //Buffer para leer
        let mut contenido = String::new(); //almacena el contenido del archivo

        buf_reader.read_to_string(&mut contenido)?; //lee el contenido

        println!("Fecha de la ultima actualizacion: {}", contenido);
    }
    Ok(()) //todo salió bien
}