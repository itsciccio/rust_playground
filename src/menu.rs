use crate::contact_book::ContactBook;
use crate::utils;

pub fn show_menu(mut contact_book: ContactBook){
    loop {

        println!("\n-- MENU --");
        println!("Enter choice:");

        let choice:i8;
        match utils::read_menu_input_to_int() {
            Ok(choice_result) => {
                choice = choice_result;
            },
            Err(_) => {
                choice = -1;
            }
        }

        match choice {
            1 => {
                match contact_book.create_person(){
                    Ok(person) => person,
                    Err(_) => (),
                };
            },
            2 => {
                contact_book.view_all_person_list();
            },
            3 => {
                match contact_book.view_person_by_id(){
                    Ok(person) => person,
                    Err(_) => (),
                }
            },
            4 => {
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
