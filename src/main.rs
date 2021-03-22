use std::io;
use rand::Rng;
fn main() {
    println!("Hello, world!");
    let mut x = 1;
    println!("the value of x is : {}", x);
    x = 2;
    println!("then x equals to {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    println!("input your guess number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess_num = String::new();
    io::stdin().read_line(& mut guess_num)
        .expect("fail to read the number");
    println!("you guess {}", guess_num);

}
