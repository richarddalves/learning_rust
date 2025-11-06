use stdin_helper::*;

fn main() {
    let string_a_comprimir: String = get_string("Insira o texto a ser comprimido: ");
    let string_comprimida: String = comprimir(string_a_comprimir);

    println!("Texto comprimido: {string_comprimida}");

}

fn comprimir(string_a_comprimir: String) -> String {

    let len = string_a_comprimir.len();
    let mut nova_string = String::new();

    let mut qtd_repeticao = 0;
    let mut index_fim_repeticao = 0;
    
    // Verificar se o caractere seguinte é igual ao atual e até onde vai
    for (i, char) in string_a_comprimir.char_indices() {

        if index_fim_repeticao > 0 && i <= index_fim_repeticao {
            continue;
        }

        for j in (i + 1)..len {
            if string_a_comprimir.chars().nth(j).unwrap() == char {
                qtd_repeticao += 1;
                index_fim_repeticao = j;
            } else {
                break;
            }
        }

        nova_string.push(char);
        if qtd_repeticao > 0 {
            nova_string.push_str(format!("{}", qtd_repeticao + 1).as_str());
        }

        qtd_repeticao = 0;
    }

    nova_string
}