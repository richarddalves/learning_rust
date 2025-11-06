use stdin_helper::*;

fn main() {
    let numero = get_u32("Digite um número: ");
    println!("O número {numero} {} primo!", if is_primo(numero) {"é"} else {"não é"});
}

fn is_primo(num: u32) -> bool {

    if num <= 1 {
        return false;
    }

    let limite = (num as f64).sqrt() as u32;

    for divisor in 2..=limite {
        if num % divisor == 0 {
            println!("Divisível por {divisor}.");
            return false;
        }
    }


    true
}