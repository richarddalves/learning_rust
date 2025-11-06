use std::io::{self, Write};

fn main() {
    let lista_notas = obter_sequencia("Insira as notas, separando com vírgula: ");
    let media_notas = media_notas(lista_notas);

    println!("A média das notas é: {media_notas}.");
}

fn obter_sequencia(msg: &str) -> Vec<f64> {
    let mut input_buffer = String::new();
    let mut sequencia: Vec<f64> = Vec::new();

    'externo: loop {
        input_buffer.clear();

        print!("{msg}");
        io::stdout().flush().expect("Erro ao fazer flush");

        io::stdin().read_line(&mut input_buffer).expect("Erro ao ler entrada");

        input_buffer = input_buffer.replace(" ", "").trim().to_string();

        let lista_string: Vec<&str> = input_buffer.split(",").collect();

        for elemento in lista_string {
            match elemento.parse::<f64>() {
                Ok(numero) => {
                    sequencia.push(numero);
                },

                _ => {
                    println!("Sequência inválida, {elemento} não é um elemento válido.");
                    println!("Insira uma sequência correta.\n");
                    continue 'externo;
                }
            }
        }

        break;
    }
    
    sequencia

}


fn media_notas (lista_notas: Vec<f64>) -> f64 {
    let num_notas = lista_notas.len();
    let mut soma = 0.0;

    for nota in lista_notas {
        soma += nota;
    }

    soma/num_notas as f64
}