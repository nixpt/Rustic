use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
      
    println!("[*] lets play a number guessing game.\n");

     // Genarating secret number.
    let secn = rand::thread_rng()
    .gen_range(1..=100);

    // looping through until match.
   loop{

    //user input
    let mut guess = String::new();

    println!("[+] Input your guess:");  

    // Reading user number.
    io::stdin()
    .read_line(&mut guess)
    .expect("[-] Could'nt read a number");

    println!("You guessed: {guess}");

    // Parsing user input
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    //Compairing number
    match guess.cmp(&secn){
        Ordering::Less => println!("Give me something bigger than that\n lets try again"),
        Ordering::Greater => print!("Give me something smaller than that\n lets try again"),
        Ordering::Equal => {
            print!("Ah! ha, you chose the correct number\n You Win!!");
            break;
        }
    }


   }
    


}
