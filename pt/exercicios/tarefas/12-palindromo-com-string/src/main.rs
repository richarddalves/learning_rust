use stdin_helper::get_string;

fn main() {
    let texto = get_string("Insira um texto: ");
    let is_palindromo = is_palindromo(&texto);

    println!("{}", is_palindromo);
}

fn is_palindromo(texto: &String) -> bool {

    // shadowing para considerar o texto apenas em lowercase, mas sem mudar realmente
    let mut texto = texto.to_lowercase();

    // remove todos os espaços e pontuação
    texto = texto.replace(" ", "").replace(".", "").replace(",", "").replace("!", "");

    // Verifica se é um palindromo
    texto.chars().eq(texto.chars().rev())
}