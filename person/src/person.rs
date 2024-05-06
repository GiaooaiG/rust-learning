pub struct Person {
    pub name: String,
    pub age: u32,
    phone_number: String,
}
impl Person {
    pub fn get_phone_number(&self) -> &String {
        return &self.phone_number;
    }
    pub fn set_phone_number(&mut self, new_number: String) {
        self.phone_number = new_number;
    }
}
pub mod student;
