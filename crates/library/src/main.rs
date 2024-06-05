use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}
pub struct 
pub struct Reader{
    name: String,
    tel_number: u64,
    is_vip: bool,
}
impl Reader {
    pub fn new(name:String,tel_number:u64,is_vip:bool) -> Reader{
        Reader{name,tel_number,is_vip}
    }
}
fn run(){
    let readers:HashMap<u64,Reader> = HashMap::new();
}
fn add_reader(reader:Reader,hashmap: &mut HashMap<u64,Reader>){
}