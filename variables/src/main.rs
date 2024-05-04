struct Rectangle{
    width:u32,
    length:u32,
}
impl Rectangle {
    fn area(&self) -> u32{
        self.width*self.length
    }
}
fn main(){
    let rect = Rectangle{
        length:30,
        width:50,
    };
    println!("{}",rect.area());
    for i in  (1..4).rev(){
        println!("{}",i);
    }
}
enum _Ok{
    Yes,
    No,
}