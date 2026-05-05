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

    if choice == 1 {
        celsius_to_fahrenheit();
    } else if choice == 2 {
        fahrenheit_to_celsius();
    } else {
        println!("Invalid choise. Select 1 or 2");
    }

}

fn celsius_to_fahrenheit(){
    println!("Enter temperature in Celcius");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse(){
        Ok(num) => num, 
        Err(_) => {
            print!("Invlid input. Enter valid number");
            return;
        }
    }; 

    let fahrenheit =  (temp * 9.0/5.0) + 32.0; 
    print!("{:.2} C is {:.2} F", temp, fahrenheit); 

}

fn fahrenheit_to_celsius(){
    print!("Enter temperature in Fahrenheit: ");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input"); 

    let temp: f64 = match temp.trim().parse(){
        Ok(num) => num, 
        Err(_) => {
            println!("Invliad input. Enter valid number"); 
            return; 
        }
    }; 

    let celcius = (temp - 32.0) * 5.0/9.0;
    print!("{:.2} F is {:.2} C", temp, celcius); 

}

