struct Rectangle{
    width:u32,
    length:u32,
}
impl Rectangle {
    fn area(&self) -> u32{
        self.width*self.length
    }
    fn num() -> u32{
        1
    }
}
fn main(){
    let _a = Some(3);
    let rect = Rectangle{
        length:30,
        width:50,
    };
    println!("{}",rect.area());
    for i in  (1..4).rev(){
        println!("{}",i);
    }
    println!("{}",Rectangle::num());
    let is_ok = Ok::Yes;
    match is_ok {
        Ok::Yes => println!("OK"),
        _ => println!("No"),
    };
}
enum Ok{
    Yes,
    No,
}
