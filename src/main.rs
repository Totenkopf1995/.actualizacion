use chrono::{Local, DateTime};
use std::io::{self, Write};

fn main() {
    let mut respuesta = String::new();

    while respuesta.trim().to_lowercase() != "s" && respuesta.trim().to_lowercase() != "n" {
        println!("¿Desea actualizar? (s/n)");
        io::stdout().flush().unwrap(); // Limpiar el búfer de stdout
        io::stdin().read_line(&mut respuesta).unwrap();
        if respuesta.trim().to_lowercase() != "s" && respuesta.trim().to_lowercase() != "n" {
            println!("La respuesta no es correcta. Por favor, responda con 's' o 'n'.");
        }
    }

    if respuesta.trim().to_lowercase() == "s" {
        println!("El usuario eligió actualizar.");
    } else {
        println!("El usuario eligió no actualizar.");
    }
}

fn imprimir_fecha() {
    let ahora: DateTime<Local> = Local::now();
    println!("{}", ahora);
}
