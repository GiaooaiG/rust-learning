use std::collections::HashMap;

fn main() {
    run();
}
pub struct Reader {
    name: String,
    tel_number: u64,
    is_vip: bool,
}
enum Choice {
    Exit,
    Register,
}
impl Reader {
    pub fn new(name: String, tel_number: u64, is_vip: bool) -> Reader {
        Reader {
            name,
            tel_number,
            is_vip,
        }
    }
}
fn run() {
    let mut readers: HashMap<u64, Reader> = HashMap::new();
    loop {
        match get_choice() {
            None => println!("非法输入!"),
            Some(choice) => match choice {
                Choice::Exit => break,
                Choice::Register => register(&mut readers),
            },
        };
    }
}
fn add_reader(reader: Reader, hashmap: &mut HashMap<u64, Reader>) {
    hashmap.insert(reader.tel_number, reader);
}
use std::io;
fn get_choice() -> Option<Choice> {
    println!("选择你的操作:");
    println!("0: 退出");
    println!("1: 读者注册");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error Reading Line");
    match input.trim() {
        "0" => Some(Choice::Exit),
        "1" => Some(Choice::Register),
        _ => None,
    }
}
fn register(hashmap: &mut HashMap<u64, Reader>) {
    let name: String;
    let tel_number: u64;
    let is_vip: bool;
    let mut input = String::new();
    println!("输入姓名:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Error Reading Line");
    name = input.trim().parse().unwrap();
    println!("输入电话:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Error Reading Line");
    tel_number = input.trim().parse().unwrap();
    println!("注册会员吗[y/n](n):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Error Reading Line");
    is_vip = match input.as_str() {
        "y" => true,
        "n" => false,
        _ => false,
    };
    let reader = Reader {
        name,
        tel_number,
        is_vip,
    };
    add_reader(reader, hashmap)
}
