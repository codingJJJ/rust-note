use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let scret = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("读取失败");
        let guess: i32 = match guess.trim().parse() {
            Ok(i) => i,
            Err(_) => panic!("请输入一个整数"),
        };
        match guess.cmp(&scret) {
            Ordering::Equal => {
                println!("you win!");
                break;
            }
            Ordering::Less => println!("to less"),
            Ordering::Greater => println!("too big"),
        };
    }
}
