use std::io;

fn calcular_media(nota1: f64, nota2: f64, nota3: f64) -> f64 {
    let npt = (nota1 + nota2) / 2.0;
    (npt * 0.7) + (nota3 * 0.3)
}

fn main() {
    let mut entrada = String::new();

    println!("Digite a nota da NP1:");
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a nota");
    let nota1: f64 = entrada.trim().parse().expect("Digite uma nota valida");

    entrada.clear();
    println!("Digite a nota da NP2:");
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a nota");
    let nota2: f64 = entrada.trim().parse().expect("Digite uma nota valida");

    entrada.clear();
    println!("Digite a nota da NPL:");
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a nota");
    let nota3: f64 = entrada.trim().parse().expect("Digite uma nota valida");

    let npt = (nota1 + nota2) / 2.0;
    let media_final = calcular_media(nota1, nota2, nota3);

    println!("NPT: {:.2}", npt);
    println!("Media final: {:.2}", media_final);

    if media_final >= 60.0 {
        println!("Parabens! Voce foi aprovado.");
    } else {
        println!("Reprovado.");
    }
}