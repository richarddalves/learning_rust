use std::io::{self, Write};

const COINS: [u32; 4] = [25, 10, 5, 1];

fn main() {

    // Pergunta quanto o usuário deve, em centavos
    let change_owed = get_u32("Troco devido (em centavos): ");
    let coins_needed = get_coins_amount(change_owed);

    println!("Você deve devolver {}.",
        if coins_needed == 1 {
            "1 moeda".to_string()
        } else {
            format!("{coins_needed} moedas")
        }
    );
    
}

fn get_u32(msg: &str) -> u32 {

    let mut input_buffer = String::new();

    loop {
        input_buffer.clear();

        print!("{msg}");
        io::stdout().flush().expect("Erro ao fazer flush");

        io::stdin().read_line(&mut input_buffer).expect("Erro ao ler entrada.");

        match input_buffer.trim().parse::<u32>() {
            Ok(value) => if value != 0 {return value;} else {continue;}

            _ => continue
        }
    }
}

fn get_coins_amount(mut change_owed: u32) -> u32 {
    let mut coins_needed: u32 = 0;

    for coin in COINS {

         loop {

             if change_owed as i32 - coin as i32 >= 0 {
                change_owed -= coin;
                coins_needed += 1;
             } else {
                break;
             }

         }

     }

     coins_needed
}