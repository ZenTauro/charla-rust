enum Mensaje {
    Escribir(String),
    Leer,
    Ping,
    RoudTime
}

enum Option<T> {
    Ok(T),
    None
}

fn main() {
    let mut mensaje = Mensaje::Escribir("Hola mundo");
    mensaje = Mensaje::Leer;

    match mensaje {
        Mensaje::Escribir(msg) => ...,
        Mensaje::Leer => ...,
        Mensaje::Ping => pong(),
    }
}
