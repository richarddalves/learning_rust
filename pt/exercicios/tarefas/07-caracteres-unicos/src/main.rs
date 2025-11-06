use std::char;

fn main() {
    let string_de_teste = "Oii";

    if tem_caracteres_unicos(string_de_teste) {
        println!("A string possui todos os caracteres únicos.");
    } else {
        println!("A string não possui todos os caracteres únicos.");
    }
}

fn tem_caracteres_unicos(input: &str) -> bool {
    let mut caracteres_inseridos: Vec<char> = Vec::new();

    for char in input.chars() {
        if caracteres_inseridos.contains(&char) {
            return false;
        } else {
            caracteres_inseridos.push(char);
        }
    }

    true
}
