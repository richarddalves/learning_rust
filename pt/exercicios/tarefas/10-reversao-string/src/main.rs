use stdin_helper::get_string;

fn main() {
    let string_a_reverter = get_string("Insira um texto: ");
    let revertido = reverter(&string_a_reverter);

    println!("Revertido: {revertido}");
}

fn reverter(string_a_reverter: &String) -> String {

    let string_revertida = {
        let mut temp_string: String = String::new();

        for letra in string_a_reverter.chars().rev() {
            temp_string.push(letra);
        }

        temp_string
    };

    string_revertida
}
