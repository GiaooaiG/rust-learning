use std::io;  // prelude
use rand::Rng;  // trait
use std::cmp::Ordering;

fn main(){
    let number :u32 = rand::thread_rng().gen_range(1..101);
    loop{
        println!("输入一个数：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        // shadow
        let guess :u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你输入的数是：{guess}");
        match guess.cmp(&number){
            Ordering::Greater => println!("太大了！"),
            Ordering::Less => println!("太小了！"),
            Ordering::Equal => {
                println!("你赢了！");
                break;
            },
        }
    }
}