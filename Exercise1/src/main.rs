use std::io;

fn get_int() -> i32 {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        panic!("Console input error: {e}");
    }

    let input = match input[..input.len() - 2].parse::<i32>() {
        Ok(t) => t,
        Err(..) => get_int(),
    };
    input
}

fn main() {
    println!("Введите номер места:");
    let location = get_int();
    let answer = location / 4 + if location % 4 > 0 { 1 } else { 0 };
    println!("Купе: {answer}");
}
