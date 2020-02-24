fn main() {
    println!("Hello, world!");
    let x: [u64; 6] = [1, 2, 3, 4, 5, 6];
    let y: &[u64] = &x[1..4];
    for i in y {
        let alumn = Alumno::new("Pepe".to_owned(), *i);
        println!(
            "{} se graduará en {} (si todo sale bien)",
            alumn.nombre,
            alumn.graduation_year(2020)
        );
    }
}

struct Alumno {
    nombre: String,
    years_til_grad: u64,
}

impl Alumno {
    fn new(name: String, years: u64) -> Alumno {
        Alumno {
            nombre: name,
            years_til_grad: years,
        }
    }

    fn graduation_year(&self, current_year: u64) -> u64 {
        current_year + self.years_til_grad
    }
}
