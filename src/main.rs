mod menu;
mod person;
mod contact_book;
mod utils;

fn main(){
    let contactbook = contact_book::ContactBook::new();
    menu::show_menu(contactbook);
}