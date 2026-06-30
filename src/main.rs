use std::io::{self, stdin};

struct Task {
    id: u32,
    description: String,
}

fn main() {
    let mut tasks:Vec<Task> = Vec::new();

    loop {
        println!("Bienvenido.");

        println!("1)- Listar tareas pendientes");
        println!("2)- Crear tarea");
        println!("3)- Eliminar tarea");
        println!("0)- Salir");

        let option = loop {
            let mut option:String = String::new();
            println!("Ingresa la opción");
            io::stdin().read_line(&mut option).expect("Falló al ingresar opción");
            match option.trim().parse::<u8>() {
                Ok(number) => break number,
                Err(_) => {
                    println!("Se espera un número");
                    continue
                }
            };
        };

        match option {
            0 => break,
            1 => print_tasks(&tasks),
            2 => {
                let task = create_task(&tasks);
                tasks.push(task);
            },
            3 => delete_task(&mut tasks),
            _ => println!("Dato no valido")
        };
    }
}

fn print_tasks(tasks: &Vec<Task>) {
    if tasks.len() < 1 {
        println!("No hay tareas para mostrar");
    } else {
        for task in tasks {
            println!("{} - {}", task.id, task.description);
        }
    }
}

fn create_task(tasks: &Vec<Task>) -> Task {
    let mut id = tasks.len() as u32;
    id = id + 1;

    println!("{}", id);

    let description:String = loop {
        println!("Ingresá la descripción");
        let mut description = String::new();
        stdin().read_line(&mut description).expect("Error al ingresar la descripcion");

        let trimmed = description.trim().to_string();
        if description.is_empty() {
            continue;
        } else {
            break trimmed;
        }
    };

    return Task {
        id,
        description,
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    let id = valid_id();
    match tasks.iter().position(|task| task.id == id){
        Some(index) => {
            tasks.remove(index);
            println!("Tarea eliminada con éxito");
        },
        None => {
            println!("No se encontró la tarea");
        }
    }
}

fn valid_id() -> u32 {
    loop {
        println!("Ingresá el id");
        let mut id = String::new();
        io::stdin().read_line(&mut id).expect("Error al ingresar el id");
        match id.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Se espera un número válido");
                continue;
            }
        }
    }
}