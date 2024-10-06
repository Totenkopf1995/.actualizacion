use colour::{e_red_ln, green_ln};
use std::{env, fs, process::Command};

pub(crate) fn verificacion_os() -> &'static str {
    let os = env::consts::OS;

    // Intentar leer el archivo /etc/os-release
    let contents = fs::read_to_string("/etc/os-release").ok();

    match os {
        "android" => "android",
        "linux" => {
            if let Some(contents) = contents {
                if contents.contains("ubuntu") {
                    "ubuntu"
                } else if contents.contains("debian") {
                    "debian"
                } else if contents.contains("fedora") {
                    "fedora"
                } else {
                    e_red_ln!("No estoy seguro de qué distribución de Linux estás ejecutando.");
                    "linux" // Devuelve un valor por defecto
                }
            } else {
                // Si no se puede leer el archivo, ejecutar uname -a para verificar
                green_ln!("No se pudo leer /etc/os-release. Ejecutando uname -a...");

                let output = Command::new("uname")
                    .arg("-a")
                    .output()
                    .expect("Error al ejecutar uname");

                let output_str = String::from_utf8_lossy(&output.stdout);

                // Verificar si la salida contiene información que sugiera Android
                if output_str.contains("Android") {
                    "android"
                } else {
                    green_ln!(
                        "No se pudo determinar la distribución. Salida de uname: {}",
                        output_str
                    );
                    "linux" // Devuelve un valor por defecto
                }
            }
        }
        _ => {
            e_red_ln!("No estoy seguro de qué sistema operativo estás ejecutando.");
            "desconocido" // Devuelve un valor por defecto
        }
    }
}
