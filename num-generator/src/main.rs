use std::io;
use std::random;



fn main() {

    let NUMBER : i32 = random::random::<i32>();

    '_outer_loop: loop {
        

        println!("Enter a number, only the first valid number will be taken: ");
        let mut _input : String = String::new();
        let _ = io::stdin().read_line(&mut _input);

        if _input == "exit" {
            break '_outer_loop;
        }

        let mut split_input = _input.trim().split(" ");

        let first_number = split_input.next().unwrap();
        
        if first_number.parse::<i32>().is_err(){
            println!("Please enter a valid number");
            continue;
        }

        let guess : i32 = first_number.parse::<i32>().unwrap();

        if guess == NUMBER {
            println!("You guessed the number!");
            break '_outer_loop;
        }
        else if guess < NUMBER {
            println!("Too small");
            continue;
        }
        else if guess > NUMBER {
            println!("Too big");
            continue;
        }

        println!("You entered: {}", first_number.parse::<i32>().unwrap());


    }

}
