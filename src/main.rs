use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Угадай число!");

    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    loop {
        println!("Введите свою догатку.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Неудалось прочитать строку!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Вы загадали: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком малое число!"),
            Ordering::Greater => println!("Слишком большое число!"),
            Ordering::Equal => {
                println!("Вы выиграли!");
                break;
            },
        }
    }
}