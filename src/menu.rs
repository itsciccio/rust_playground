use std::io::stdin;
use crate::person::person_options;

fn read_menu_input_to_int() -> Result<i8, std::num::ParseIntError> {

    let mut input = String::new();
    stdin().read_line(&mut input).expect("ok");       

    let choice = input.trim().parse::<i8>()?;

    Ok(choice)
}

pub fn show_menu(){
    loop {

        println!("\n-- MENU --");
        println!("Enter choice:");

        let choice:i8;
        match read_menu_input_to_int() {
            Ok(choice_result) => {
                choice = choice_result;
            },
            Err(_) => {
                choice = -1;
            }
        }

        match choice {
            1 => {
                match person_options::create_person(){
                    Ok(person) => person,
                    Err(_) => (),
                };
            },
            2 => {
                println!("Choice 2");
            },
            3 => {
                println!("Goodbye");
                break;
            },
            -1 => {
                println!("Invalid input")
            }
            _ => {
                println!("Choice does not exist")
            }
        }
    }
}
