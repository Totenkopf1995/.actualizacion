use std::{env, fs};

pub(crate) fn verificacion_os() -> &'static str {

    let os = env::consts::OS;
    let contents = fs::read_to_string("/etc/os-release")
        .expect("Algo salió mal al leer el archivo");

    match os {
        "android" => "android",
        "linux" => {
            if contents.contains("ubuntu") {
                "ubuntu"
            } else if contents.contains("debian") {
                "debian"
            } else if contents.contains("fedora") {
                "fedora"
            } else {
                println!("No estoy seguro de qué distribución de Linux estás ejecutando.");
                "linux" // Devuelve un valor por defecto
            }
        },
        _ => {
            println!("No estoy seguro de qué sistema operativo estás ejecutando.");
            "desconocido" // Devuelve un valor por defecto
        }
    }
}