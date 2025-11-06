use std::io::{self, Write};

fn main() {
    let sequencia = obter_sequencia(
    "Insira uma sequência de números reais, separando com vírgula (ex.: \"4, 2.0, 5, 3.14, 15, etc...\"): "
    );

    let soma_dos_pares = soma_dos_pares(sequencia);

    println!("A soma dos pares é {soma_dos_pares}.");

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
                    println!("Sequência inválida, insira uma sequência correta.\n");
                    continue 'externo;
                }
            }
        }

        break;
    }
    
    sequencia

}


fn soma_dos_pares(sequencia: Vec<f64>) -> u32 {
    let mut soma: u32 = 0;

    for elemento in sequencia {
        if elemento % 2.0 == 0.0 {
            soma +=  elemento as u32;
        }
    }

    soma
}