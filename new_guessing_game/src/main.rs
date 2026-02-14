use std::io;
use rand::Rng;

// fn main(){
//     println!("Guess the game");
//     println!("Please input your guess");


//     let mut guess  = String::new(); //  mutable variable (in rust variables are immutable by default , so we added mut to mutate it)

//     io::stdin().read_line(&mut guess).expect("Failed to read the line");

//     println!("You guessed : {guess}");
// }

// Practice more about variables and mutations


// fn main(){
//     let x =2;
//     let y = 3;

//     println!("Numbers are {x} and {y} and sum is {}" , x+y);
// }

// Guess the number 2.0

fn main(){
    println!("Guess the number!");

    let secret_number =  rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("you guessed {guess}");
}