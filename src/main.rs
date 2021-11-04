use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("请输出一个数字");

    let secret = rand::thread_rng().gen_range(0, 10);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");
        let guess: i32 = guess.trim().parse().expect("请输入一个整数");
        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("u win");
                break;
            }
            Ordering::Greater => println!("big"),
            _ => println!("less"),
        };
    }
    println!("{}", secret);
}
