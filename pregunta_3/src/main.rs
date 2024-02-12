pub mod helpers;
use crate::helpers::{print_welcome, print_client_help, parse_assignment_cmd, parse_consult_cmd};
use std::io::Write;

/**
 * Implementación en Rust (btw) de un simulador de inicialización virtual en tiempo constante
 * mediante uso de tablas auxiliares. Preungta 3 de la cuarta tarea de Diseño de Algoritmos I. 
 * Trimestre Ene-Mar 2024 (Prof. Ricardo Monascal).
 * 
 * Autor: Santiago Finamore
 * Carnet: 18-10125
 */
fn main() {
    //Extraer tamaño del arreglo a manejar de los argumentos de consola
    let cmdline_args: Vec<String> = std::env::args().collect();

    if cmdline_args.len() != 2 {
        println!("Número erróneo de argumentos.");
        std::process::exit(1);
    }

    let array_size: i32 = cmdline_args[1].parse().unwrap_or(-1);
    if array_size <= 0 {
        println!("El argumento ingresado es incorrecto.");
        println!("Recordar que el cliente se inicia con un único argumento de consola correspondiente al tamaño del arreglo a manejar");
        std::process::exit(1);
    }

    //Declaración de variables
    let mut exit_flag: bool = false;
    let mut ctr: u32 = 0;
    /* Necesario usar vectores pues Rust no permite la declaración de arreglos cuya
    longitud no sea conocida en tiempo de compilación */
    let mut a: Vec<u32> = Vec::with_capacity(array_size as usize);
    let mut b: Vec<u32> = Vec::with_capacity(array_size as usize);
    let mut t: Vec<i32> = Vec::with_capacity(array_size as usize);

    /* Bloque 'unsafe' requerido para poder extender longitud de vectores
    sin tener que inicializar sus valores.*/
    unsafe {
        a.set_len(array_size as usize);
        b.set_len(array_size as usize);
        t.set_len(array_size as usize);
    }

    //Dar bienvenida al usuario y comenzar ejecución
    print_welcome();
    loop {
        let mut user_input = String::new();

        print!("Ingrese el comando a ejecutar: ");
        std::io::stdout()
            .flush()
            .expect("Ocurrió un error al momento de hacer 'flush' de la salida estándar. Abortando.");

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Ocurrión un error leyendo el input estándar");
        user_input.make_ascii_lowercase();

        match &user_input[..user_input.len()-1] {
            cmd if cmd.starts_with("asignar ") => {
                let assign_parse = parse_assignment_cmd(cmd);
                match assign_parse {
                    Ok((pos, val)) => {
                        if pos >= array_size as u32 {
                            println!("La posición ingresada está fuera del rango del arreglo");
                        } else {
                            ctr += 1;
                            a[ctr as usize] = pos;
                            b[pos as usize] = ctr;
                            t[pos as usize] = val;
                            println!("Se inicializó la posición {} con el valor {}.", pos, val)
                        }
                    }
                    Err(_) => {
                        println!("El comando de asignación ingresado no es correcto.");
                        println!("Para ver uso de comandos, ver 'help'");
                    }
                }
            }
            cmd if cmd.starts_with("consultar ") => {
                let consult_parse = parse_consult_cmd(cmd);
                match consult_parse {
                    Ok(pos) => {
                        if pos >= array_size as u32 {
                            println!("La posición ingresada está fuera del rango del arreglo");
                        } else {
                            //as usize as usize as usize as usize as usize
                            let init_check: bool = (1 <= b[pos as usize] && b[pos as usize] <= ctr) && (a[b[pos as usize] as usize] == pos);
                            if init_check {
                                println!("La posición {} del arreglo si ha sido inicializada.", pos);
                            } else {
                                println!("La posición {} del arreglo no ha sido inicializada.", pos);
                            }
                        }
                    }
                    Err(_) => {
                        println!("El comando de consulta ingresado no es correcto.");
                        println!("Para ver uso de comandos, ver 'help'");
                    }
                }
            }
            "limpiar" => {
                a = a.iter().map(|_| {0}).collect();
                b = b.iter().map(|_| {0}).collect();
                t = t.iter().map(|_| {-1}).collect();
            },
            "help" => print_client_help(),
            "salir" => exit_flag = true,
            _ => println!("Comando no reconocido. Para ver comandos disponibles, ingresar 'help'"),
        }
        if exit_flag {
            break;
        }
    }
}
