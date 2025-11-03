//! Calcula o fatorial de um número

/// Crate auxiliar para obter entradas do usuário pelo terminal
use stdin_helper::{self, get_number};

fn main() {
    let fatorando = get_number("Insira o número a ser fatorado: ");

    let produto = fatorial(fatorando);

    println!("O fatorial de {fatorando} é {produto}.");
}

/// Função que retorna um fatorial de um numero
fn fatorial(fatorando: u128) -> u128 {

    let mut produto = 1;

    if fatorando <= 1 {
        return produto;
    }

    for antecessor in 2..=fatorando {
        produto *= antecessor;
    }

    produto
}
