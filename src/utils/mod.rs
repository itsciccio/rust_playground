use std::io::stdin;

pub fn read_menu_input_to_int() -> Result<i8, std::num::ParseIntError> {

    let mut input = String::new();
    stdin().read_line(&mut input).expect("ok");       

    let choice = input.trim().parse::<i8>()?;

    Ok(choice)
}

pub fn string_to_phone(number:String) -> Result<u128, std::num::ParseIntError> {
    let number = number.trim().parse::<u128>()?;
    Ok(number)
}