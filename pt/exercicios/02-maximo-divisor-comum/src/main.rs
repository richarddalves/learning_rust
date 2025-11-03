//! Enunciado:
//! ​Dado duas variáveis a = 15 e b = 40 , faça um programa para determinar o máximo divisor comum entre eles

use std::{io::{self, Write}};

fn main() {
    let a = pedir_u32("Insira o valor a: ");
    let b = pedir_u32("Insira o valor b: ");

    let mdc_manual = mdc(a, b);
    let mdc_euclidiano: u32 = mdc_euclides(a, b);

    println!("\nO máximo divisor comum entre {a} e {b} é {mdc_manual}.");
    println!("Segundo o algoritmo euclidiano é: {mdc_euclidiano}.");

}

/// Lê entrada do usuário e converte para u32
fn pedir_u32(msg: &str) -> u32 {
    let mut input_buffer = String::new();

    print!("{msg}");
    io::stdout().flush().expect("Erro ao fazer flush");

    io::stdin().read_line(&mut input_buffer).expect("Erro ao ler entrada");
    
    input_buffer.trim().parse::<u32>().expect("Erro ao converter entrada para número")
}

/// Retorna o **máximo divisor comum** entre `a` e `b`
fn mdc(a: u32, b: u32) -> u32 {

    let mut fatores_em_comum: Vec<u32> = Vec::new();
    let mut mdc = 1;

    if a == 0 {
        return b;
    }

    if b == 0 {
        return a;
    }

    let primos_a = decompor_em_fatores_primos(a);
    let mut primos_b = decompor_em_fatores_primos(b);

    for fator_a in primos_a {
        match primos_b.binary_search(&fator_a) {

            Ok(index_b) => {
                fatores_em_comum.push(fator_a);
                primos_b.remove(index_b);
            }

            _ => { }
        }
    }

    for fator in fatores_em_comum {
        mdc *= fator;
    }

    mdc
    
}

/// Retorna o mdc entre `a` e `b` utilizando o algoritimo de euclides
/// 
/// ## O Algoritmo
/// 
/// O Algoritmo Euclidiano para encontrar o MDC(A,B) é dado por:
/// - Se A = 0, então MDC(A,B)=B, uma vez que MDC(0,B)=B, e podemos parar a verificação. 
/// - Se B = 0, então MDC(A,B)=A, uma vez que o MDC(A,0)=A, e podemos parar a verificação. 
/// - Escreva A na forma do resto do quociente (A = B⋅Q + R)
/// - Encontre o MDC(B,R) usando o Algoritmo Euclidiano, já que MDC(A,B) = MDC(B,R)
fn mdc_euclides(a: u32, b: u32) -> u32 {
    if a == 0 {
        return b;
    }

    if b == 0 {
        return a;
    }

    // a = b * quociente + resto
    // agora ao invés de descobrir o mdc(a, b), só precisamos descobrir o mdc entre (b, resto)
    let _quociente = a / b;
    let resto = a % b;

    mdc_euclides(b, resto)
}

/// Decompõe um número em fatores primos e retorna um vector com eles
/// 
/// ## Exemplos para fins de lógica
/// ```code
/// decompor_em_fatores_primos(8) = [2, 2, 2]
/// decompor_em_fatores_primos(9) = [3, 3]
/// decompor_em_fatores_primos(12) = [2, 2, 3]
/// decompor_em_fatores_primos(22) = [2, 11]
/// ```
fn decompor_em_fatores_primos(mut num: u32) -> Vec<u32> {

    
    let numeros_primos = primos_ate(num); // Ex.: primos até 12: [2, 3, 5, 7, 11]
    let mut fatores_primos = Vec::new();

    for primo in numeros_primos {
            while num % primo == 0 {
            num /= primo;
            fatores_primos.push(primo);
        }
    }

    fatores_primos
}

/// Encontra os números primos até o número n
fn primos_ate(n: u32) -> Vec<u32> {
    let mut primos = Vec::new();

    for num in 2..=n {
        if is_primo(num) {
            primos.push(num);
        }
    }

    primos
}

/// Diz se um número é primo ou não
fn is_primo(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let limite = (num as f64).sqrt() as u32;

    for divisor in 2..=limite {
        if num % divisor == 0 {
            return false;
        }
    }

    true
}