fn main() {
    println!("输入一个数字:");
    let mut input: String = String::from("");
    std::io::stdin().read_line(&mut input).expect("无法读取");
    let mut input: u64 = input.trim().parse().expect("无法转换为u64");
    while input > 1 {
        for i in 2..input + 1 {
            if input % i == 0 {
                print!("{} ", i);
                input /= i;
                break;
            }
        }
    }
    println!("");
}
