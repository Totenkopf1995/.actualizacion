use chrono::{Local, DateTime};
use std::env;
use std::io;
use std::fs;
use std::process::Command;

fn main() {

    let mut input = String::new();

    println!("Desea Actualizar(s/n): ");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");

    // Convertir la entrada a un carácter único
    let mut input_char = input.trim().chars().next().expect("No se ingresó ningún caracter");

    // Comparar con caracteres individuales
    while input_char != 's' && input_char != 'n' {
        println!("Ingrese 's' para sí o 'n' para no:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        input_char = input.trim().chars().next().expect("No se ingresó ningún caracter");
    }

    println!("Respuesta válida: {}", input_char);
    os();
    imprimir_fecha();
}

fn imprimir_fecha() {
    let ahora: DateTime<Local> = Local::now();
    println!("{}", ahora);
}

fn os() {

    let os = env::consts::OS;
    let contents = fs::read_to_string("/etc/os-release")
        .expect("Algo salió mal al leer el archivo");

    match os {
        "android" => {
            println!("Estás ejecutando Android!");
        },
        "linux" => {
            println!("Estás ejecutando Linux!");
            if contents.contains("ubuntu") {
                println!("Estás ejecutando Ubuntu!");
            } else if contents.contains("debian") {
                println!("Estás ejecutando Debian!");
            } else if contents.contains("fedora") {
                println!("Estás ejecutando Fedora!");
            } else {
                println!("No estoy seguro de qué distribución de Linux estás ejecutando.");
            }
        },
        _ => println!("No estoy seguro de qué sistema operativo estás ejecutando."),
    }
}