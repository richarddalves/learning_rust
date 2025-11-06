fn main() {
    count(10);
    count_down(10);
}

fn count(num: u32) {
    for n in 1..=num {
        print!("{n} ");
    }
    
    println!();
}

fn count_down(mut num: u32) {
    while num >= 1 {
        print!("{num} ");
        num -= 1;
    }

    println!()
}