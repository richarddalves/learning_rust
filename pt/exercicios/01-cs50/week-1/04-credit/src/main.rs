use stdin_helper;

fn main() {
    let cartao_inserido = stdin_helper::get_number("Insira o número de um cartão: ");

    let bandeira_cartao = algoritmo_luhns(cartao_inserido);

    println!("Seu cartão é {bandeira_cartao}!");
    
}

fn obter_digitos(mut num: u64) -> Vec<u64> {
    let mut lista_de_digitos: Vec<u64> = Vec::new();

    while num % 10 != num {
        lista_de_digitos.push(num % 10);
        num /= 10;
    }

    lista_de_digitos.push(num);

    lista_de_digitos

}

fn algoritmo_luhns(cartao: u64) -> &'static str {


    // Transforma o cartão inserido em uma lista de digitos
    let lista_de_digitos: Vec<u64> = obter_digitos(cartao);

    let mut digitos_para_soma: Vec<u64> = Vec::new();

    // ===== Primeiro passo =====
    // Primeiro obtemos digito-nao, digito-sim multiplocados por 2
    for (i, digito) in lista_de_digitos.iter().enumerate() {
        if i % 2 != 0 {
            digitos_para_soma.push(*digito * 2);
        }
    }

    // Depois, somamos os digitos de cada um desses digitos da lista
    let mut soma_primeiro_passo = 0;
    for digito in digitos_para_soma {
        if digito % 10 != digito {
            soma_primeiro_passo += digito % 10;
            soma_primeiro_passo += digito / 10;
        } else {
            soma_primeiro_passo += digito;
        }
    }

    // ===== Segundo passo =====
    // Somamos esse total do primeiro passo com os digitos que não foram multiplicdos por 2 inicialmente
    for (i, digito) in lista_de_digitos.iter().enumerate() {
        if i % 2 == 0 {
            soma_primeiro_passo += digito;
        }
    }


    // ===== Terceiro passo =====
    // Se o último digito dessa soma for 0, o número é legitimo
    let is_cartao_valido = soma_primeiro_passo % 10 == 0;

    if is_cartao_valido {
        match lista_de_digitos.len() {
            15 => return "AMEX",
            13 => return "VISA",
            16 => {
                if *lista_de_digitos.last().unwrap() == 4 {
                    return "VISA";
                } else {
                    return "MASTERCARD";
                }
            }

            _ => return "INVALIDO"

        }
    } else {
        return "INVALIDO";
    }

}
