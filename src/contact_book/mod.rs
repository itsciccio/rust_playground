use std::io::stdin;
use uuid::Uuid;

use crate::person::Person;
use crate::utils;

pub struct ContactBook {
    person_list: Vec<Person>,
}

impl ContactBook {
    pub fn new() -> ContactBook {
        let person_list: Vec<Person> = Vec::new();
        ContactBook{person_list}
    }

    pub fn create_person(&mut self) -> Result<(), std::num::ParseIntError> {

        println!("Enter name:");
        let mut input_name = String::new();
        stdin().read_line(&mut input_name).expect("ok");       
        let name = input_name.trim();
    
        println!("Enter phone #:");
        let mut input_number = String::new();
        stdin().read_line(&mut input_number).expect("ok");       
        let phone = match utils::string_to_phone(String::from(input_number.trim())){
            Ok(number) => number,
            Err(e) => {
                println!("Invalid phone number entered.");
                println!("Aborting...");
                return Err(e)
            },
        };
        
        let person = Person::new(String::from(name), phone);
    
        println!("Created person!");
        // println!("Name:  {}", person.get_name());
        // println!("Phone: {}", person.get_phone());

        self.person_list.push(person);
        
        Ok(())
    }

    pub fn view_all_person_list(&self) {
        for person in &self.person_list {
            println!("ID:    {}", person.get_id());
            println!("Name:  {}", person.get_name());
            println!("Phone: {}", person.get_phone());
        }
    }

    pub fn find_person_by_id(&self) -> Result<&Person, &str> {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("ok");    

        //retry all of this... turns out the string had a newline \n at the end... haq alla

        // let input_parse: &str = &input;

        let uuid_to_search = match Uuid::parse_str(input.trim()){
            Ok(uuid) => uuid,
            Err(_) => {
                // println!("Invalid UUID.");
                return Err("Invalid UUID.")
            }
        };
        
        for person in &self.person_list {
            if person.get_id() == uuid_to_search {
                return Ok(person);            
            }
        }
        return Err("Person not found.")
    }

    pub fn view_person_by_id(&self) -> Result<(), uuid::Error>  {
        let person = match self.find_person_by_id(){
            Ok(person) => person,
            Err(e) => {
                println!("{}", e);
                return Ok(())
            }
        };

        println!("Name:  {}", person.get_name());
        println!("Phone: {}", person.get_phone()); 

        Ok(())
    }
}

