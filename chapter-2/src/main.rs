use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数");
    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("神秘数字是：{}", secret_number);
    loop {
        println!("猜一个数!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是：{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("你赢了!");
                break;
            }
        }
    }
}
