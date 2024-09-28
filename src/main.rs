
mod fecha{
    pub mod imprimir_fecha;
    pub mod leer_fecha;
}

mod os{
    pub mod comandos_os;
    pub mod verificacion_os;
}

use std::io::stdin;
use crate::fecha::imprimir_fecha::imprimir_fecha;
use crate::fecha::leer_fecha::leer_fecha;
use crate::os::comandos_os::comandos_os;

fn main() {

    let mut input = String::new();
    let mut input_char;

    leer_fecha().expect("error al leer fecha"); //llama al archivo leer_fecha.rs

    loop {
        println!("Desea Actualizar(s/n): ");

        match stdin().read_line(&mut input){
            Ok(_) => {
                input_char = input.trim().chars().next().expect("No se ingresó ningún caracter");

                // validacion de caracter
                if input_char == 's' || input_char == 'n' {
                    println!("Respuesta válida: {}", input_char);
                    if input_char == 's' {
                        comandos_os(); //llama al archivo comando_os.rs si la entrada es 's'
                        imprimir_fecha().unwrap(); //llama al archivo imprimir_fecha.rs para indicar la fecha de la actualizacion
                        break;
                    }else {
                        break; //Salir del bucle si la entrada es 'n'
                    }
                } else {
                    println!("Entrada no válida. Ingrese 's' para sí o 'n' para no.");
                }
            }
            Err(error) => {
                eprintln!("Error al leer la entrada: {}", error);
                return;
            }
        }
        input.clear(); //limpiar la entrada antes de la proxima iteracion
    }
}