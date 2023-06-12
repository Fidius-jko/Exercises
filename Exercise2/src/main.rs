use std::io;

fn get_int() -> u64 {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        panic!("Console input error: {e}");
    }

    let input = match input[..input.len() - 2].parse::<u64>() {
        Ok(t) => t,
        Err(e) => {
            println!("{e}");
            get_int()
        }
    };
    input
}

fn main() {
    println!("Введите число людей: ");
    let number_of_people = get_int();
    println!(
        "Число выживших: {}",
        number_of_people / 2 + number_of_people % 2
    );
}
