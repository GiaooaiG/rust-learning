fn main() {
    let new_apple = Apple{
        size : 2,
    };
    println!("{}",new_apple.eat());
    println!("{}",new_apple.size);
}
trait Eatable {
        fn eat(&self) -> &str ;
}

struct Apple {
    pub size :u32,
}
impl Eatable for Apple {
    fn eat(&self) -> &str {
        "Eat an Apple"
    }
}