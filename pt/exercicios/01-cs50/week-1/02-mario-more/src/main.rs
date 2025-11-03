//! Programa que imprime uma pirâmide com `n` linhas indicadas pelo usuário

use std::io::{self, Write};

/*

linha 1: 4 espaços, 1 #
linha 2: 3 espaços, 2 #
linha 3: 2 espaços, 3 #
linha 4: 1 espaço,  4 #
linha 5: 0 espaços, 5 #

    #  #
   ##  ##
  ###  ###
 ####  ####
#####  #####

*/

fn main() {
    let altura = pedir_u8("Altura: ");

    for linha_atual in 1 ..= altura {

        let qtd_espacos = altura - linha_atual;
        
        imprimir_linha(qtd_espacos, linha_atual);
        
    }

}

/// Pede por um u8 até o usuário indicar algum u8 correto.
fn pedir_u8(msg: &str) -> u8 {

    let mut input_buffer = String::new();

    loop {
        // ======== Obter input ========
        print!("{msg}");
        io::stdout().flush().expect("Erro ao fazer flush");
        
        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).expect("Erro ao ler entrada");

        // ======== Avaliar input ========
        match input_buffer.trim().parse::<u8>() {
            Ok(valor) => return valor,

            Err(_) => {
                continue;
            }
        }

    }

}

/// Função para imprimir cada linha da pirâmide
/// Usa while para primeiro ir imprimindo os espaços e depois,
/// Usa for para ir imprimindo os tijolos
fn imprimir_linha(mut qtd_espacos: u8, qtd_tijolos: u8) {

    // Utilizei 3 tipos diferentes de loops apenas para praticar cada um

    // Imprime n vezes conforme vai decrementando
    while qtd_espacos != 0 {
            print!(" ");
            qtd_espacos -= 1;
    }

    // Imprime n vezes
    for _ in 1..=qtd_tijolos {
        print!("#"); 
    }

    // Espaço entre os dois lados.
    print!("  ");

    // Utilizando método idiomático do Rust
    print!("{}", "#".repeat(qtd_tijolos as usize));
    // print!("{}", " ".repeat(qtd_espacos as usize)); // Os espaços aqui são desnecessários.

    println!();
}