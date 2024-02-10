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
            "limpiar" => println!("Se solicitó des-inicializar el arreglo"),
            "help" => print_client_help(),
            "salir" => exit_flag = true,
            _ => println!("Comando no reconocido. Para ver comandos disponibles, ingresar 'help'"),
        }
        if exit_flag {
            break;
        }
    }
}

/**
 * Función invocada al momento de recibir un comando de asignación por parte del usuario.
 * Recibe como argumento el string raw del comando ingresado y retorna un objeto de tipo
 * Result que consiste en un valor Ok() en caso de que el comando haya sido invocado
 * correctamente y Err en caso contrario.
 */
fn parse_assignment_cmd(cmd: &str) -> Result<(u32, i32), &str>{
    let cmd_contents: Vec<&str> = cmd.split(' ').collect();

    if cmd_contents.len() != 3 {
        return Err("Número inválido de argumentos")
    }

    let pos: Result<u32, _> = cmd_contents[1].parse();
    let val: Result<i32, _> = cmd_contents[2].parse();

    if let Err(_) = pos {
        return Err("Input de posición inválido.");
    }
    
    if let Err(_) = val {
        return Err("Input de valor inválido.");
    }

    //Desenvolver valores despues de asegurar correctitud
    let pos = pos.unwrap();
    let val = val.unwrap();

    Ok((pos, val))
}

/**
 * Función invocada en el momento en que el usuraio solicita verificar la
 * inicialización de alguna posición del arreglo. Recibe como input el
 * raw string conteniendo el comando ingresado por el usuario y retorna
 * un valor de tipo Result que contiene la posición a consultar en caso
 * de que el comando sea correcto y un valor de error en caso contrario.
 */
fn parse_consult_cmd(cmd: &str) -> Result<u32, &str> {
    let cmd_contents: Vec<&str> = cmd.split(' ').collect();

    if cmd_contents.len() != 2 {
        return Err("número erróneo de argumentos");
    }

    let pos: Result<u32, _> = cmd_contents[1].parse();

    if let Err(_) = pos {
        return Err("valor ingresado no es número positivo");
    }

    Ok(pos.unwrap())

}

/**
 * Procedimiento que da la bienvenida al programa
 */
fn print_welcome() {
    println!("¡Bienvenido al simulador de inicialización virtual!");
    println!("Para ver comandos disponibles, ingresar 'help'");
}

/**
 * Procedimieto que muestra en pantalla el manual de uso del cliente.
 */
fn print_client_help() -> () {
    println!("COMANDOS DEL SIMULADOR:");
    println!("\ti. ASIGNAR <POS> <VAL>: Asignar el valor <VAL> a la posición <POS> del arreglo.");
    println!("\tii. CONSULTAR <POS>: Consultar si la posición <POS> del arreglo ya fue inicializada");
    println!("\tiii. LIMPIAR: Borra todos los contenidos de la tabla, efectivamente dejando ninguna posición sin inicializar.");
    println!("\tiv. help: Muestra en pantalla el uso y formato de los comandos.");
}