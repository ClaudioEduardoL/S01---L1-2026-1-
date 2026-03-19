use std::io;

fn eh_impar(numero: i32) -> bool {
    numero % 2 != 0
}

fn main() {
    let mut escolha = String::new();
    let numero1: i32;
    let numero2: i32;

    println!("Jogador 1, escolha par ou impar:");
    io::stdin()
        .read_line(&mut escolha)
        .expect("Erro ao ler a escolha");

    let escolha = escolha.trim().to_lowercase();

    let mut entrada1 = String::new();
    println!("Jogador 1, digite seu numero:");
    io::stdin()
        .read_line(&mut entrada1)
        .expect("Erro ao ler o numero");
    numero1 = entrada1.trim().parse().expect("Digite um numero valido");

    let mut entrada2 = String::new();
    println!("Jogador 2, digite seu numero:");
    io::stdin()
        .read_line(&mut entrada2)
        .expect("Erro ao ler o numero");
    numero2 = entrada2.trim().parse().expect("Digite um numero valido");

    let soma = numero1 + numero2;
    let soma_e_impar = eh_impar(soma);

    if (escolha == "impar" && soma_e_impar) || (escolha == "par" && !soma_e_impar) {
        println!("Soma dos numeros: {}", soma);
        println!("Vencedor: Jogador 1");
    } else {
        println!("Soma dos numeros: {}", soma);
        println!("Vencedor: Jogador 2");
    }
}