use std::io;

fn imprimir_multiplos(numero: i32, limite_inferior: i32, limite_superior: i32) {
    for i in limite_inferior..=limite_superior {
        if i % numero == 0 {
            println!("{}", i);
        }
    }
}

fn main() {
    let mut entrada = String::new();

    println!("Digite o numero da tabuada:");
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler o numero");
    let numero: i32 = entrada.trim().parse().expect("Digite um numero valido");

    entrada.clear();
    println!("Digite o limite inferior:");
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler o limite inferior");
    let limite_inferior: i32 = entrada.trim().parse().expect("Digite um numero valido");

    entrada.clear();
    println!("Digite o limite superior:");
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler o limite superior");
    let limite_superior: i32 = entrada.trim().parse().expect("Digite um numero valido");

    imprimir_multiplos(numero, limite_inferior, limite_superior);
}