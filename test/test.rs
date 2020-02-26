enum Mensaje {
    Escribir(String),
    Leer,
    Ping,
    RoudTime
}

enum Option<T> {
    Some(T),
    None
}

fn main() {
    let mut mensaje = Mensaje::Escribir("Hola mundo".to_owned());
    mensaje = Mensaje::Leer;

    match mensaje {
        Mensaje::Escribir(msg) => unimplemented!(),
        Mensaje::Leer => unimplemented!(),
        Mensaje::Ping => unimplemented!(),
    }
}
