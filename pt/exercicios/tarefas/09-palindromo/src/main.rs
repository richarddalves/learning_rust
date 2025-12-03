use stdin_helper::get_i32;

fn main() {
    let num = get_i32("Insira um número: ");
    println!("{}", is_palindromo(num));
}

fn is_palindromo(mut num: i32) -> bool {

    if num < 0 {
        return false;
    }

    let lista_digitos: Vec<i32> = {
        let mut lista = Vec::<i32>::new();
        while num != 0 {
            lista.push(num % 10);
            num /= 10;
        }
        lista
    };


    lista_digitos.iter().eq(lista_digitos.iter().rev())
}
