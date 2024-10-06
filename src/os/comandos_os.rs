use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use colour::{blue_ln, e_red_ln, green_ln};
use crate::os::verificacion_os::verificacion_os;

pub(crate) fn comandos_os() {
    let sistema = verificacion_os();
    blue_ln!("Distribucion: {}", verificacion_os());
    green_ln!("Actualizando.......");

    match sistema {
        "android" => {
            execute_command("pkg", &["update"]);
            execute_command("pkg", &["upgrade", "-y"]);
            execute_command("pkg", &["remove", "-y"]);
            execute_command("pkg", &["autoclean"]);
            execute_command("pkg", &["clean"]);
        },
        "ubuntu" | "debian" => {
            execute_command("sudo", &["apt", "update"]);
            execute_command("sudo", &["apt", "upgrade", "-y"]);
            execute_command("sudo", &["apt", "autoremove", "-y"]);
            execute_command("sudo", &["apt", "remove", "-y"]);
            execute_command("sudo", &["apt", "autoclean"]);
            execute_command("sudo", &["apt", "clean"]);
        },
        "fedora" => {
            execute_command("sudo", &["dnf", "upgrade", "--refresh"]);
            execute_command("sudo", &["dnf", "install", "dnf-plugin-system-upgrade"]);
            execute_command("sudo", &["dnf", "autoremove", "-y"]);
            execute_command("sudo", &["dnf", "clean", "all"]);
        },
        _ => {
            e_red_ln!("error: sistema operativo no soportado");
        }
    }
}

fn execute_command(command: &str, args: &[&str]) {
    let process = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match process {
        Ok(mut child) => {
            let stdout = child.stdout.take().expect("No se pudo obtener stdout");
            let stderr = child.stderr.take().expect("No se pudo obtener stderr");

            // Leer stdout
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(e) => e_red_ln!("Error leyendo stdout: {}", e),
                }
            }

            // Leer stderr
            let reader_err = BufReader::new(stderr);
            for line in reader_err.lines() {
                match line {
                    Ok(line) => eprintln!("{}", line),
                    Err(e) => e_red_ln!("Error leyendo stderr: {}", e),
                }
            }

            // Esperar a que el proceso termine
            let _ = child.wait().expect("El proceso no pudo terminar");
        },
        Err(e) => {
            e_red_ln!("Error al ejecutar {}: {}", command, e);
        }
    }

    sleep(Duration::from_secs(2)); // Espera 2 segundos
}