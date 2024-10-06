mod fecha {
    pub mod imprimir_fecha;
    pub mod leer_fecha;
}

mod os {
    pub mod comandos_os;
    pub mod verificacion_os;
}

use crate::fecha::imprimir_fecha::imprimir_fecha;
use crate::fecha::leer_fecha::leer_fecha;
use crate::os::comandos_os::comandos_os;
use colour::{blue_ln, dark_magenta_ln, e_red_ln, green_ln, red_ln};
use std::io::stdin;

fn main() {
    let mut input = String::new();
    let mut input_char;

    leer_fecha().expect("error al leer fecha"); //llama al archivo leer_fecha.rs

    loop {
        dark_magenta_ln!("Desea Actualizar(s/n): ");

        match stdin().read_line(&mut input) {
            Ok(_) => {
                input_char = input
                    .trim()
                    .chars()
                    .next()
                    .expect("No se ingresó ningún caracter");

                // validacion de caracter
                if input_char == 's' || input_char == 'n' {
                    //println!("Respuesta válida: {}", input_char);
                    if input_char == 's' {
                        green_ln!("Se Actualizara");
                        comandos_os(); //llama al archivo comando_os.rs si la entrada es 's'
                        imprimir_fecha().unwrap(); //llama al archivo imprimir_fecha.rs para indicar la fecha de la actualizacion
                        break;
                    } else {
                        blue_ln!("No se actualizara");
                        break; //Salir del bucle si la entrada es 'n'
                    }
                } else {
                    red_ln!("Entrada no válida. Ingrese 's' para sí o 'n' para no.");
                }
            }
            Err(error) => {
                e_red_ln!("Error al leer la entrada: {}", error);
                return;
            }
        }
        input.clear(); //limpiar la entrada antes de la proxima iteracion
    }
}
