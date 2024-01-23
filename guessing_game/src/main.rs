use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("업/다운");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");
    let mut count = 0;

    loop {
        println!("숫자를 입력하세요! (1~100) {count}회차");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        // println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("업"),
            Ordering::Greater => println!("다운"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
        count = count + 1;
    }
}
