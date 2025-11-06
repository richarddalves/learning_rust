use stdin_helper::*;

fn main() {
    let s1 = get_string("Digite a primeira string: ");
    let s2 = get_string("Digite a segunda string: ");

    println!("{}", is_permutacao(&s1, &s2));
}

fn is_permutacao(s1: &str, s2: &str) -> bool {

    if s1.len() != s2.len() {
        return false;
    }

    for char in s1.to_lowercase().chars() {
        if !s2.to_lowercase().contains(char) {
            return false;
        }
    }

    true
}
