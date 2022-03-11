use std::io::stdin;
use crate::person::Person;

fn string_to_phone(number:String) -> Result<u128, std::num::ParseIntError> {
    let number = number.trim().parse::<u128>()?;
    Ok(number)
}

pub fn create_person() -> Result<(), std::num::ParseIntError> {

    println!("Enter name:");
    let mut input_name = String::new();
    stdin().read_line(&mut input_name).expect("ok");       
    let name = input_name.trim();

    println!("Enter phone #:");
    let mut input_number = String::new();
    stdin().read_line(&mut input_number).expect("ok");       
    let phone = match string_to_phone(String::from(input_number.trim())){
        Ok(number) => number,
        Err(e) => {
            println!("Invalid phone number entered.");
            println!("Aboring person creation...");
            return Err(e)
        },
    };
    
    let person = Person {
        name: String::from(name),
        phone: phone,
    };

    println!("Created person!");
    println!("Name:  {}", person.name);
    println!("Phone: {}", person.phone);
    
    Ok(())
}