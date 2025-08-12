use std::{cmp::Ordering, io};
//std io library used for input and output
use rand::Rng;


fn main() {
    let correct = rand::rng().random_range(1..=10);
    // println!("correct : {correct}");
    //.= means include 10
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
}
