use std::io;

fn correct() {
    println!("Excelente")
}

fn incorrect() {
    println!("Incorrecto")
}

fn main() -> io::Result<()> {
    let mut puntaje: i8 = 20;
    let mut name = String::new();

    let stdin = io::stdin();

    let mut respuesta = String::new();

    println!("Hola, Bienvenido a mi proyecto.");
    println!("Te pondré a prueba con preguntas del estado de emergencia");

    println!("Ingresa tu nombre");
    stdin.read_line(&mut name).expect("fail");

    println!("Hola {} para responder las preguntas ingresa la letra de la alternativa correcta y das click a enter:", &name.trim());
    println!("Tienes {} puntos", puntaje);

    println!("1) ¿En qué horario se rige el toque de queda?");
    println!("a) 7:00 p.m. a 7:00 a.m.");
    println!("b) 8:00 p.m. a 4:00 a.m.");
    println!("c) 6:00 p.m. a 5:00 a.m.");
    println!("d) 4:00 p.m. a 7:00 a.m.");

    stdin.read_line(&mut respuesta).expect("fail");

    match respuesta.trim() {
        "b" => correct(),
        _ => {
            incorrect();
            puntaje = puntaje - 4;
        }
    }

    println!("2) ¿Donde se reporto el primer caso?");
    println!("a) España");
    println!("b) Estados Unidos");
    println!("c) China");
    println!("d) Rusia");

    let mut respuesta = String::new();
    stdin.read_line(&mut respuesta).expect("fail");

    match respuesta.trim() {
        "c" => correct(),
        _ => {
            incorrect();
            puntaje = puntaje - 4;
        }
    }

    println!("3) ¿Por qué todos debemos usar mascarilla?");
    println!("a) Reducir la tasa de contagio");
    println!("b) Para vernos bien");
    println!("c) Para no oler feos olores");
    println!("d) Todas las anteriores");

    let mut respuesta = String::new();
    stdin.read_line(&mut respuesta).expect("fail");

    match respuesta.trim() {
        "a" => correct(),
        "b" => {
            println!("Incorrecto la mascarilla no es un accesorio");
            puntaje = puntaje - 4;
        }
        "c" => {
            println!("Incorrecto la mascarilla no es capaz de filtrar olores");
            puntaje = puntaje - 4;
        }
        "d" => {
            println!("Incorrecto fijate algunos son inecesarios");
            puntaje = puntaje - 4;
        }
        _ => {
            incorrect();
            puntaje = puntaje - 4;
        }
    }

    println!("4) ¿Cuál es el número telefónico para registrar un caso sospechoso?");
    println!("a) 118");
    println!("b) 911");
    println!("c) 115");
    println!("d) 113");

    let mut respuesta = String::new();
    stdin.read_line(&mut respuesta).expect("fail");

    match respuesta.trim() {
        "d" => correct(),
        _ => {
            incorrect();
            puntaje = puntaje - 4
        }
    }

    println!("5) ¿En qué año inicio la pandemia?");
    println!("a) 2019");
    println!("b) 2020");
    println!("c) 2018");
    println!("d) 2017");

    let mut respuesta = String::new();
    stdin.read_line(&mut respuesta).expect("fail");

    match respuesta.trim() {
        "b" => correct(),
        _ => {
            incorrect();
            puntaje = puntaje - 4
        }
    }

    println!(
        "Gracias por participar {} en mi proyecto de programación :) Lograste alcanzar {} puntos",
        name.trim(),
        puntaje
    );
    Ok(())
}
