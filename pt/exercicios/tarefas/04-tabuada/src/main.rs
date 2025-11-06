use stdin_helper::{self, get_u32};

fn main() {
    let multiplicando = get_u32("Insira o multiplicando: ");
    exibir_tabuada(multiplicando);
}

fn exibir_tabuada(multiplicando: u32) {
    for multiplicador in 1..=12 {
        let produto = multiplicando * multiplicador;
        
        println!("{multiplicando} x {multiplicador} = {produto}");
    }
}
