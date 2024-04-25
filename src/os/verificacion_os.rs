use std::{env, fs};

pub(crate) fn verificacion_os() {

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