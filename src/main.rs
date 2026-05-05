use std::io; 

fn main(){
    println!("Temperature converter");
    println!("1: Celcius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");
    println!("Select an option (1 o 2): ");

    let mut choice = String::new(); //string EMPTY variable that is mutable 
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: u32 = match choice.trim().parse(){
        Ok(num) => num, 
        Err(_) => {
            println!("Invalid choise. Try again adn enter 1 or 2"); 
            return;
        }
    };


}

