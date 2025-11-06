use std::io::{self, Write};

fn main() {
    let vetor_numeros = obter_numeros();
    let maior = maior_numero(vetor_numeros);

    println!("O maior número é {maior}.");
}

fn obter_numeros() -> Vec<i32> {
    let mut input_buffer = String::new();
    let mut contador  = 1;

    let mut numeros: Vec<i32> = Vec::new();

    println!("Insira os números do vetor. Para parar, envie um ponto (.)");

    loop {
        input_buffer.clear();

        print!("Insira o {contador}º número: ");
        io::stdout().flush().expect("Erro ao fazer flush");

        io::stdin().read_line(&mut input_buffer).expect("Erro ao ler entrada");

        if input_buffer.trim() == "." {break}

        match input_buffer.trim().parse::<i32>() {
            Ok(valor) => {
                numeros.push(valor);
                contador += 1;
            },

            _ => continue
        }
    }

    numeros
}

fn maior_numero(vetor_numeros: Vec<i32>) -> i32 {
    let mut maior_ate_agora = *vetor_numeros.first().expect("O vetor não tem elementos.");

    for num in vetor_numeros {
        if num > maior_ate_agora {
            maior_ate_agora = num;
        }
    }

    maior_ate_agora
}