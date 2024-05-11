enum Coin {
    A(i32),
    B,
    C,
}

impl Coin {
    fn print(&self) -> &str {
        match self {
            Coin::A(_) => "A",
            _ => "",
        }
    }
}

fn main() {
    let coin = crate::Coin::A(11);
    mod1::function()
}
mod mod1 {
    pub fn function() {
        println!("你好")
    }
}
