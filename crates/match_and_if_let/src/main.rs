#[allow(dead_code)]
enum Coin {
    A(i32),
    B,
    C,
}
#[allow(dead_code)]
impl Coin {
    fn print(&self) -> &str {
        match self {
            Coin::A(_) => "A",
            _ => "",
        }
    }
}
#[allow(unused_variables)]
fn main() {
    let coin = crate::Coin::A(11);
    mod1::function()
}
mod mod1 {
    pub fn function() {
        println!("你好")
    }
}
