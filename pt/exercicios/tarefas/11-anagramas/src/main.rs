use stdin_helper::get_string;

fn main() {
    let texto_1 = get_string("Insira o texto 1: ").to_lowercase();
    let texto_2 = get_string("Insira o texto 2: ").to_lowercase();

    let is_anagram = is_anagram(&texto_1, &texto_2);

    println!("É anagrama? {is_anagram}");
}

fn is_anagram(texto_1: &String, texto_2: &String) -> bool {

    let len1 = texto_1.len();
    let len2 = texto_2.len();

    let mut vec1: Vec<char> = texto_1.chars().collect();
    let mut vec2: Vec<char> = texto_2.chars().collect();

    if len1 != len2 {
        return false;
    }

    vec1.sort_unstable();
    vec2.sort_unstable();

    if vec1 != vec2 {
        return false;
    }


    true
}