
mod fecha{
    pub mod imprimir_fecha;
    pub mod leer_fecha;
}

mod os{
    pub mod comandos_os;
    pub mod verificacion_os;
}

use std::io::stdin;

fn main() {

    let mut input = String::new();
    let mut input_char;

    loop {
        println!("Desea Actualizar(s/n): ");

        match stdin().read_line(&mut input){
            Ok(_) => {
                input_char = input.trim().chars().next().expect("No se ingresó ningún caracter");

                if input_char == 's' || input_char == 'n' {
                    println!("Respuesta válida: {}", input_char);
                    os::verificacion_os::verificacion_os();
                    fecha::imprimir_fecha::imprimir_fecha();
                    break; // Salir del bucle si la entrada es válida
                } else {
                    println!("Entrada no válida. Ingrese 's' para sí o 'n' para no.");
                }
            }
            Err(error) => {
                eprintln!("Error al leer la entrada: {}", error);
                return;
            }
        }
        input.clear();
    }
}


