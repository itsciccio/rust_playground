use uuid::Uuid;

pub struct Person {
    id: Uuid,
    name: String,
    phone: u128,
}

impl Person {
    pub fn new(name: String, phone: u128) -> Person {
        let new_uuid = Uuid::new_v4();    
        Person{id:new_uuid, name, phone}
    }

    pub fn view_details(&self, show_id:bool) {
        if show_id {
            println!("ID:    {}", self.get_id());
        }        
        println!("Name:  {}", self.get_name());
        println!("Phone: {}", self.get_phone());
    }

    pub fn get_id(&self)-> Uuid{
        self.id
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_phone(&self) -> u128{
        self.phone
    }
}