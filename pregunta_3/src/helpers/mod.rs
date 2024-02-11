/**
 * Función invocada al momento de recibir un comando de asignación por parte del usuario.
 * Recibe como argumento el string raw del comando ingresado y retorna un objeto de tipo
 * Result que consiste en un valor Ok() en caso de que el comando haya sido invocado
 * correctamente y Err en caso contrario.
 */
pub fn parse_assignment_cmd(cmd: &str) -> Result<(u32, i32), &str>{
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
pub fn parse_consult_cmd(cmd: &str) -> Result<u32, &str> {
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
pub fn print_welcome() {
    println!("¡Bienvenido al simulador de inicialización virtual!");
    println!("Para ver comandos disponibles, ingresar 'help'");
}

/**
 * Procedimieto que muestra en pantalla el manual de uso del cliente.
 */
pub fn print_client_help() -> () {
    println!("COMANDOS DEL SIMULADOR:");
    println!("\ti. ASIGNAR <POS> <VAL>: Asignar el valor <VAL> a la posición <POS> del arreglo.");
    println!("\tii. CONSULTAR <POS>: Consultar si la posición <POS> del arreglo ya fue inicializada");
    println!("\tiii. LIMPIAR: Borra todos los contenidos de la tabla, efectivamente dejando ninguna posición sin inicializar.");
    println!("\tiv. help: Muestra en pantalla el uso y formato de los comandos.");
}