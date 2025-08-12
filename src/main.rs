use std::{cmp::Ordering, io};
//std io library used for input and output
use rand::Rng;

fn main() {
    // let correct = rand::rng().random_range(1..=10);
    let mut how_many = String::new();
    println!("How many numbers do we want");

    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading inpput");

    //u8 : gives 255 unsigned no
    let num_guesses: u8 = how_many.trim().parse().expect("Error cant get the input");

    let mut correct = Vec::new();

    // if i is not used u can use this _ for unused value
    for _ in 0..num_guesses {
        correct.push(rand::rng().random_range(1..=10));
    }

    println!("{correct:?}");

    // println!("correct : {correct}");
    //.= means include 10

    //loop can be run until we break out
    /*
       println!("Hey guess a number from 1 to 10");
       loop{
           //why we use num inside because
           //buffer append it and do not replace it
           let mut num = String::new();
           io::stdin()
               .read_line(&mut num)
               .expect("Error reading input");

           let num : u32 = match num.trim().parse() {
               Ok(num)=> num,
               Err(e) => {
                   println!("Error parsing input: {e}");
                   continue;
               }
           };




           //expect have two enum : ok and err
           //we use expect when we dont care if it runs or not
           //but if something goes wrong it will panic





           //we can create same name var in same scope
           //this one will overshadow the new no
           //u32 is the data type of new num

           //why we call if expression instead of is statement
           //is that if can returns something and can be bind to someone
           let  message =
           //passing &correct reference
           match num.cmp(&correct){
               Ordering::Greater => "You guessed higher no.",
               Ordering::Less => "no is low than actual no",
               Ordering::Equal => {
                   print!("no is equal");
                   break;
         }
       };
       println!("{message}");
    }
    */

    //while loop
    let mut guesses_made = 0;
    let mut correct_ans_count = 0;
    println!("Hey guess a number from 1 to 10");
    while guesses_made < num_guesses {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(e) => {
                println!("Error parsing input: {e}");
                continue;
            }
        };
        
        match guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater => "You guessed higher no.",
            Ordering::Less => "no is low than actual no",
            Ordering::Equal => {
                correct_ans_count += 1;
                if guesses_made < num_guesses - 1 {
                    println!("Congrats , Lets try to guess next no");
                }
                "No is Equal"
            }
             
        };
         guesses_made += 1;
    }

    println!("Thanks for playing correct answers are:");
    for item in correct {
        print!("{item}, ");
        
    }
    println!("Correct count is : {correct_ans_count}");
}
