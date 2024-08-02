mod fecha{
    pub mod imprimir_fecha;
    pub mod leer_fecha;
}

mod os{
    pub mod comandos_os;
    pub mod verificacion_os;
}

use std::io;

fn main() {

    let mut input = String::new();

    println!("Desea Actualizar(s/n): ");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");

    // Convertir la entrada a un carácter único
    let mut input_char = input.trim().chars().next().expect("No se ingresó ningún caracter");

    // Comparar con caracteres individuales.
    while input_char != 's' && input_char != 'n' {
        println!("Ingrese 's' para sí o 'n' para no:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        input_char = input.trim().chars().next().expect("No se ingresó ningún caracter");
    }

    println!("Respuesta válida: {}", input_char);
    os::verificacion_os::verificacion_os();
    fecha::imprimir_fecha::imprimir_fecha();
}


